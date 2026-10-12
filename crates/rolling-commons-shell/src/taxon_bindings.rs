//! Canonical taxon binding for ecology render keys, authored in VibeScript.
//!
//! The fixture `web/fixtures/taxon-bindings.vibe` is the single source of
//! truth: GBIF backbone keys are the engine-interop identity (gbif_adapter,
//! bioacoustics taxon_id), name strings are display labels only. Unknown
//! render keys return `bound: 0` — they stay data-only, same rule as the
//! render selections.
use std::cell::RefCell;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

/// Structured binding extracted from the VibeScript record.
#[derive(Debug, Clone, PartialEq)]
pub struct TaxonBinding {
    pub gbif_key: u64,
    pub iri: String,
    pub rank: String,
    pub scientific_name: String,
    pub common_name: String,
    pub asset_family: String,
    pub rank_note: String,
    pub accrual_note: String,
}

thread_local! {static TAXA:RefCell<Option<vibe::Program>>=const {RefCell::new(None)};}

fn text<'a>(record: &'a BTreeMap<String, vibe::Value>, field: &str) -> Result<&'a str, JsValue> {
    let Some(vibe::Value::String(value)) = record.get(field) else {
        return Err(JsValue::from_str("taxon record field missing"));
    };
    if value.len() > 512 {
        return Err(JsValue::from_str("taxon record field exceeds bounds"));
    }
    Ok(value.as_str())
}

fn binding_for(render_key: &str) -> Result<Option<TaxonBinding>, JsValue> {
    TAXA.with(|cached| {
        let mut cached = cached.borrow_mut();
        if cached.is_none() {
            *cached = Some(
                vibe::load_program(include_str!("../../../web/fixtures/taxon-bindings.vibe"))
                    .map_err(|_| JsValue::from_str("taxon VibeScript bindings could not load"))?,
            );
        }
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        let value = vibe::eval_function(
            cached.as_ref().unwrap(),
            "taxon_for",
            vec![vibe::Value::String(render_key.to_owned())],
            &mut host,
            &mut env,
        )
        .map_err(|_| JsValue::from_str("taxon VibeScript bindings failed"))?;
        let vibe::Value::Record(record) = value else {
            return Err(JsValue::from_str("taxon policy did not return a record"));
        };
        let Some(vibe::Value::F64(bound)) = record.get("bound") else {
            return Err(JsValue::from_str("taxon bound flag missing"));
        };
        if *bound == 0.0 {
            return Ok(None);
        }
        let Some(vibe::Value::F64(gbif_key)) = record.get("gbifKey") else {
            return Err(JsValue::from_str("taxon gbifKey missing"));
        };
        if !gbif_key.is_finite() || *gbif_key <= 0.0 || *gbif_key > 9_007_199_254_740_991.0
            || gbif_key.fract() != 0.0
        {
            return Err(JsValue::from_str("taxon gbifKey exceeds bounds"));
        }
        let binding = TaxonBinding {
            gbif_key: *gbif_key as u64,
            iri: text(&record, "iri")?.to_owned(),
            rank: text(&record, "rank")?.to_owned(),
            scientific_name: text(&record, "scientificName")?.to_owned(),
            common_name: text(&record, "commonName")?.to_owned(),
            asset_family: text(&record, "assetFamily")?.to_owned(),
            rank_note: text(&record, "rankNote")?.to_owned(),
            accrual_note: text(&record, "accrualNote")?.to_owned(),
        };
        if binding.iri.is_empty() || !binding.iri.starts_with("https://www.gbif.org/species/") {
            return Err(JsValue::from_str("taxon iri exceeds bounds"));
        }
        Ok(Some(binding))
    })
}

/// Resolve a render key to its canonical taxon record as JSON; `null` when
/// the key is unbound. Used by the runtime and by export tooling so the
/// .vibe fixture is the only authored copy of the mapping.
#[wasm_bindgen]
pub fn taxon_binding_json(render_key: &str) -> Result<String, JsValue> {
    if render_key.trim().is_empty()
        || render_key.len() > 128
        || !render_key
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(JsValue::from_str("taxon render key is outside bounds"));
    }
    let Some(binding) = binding_for(render_key)? else {
        return Ok("null".to_owned());
    };
    serde_json::to_string(&serde_json::json!({
        "gbifKey": binding.gbif_key,
        "iri": binding.iri,
        "rank": binding.rank,
        "scientificName": binding.scientific_name,
        "commonName": binding.common_name,
        "assetFamily": binding.asset_family,
        "rankNote": binding.rank_note,
        "accrualNote": binding.accrual_note,
    }))
    .map_err(|_| JsValue::from_str("taxon binding serialisation failed"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_resolve_and_unbound_returns_none() {
        let kangaroo = binding_for("kangaroo").unwrap().unwrap();
        assert_eq!(kangaroo.gbif_key, 9372);
        assert_eq!(kangaroo.rank, "FAMILY");
        assert_eq!(kangaroo.iri, "https://www.gbif.org/species/9372");

        let gum = binding_for("eucalypt-gum").unwrap().unwrap();
        assert_eq!(gum.gbif_key, 7493935);
        assert_eq!(gum.asset_family, "rc:asset/flora/eucalypt-gum");
        assert!(!gum.accrual_note.is_empty());

        let snake = binding_for("snake").unwrap().unwrap();
        assert_eq!(snake.gbif_key, 11205692);
        assert!(!snake.rank_note.is_empty());

        assert_eq!(binding_for("not-a-taxon").unwrap(), None);
        assert_eq!(binding_for("").unwrap(), None);
    }
}
