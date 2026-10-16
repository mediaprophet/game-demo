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
    /// `qualia:accrual:*` class IRI when the stock is accrual-classified;
    /// empty for unclassified bindings.
    pub maturity_iri: String,
    pub irreversibility_iri: String,
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
        if !gbif_key.is_finite()
            || *gbif_key <= 0.0
            || *gbif_key > 9_007_199_254_740_991.0
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
            maturity_iri: text(&record, "maturityIri")?.to_owned(),
            irreversibility_iri: text(&record, "irreversibilityIri")?.to_owned(),
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
        "maturityIri": binding.maturity_iri,
        "irreversibilityIri": binding.irreversibility_iri,
    }))
    .map_err(|_| JsValue::from_str("taxon binding serialisation failed"))
}

/// Maximum quins per bound taxon: five vibe-authored dwc statements, the
/// foaf:depicts edge, up to two accrual-class quins, and the host-packed
/// gbifTaxonKey integer quin.
pub const MAX_DWC_QUINS: usize = 9;

/// One emitted semantic statement: the sealed quin plus its source terms,
/// so pack compile can write a hash→IRI lexicon without re-deriving hashes.
#[derive(Debug, Clone)]
pub struct DwcStatement {
    pub quin: NQuin,
    pub subject_term: String,
    pub predicate_term: String,
    pub object_term: String,
    /// True when the object is a typed integer, not a hashed IRI/literal.
    pub object_is_integer: bool,
}

/// Evaluate `dwc_statements_for` in the VibeScript fixture into
/// caller-provided buffers, then append the `urn:qualia:gbif:taxonKey` quin
/// with `INLINE_TAG_INTEGER` packing — the tag sits above 2^53 so scripts
/// cannot form it losslessly; this is the same u64 the engine uses for
/// `taxon_id` in gbif_adapter/bioacoustics.
///
/// `asset_iri` is the foaf:depicts/accrual subject; pass "" to emit
/// taxon-node statements only. Returns the statement count written.
pub fn dwc_statements(
    render_key: &str,
    asset_iri: &str,
    out: &mut [DwcStatement],
) -> Result<usize, JsValue> {
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
            None => {
                return Err(JsValue::from_str(
                    "taxon VibeScript bindings could not load",
                ))
            }
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
            let vibe::Value::Record(record) = item else {
                return Err(JsValue::from_str(
                    "dwc emission returned a non-record value",
                ));
            };
            let vibe::Value::QuinRef(quin) = record
                .get("quin")
                .ok_or_else(|| JsValue::from_str("dwc statement missing quin"))?
            else {
                return Err(JsValue::from_str("dwc statement quin is not sealed"));
            };
            let f = quin.raw_fields();
            out[n] = DwcStatement {
                quin: NQuin {
                    subject: f[0],
                    predicate: f[1],
                    object: f[2],
                    context: f[3],
                    metadata: f[4],
                    parity: f[5],
                },
                subject_term: text(record, "s")?.to_owned(),
                predicate_term: text(record, "p")?.to_owned(),
                object_term: text(record, "o")?.to_owned(),
                object_is_integer: false,
            };
            n += 1;
        }
        let subject = generate_60bit_token(binding.iri.as_bytes());
        let predicate = generate_60bit_token(b"urn:qualia:gbif:taxonKey");
        let object = INLINE_TAG_INTEGER | binding.gbif_key;
        out[n] = DwcStatement {
            quin: NQuin {
                subject,
                predicate,
                object,
                context: 0,
                metadata: 0,
                parity: NQuin::calculate_parity(subject, predicate, object, 0, 0),
            },
            subject_term: binding.iri.clone(),
            predicate_term: "urn:qualia:gbif:taxonKey".to_owned(),
            object_term: binding.gbif_key.to_string(),
            object_is_integer: true,
        };
        Ok(n + 1)
    })
}

/// Emit the Darwin Core statements for a render key as JSON — quin fields
/// are hex strings so u64 values survive JS's f64 losslessness boundary,
/// and each entry carries its source terms so exporters can write the
/// pack's hash→IRI lexicon from the same authored emission.
#[wasm_bindgen]
pub fn taxon_dwc_json(render_key: &str, asset_iri: &str) -> Result<String, JsValue> {
    let blank = DwcStatement {
        quin: NQuin::default(),
        subject_term: String::new(),
        predicate_term: String::new(),
        object_term: String::new(),
        object_is_integer: false,
    };
    let mut buf: [DwcStatement; MAX_DWC_QUINS] = std::array::from_fn(|_| blank.clone());
    let n = dwc_statements(render_key, asset_iri, &mut buf)?;
    let quins: Vec<serde_json::Value> = buf[..n]
        .iter()
        .map(|stmt| {
            serde_json::json!({
                "s": stmt.subject_term,
                "p": stmt.predicate_term,
                "o": stmt.object_term,
                "integer": stmt.object_is_integer,
                "quin": {
                    "subject": format!("0x{:016x}", stmt.quin.subject),
                    "predicate": format!("0x{:016x}", stmt.quin.predicate),
                    "object": format!("0x{:016x}", stmt.quin.object),
                    "context": format!("0x{:016x}", stmt.quin.context),
                    "metadata": format!("0x{:016x}", stmt.quin.metadata),
                    "parity": format!("0x{:016x}", stmt.quin.parity),
                },
            })
        })
        .collect();
    serde_json::to_string(&quins).map_err(|_| JsValue::from_str("dwc quin serialisation failed"))
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
        let blank = DwcStatement {
            quin: NQuin::default(),
            subject_term: String::new(),
            predicate_term: String::new(),
            object_term: String::new(),
            object_is_integer: false,
        };
        let mut buf: [DwcStatement; MAX_DWC_QUINS] = std::array::from_fn(|_| blank.clone());
        let n = dwc_statements("kangaroo", "rc:asset/fauna/inland-kangaroo-01", &mut buf).unwrap();
        assert_eq!(n, 7);
        assert!(buf[..n].iter().all(|s| s.quin.verify_ecc_parity()));
        // Host-packed integer quin is last: INLINE_TAG_INTEGER | gbifKey.
        assert_eq!(buf[n - 1].quin.object, INLINE_TAG_INTEGER | 9372);
        assert!(buf[n - 1].object_is_integer);
        // Without an asset subject the depicts edge is omitted.
        let n = dwc_statements("kangaroo", "", &mut buf).unwrap();
        assert_eq!(n, 6);
        // Accrual-classified stocks emit maturity + irreversibility quins on
        // the asset subject (the stock), not the taxon.
        let n = dwc_statements(
            "eucalypt-gum",
            "rc:asset/flora/eucalypt-gum/mature-gums-01",
            &mut buf,
        )
        .unwrap();
        assert_eq!(n, MAX_DWC_QUINS);
        let irrev_pred = generate_60bit_token(b"qualia:accrual:irreversibilityClass");
        assert!(buf[..n].iter().any(|s| s.quin.predicate == irrev_pred));
        // Unbound keys emit nothing.
        assert_eq!(
            dwc_statements("not-a-taxon", "rc:asset/fauna/x", &mut buf).unwrap(),
            0
        );
    }
}
