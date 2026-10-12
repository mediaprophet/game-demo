//! Canonical taxon binding for ecology render keys, authored in VibeScript.
//!
//! The fixture `web/fixtures/taxon-bindings.vibe` is the single source of
//! truth: GBIF backbone keys are the engine-interop identity (gbif_adapter,
//! bioacoustics taxon_id), name strings are display labels only. Unknown
//! render keys return `bound: 0` — they stay data-only, same rule as the
//! render selections.
use qualia_core_db::lexicon::generate_60bit_token;
use qualia_core_db::resolver::INLINE_TAG_INTEGER;
use qualia_core_db::NQuin;
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

fn check_render_key(render_key: &str) -> Result<(), JsValue> {
    if render_key.trim().is_empty()
        || render_key.len() > 128
        || !render_key
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(JsValue::from_str("taxon render key is outside bounds"));
    }
    Ok(())
}

/// Resolve a render key to its canonical taxon record as JSON; `null` when
/// the key is unbound. Used by the runtime and by export tooling so the
/// .vibe fixture is the only authored copy of the mapping.
#[wasm_bindgen]
pub fn taxon_binding_json(render_key: &str) -> Result<String, JsValue> {
    check_render_key(render_key)?;
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

/// Maximum quins per bound taxon: five vibe-authored dwc statements, the
/// foaf:depicts edge, and the host-packed gbifTaxonKey integer quin.
pub const MAX_DWC_QUINS: usize = 7;

/// Evaluate `dwc_statements_for` in the VibeScript fixture and copy the
/// sealed quins into the caller-provided buffer, then append the
/// `urn:qualia:gbif:taxonKey` quin with `INLINE_TAG_INTEGER` packing — the
/// tag sits above 2^53 so scripts cannot form it losslessly; this is the
/// same u64 the engine uses for `taxon_id` in gbif_adapter/bioacoustics.
///
/// `asset_iri` is the foaf:depicts subject; pass "" to emit taxon-node
/// statements only. Returns the quin count written.
pub fn dwc_quins(render_key: &str, asset_iri: &str, out: &mut [NQuin]) -> Result<usize, JsValue> {
    check_render_key(render_key)?;
    if asset_iri.len() > 512
        || asset_iri
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte == b' ')
    {
        return Err(JsValue::from_str("dwc subject iri is outside bounds"));
    }
    let Some(binding) = binding_for(render_key)? else {
        return Ok(0);
    };
    TAXA.with(|cached| {
        let cached = cached.borrow();
        let program = match cached.as_ref() {
            Some(program) => program,
            None => return Err(JsValue::from_str("taxon VibeScript bindings could not load")),
        };
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        let value = vibe::eval_function(
            program,
            "dwc_statements_for",
            vec![
                vibe::Value::String(render_key.to_owned()),
                vibe::Value::String(asset_iri.to_owned()),
            ],
            &mut host,
            &mut env,
        )
        .map_err(|_| JsValue::from_str("dwc VibeScript emission failed"))?;
        let vibe::Value::List(items) = value else {
            return Err(JsValue::from_str("dwc emission did not return a list"));
        };
        if items.len() + 1 > out.len() {
            return Err(JsValue::from_str("dwc quin buffer too small"));
        }
        let mut n = 0usize;
        for item in &items {
            let vibe::Value::QuinRef(quin) = item else {
                return Err(JsValue::from_str("dwc emission returned a non-quin value"));
            };
            let f = quin.raw_fields();
            out[n] = NQuin {
                subject: f[0],
                predicate: f[1],
                object: f[2],
                context: f[3],
                metadata: f[4],
                parity: f[5],
            };
            n += 1;
        }
        let subject = generate_60bit_token(binding.iri.as_bytes());
        let predicate = generate_60bit_token(b"urn:qualia:gbif:taxonKey");
        let object = INLINE_TAG_INTEGER | binding.gbif_key;
        out[n] = NQuin {
            subject,
            predicate,
            object,
            context: 0,
            metadata: 0,
            parity: NQuin::calculate_parity(subject, predicate, object, 0, 0),
        };
        Ok(n + 1)
    })
}

/// Emit the Darwin Core quins for a render key as JSON — fields are hex
/// strings so u64 values survive JS's f64 losslessness boundary. Used by
/// pack/export tooling; the .vibe fixture authors the statements.
#[wasm_bindgen]
pub fn taxon_dwc_json(render_key: &str, asset_iri: &str) -> Result<String, JsValue> {
    let mut buf = [NQuin::default(); MAX_DWC_QUINS];
    let n = dwc_quins(render_key, asset_iri, &mut buf)?;
    let quins: Vec<serde_json::Value> = buf[..n]
        .iter()
        .map(|quin| {
            serde_json::json!({
                "subject": format!("0x{:016x}", quin.subject),
                "predicate": format!("0x{:016x}", quin.predicate),
                "object": format!("0x{:016x}", quin.object),
                "context": format!("0x{:016x}", quin.context),
                "metadata": format!("0x{:016x}", quin.metadata),
                "parity": format!("0x{:016x}", quin.parity),
            })
        })
        .collect();
    serde_json::to_string(&quins)
        .map_err(|_| JsValue::from_str("dwc quin serialisation failed"))
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

    #[test]
    fn dwc_emission_seals_taxon_node_and_depicts() {
        let mut buf = [NQuin::default(); MAX_DWC_QUINS];
        let n = dwc_quins("kangaroo", "rc:asset/fauna/inland-kangaroo-01", &mut buf).unwrap();
        assert_eq!(n, MAX_DWC_QUINS);
        assert!(buf[..n].iter().all(|quin| quin.verify_ecc_parity()));
        // Host-packed integer quin is last: INLINE_TAG_INTEGER | gbifKey.
        assert_eq!(buf[n - 1].object, INLINE_TAG_INTEGER | 9372);
        // Without an asset subject the depicts edge is omitted.
        let n = dwc_quins("kangaroo", "", &mut buf).unwrap();
        assert_eq!(n, MAX_DWC_QUINS - 1);
        // Unbound keys emit nothing.
        assert_eq!(dwc_quins("not-a-taxon", "rc:asset/fauna/x", &mut buf).unwrap(), 0);
    }
}
