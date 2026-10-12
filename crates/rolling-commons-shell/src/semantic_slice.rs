//! Pack-resident semantic slices — the browser ontology delivery path (G50).
//!
//! `ontology_loader.rs` is `cfg !wasm32`, so ontology/semantic data reaches
//! the browser only through packs. A slice is the `sem/` entries of a
//! verified HMC pack: `.quins` blobs (raw 48-byte NQuins, little-endian
//! fields in struct order) plus `.lex` lexicons (`<16-hex-hash> <iri>` lines)
//! that make the hashes resolvable. Digests are verified before any bytes
//! are read; parity is re-checked on every quin.
use qualia_core_db::lexicon::generate_60bit_token;
use qualia_core_db::query::shacl_compiler::{
    validate_shacl_property, ShaclConstraint, ShaclDatatype,
};
use qualia_core_db::NQuin;
use wasm_bindgen::prelude::*;

const MAX_SEMANTIC_QUINS: usize = 65_536;
const MAX_LEXICON_LINES: usize = 65_536;
const MAX_LEX_LINE: usize = 768;
const MAX_CONSTRAINTS: usize = 8;

/// A verified, browser-resident semantic slice from an HMC pack.
#[wasm_bindgen]
pub struct SemanticSlice {
    quins: Vec<NQuin>,
    lexicon: Vec<(u64, String)>,
}

fn quin_from_bytes(bytes: &[u8]) -> NQuin {
    let read = |i: usize| {
        u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap_or_default())
    };
    NQuin {
        subject: read(0),
        predicate: read(1),
        object: read(2),
        context: read(3),
        metadata: read(4),
        parity: read(5),
    }
}

#[wasm_bindgen]
impl SemanticSlice {
    /// Parse a verified HMC pack and collect its `sem/` entries. Every pack
    /// entry digest is verified first; quin parity is re-checked on load.
    #[wasm_bindgen(constructor)]
    pub fn from_pack(pack: &[u8]) -> Result<SemanticSlice, JsValue> {
        use qualia_core_db::bundle::BundleReader;
        let reader =
            BundleReader::parse(pack).map_err(|e| JsValue::from_str(&e.to_string()))?;
        if reader
            .entries()
            .iter()
            .any(|entry| !reader.verify_entry(&entry.key))
        {
            return Err(JsValue::from_str("pack entry digest verification failed"));
        }
        let mut quins: Vec<NQuin> = Vec::new();
        let mut lexicon: Vec<(u64, String)> = Vec::new();
        for entry in reader.entries() {
            let Some(name) = entry.key.strip_prefix("sem/") else {
                continue;
            };
            let Some(bytes) = reader.get(&entry.key) else {
                return Err(JsValue::from_str("semantic entry missing from pack"));
            };
            if name.ends_with(".quins") {
                if bytes.len() % 48 != 0 {
                    return Err(JsValue::from_str("semantic quin blob is not 48-byte aligned"));
                }
                for chunk in bytes.chunks_exact(48) {
                    if quins.len() >= MAX_SEMANTIC_QUINS {
                        return Err(JsValue::from_str("semantic slice exceeds quin bound"));
                    }
                    let quin = quin_from_bytes(chunk);
                    if !quin.verify_ecc_parity() {
                        return Err(JsValue::from_str("semantic quin parity check failed"));
                    }
                    quins.push(quin);
                }
            } else if name.ends_with(".lex") {
                let text = std::str::from_utf8(bytes)
                    .map_err(|_| JsValue::from_str("lexicon is not utf-8"))?;
                for line in text.lines() {
                    if line.is_empty() {
                        continue;
                    }
                    if lexicon.len() >= MAX_LEXICON_LINES || line.len() > MAX_LEX_LINE {
                        return Err(JsValue::from_str("lexicon exceeds bounds"));
                    }
                    let (hash_hex, iri) = line
                        .split_once(' ')
                        .ok_or_else(|| JsValue::from_str("lexicon line is not '<hash> <iri>'"))?;
                    let hash = u64::from_str_radix(hash_hex.trim_start_matches("0x"), 16)
                        .map_err(|_| JsValue::from_str("lexicon hash is not hex"))?;
                    lexicon.push((hash, iri.to_owned()));
                }
            }
        }
        Ok(SemanticSlice { quins, lexicon })
    }

    /// Number of verified quins in the slice.
    #[wasm_bindgen(getter)]
    pub fn count(&self) -> u32 {
        self.quins.len() as u32
    }

    /// Resolve a hash (hex string, `0x` optional) to its lexicon term.
    pub fn resolve(&self, hash_hex: &str) -> Option<String> {
        let hash = u64::from_str_radix(hash_hex.trim_start_matches("0x"), 16).ok()?;
        self.lexicon
            .iter()
            .find(|(h, _)| *h == hash)
            .map(|(_, iri)| iri.clone())
    }

    /// Emit the slice as `{quin hex fields, s, p, o}` JSON — `s`/`p`/`o` are
    /// the lexicon-resolved terms ("" when the hash is not in the lexicon).
    pub fn quins_json(&self) -> String {
        let resolve = |h: u64| {
            self.lexicon
                .iter()
                .find(|(lh, _)| *lh == h)
                .map(|(_, iri)| iri.as_str())
                .unwrap_or("")
        };
        let rows: Vec<serde_json::Value> = self
            .quins
            .iter()
            .map(|quin| {
                serde_json::json!({
                    "s": resolve(quin.subject),
                    "p": resolve(quin.predicate),
                    "o": resolve(quin.object),
                    "quin": {
                        "subject": format!("0x{:016x}", quin.subject),
                        "predicate": format!("0x{:016x}", quin.predicate),
                        "object": format!("0x{:016x}", quin.object),
                        "context": format!("0x{:016x}", quin.context),
                        "metadata": format!("0x{:016x}", quin.metadata),
                        "parity": format!("0x{:016x}", quin.parity),
                    },
                })
            })
            .collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".to_owned())
    }

    /// SHACL-validate a subject's property against JSON constraints —
    /// `[{"kind":"minCount","value":1}, {"kind":"in","values":["<iri>",..]}]`.
    /// Subset of `poet_host` SHACL.validate kinds: minCount, maxCount,
    /// minLength, maxLength, datatype (xsd:* short names), in.
    pub fn validate_shacl(
        &self,
        subject_iri: &str,
        path_iri: &str,
        constraints_json: &str,
    ) -> Result<bool, JsValue> {
        if subject_iri.len() > 512 || path_iri.len() > 512 {
            return Err(JsValue::from_str("shacl iri exceeds bounds"));
        }
        let spec: serde_json::Value = serde_json::from_str(constraints_json)
            .map_err(|_| JsValue::from_str("shacl constraints are not json"))?;
        let Some(list) = spec.as_array() else {
            return Err(JsValue::from_str("shacl constraints must be a list"));
        };
        if list.len() > MAX_CONSTRAINTS {
            return Err(JsValue::from_str("shacl constraints exceed bound"));
        }
        let mut buf = [ShaclConstraint::MinCount(0); MAX_CONSTRAINTS];
        for (i, item) in list.iter().enumerate() {
            let kind = item
                .get("kind")
                .and_then(|k| k.as_str())
                .ok_or_else(|| JsValue::from_str("constraint.kind missing"))?;
            let value = item.get("value").and_then(|v| v.as_u64()).unwrap_or(0);
            buf[i] = match kind {
                "minCount" => ShaclConstraint::MinCount(value.max(1) as u32),
                "maxCount" => ShaclConstraint::MaxCount(value as u32),
                "minLength" => ShaclConstraint::MinLength(value as u32),
                "maxLength" => ShaclConstraint::MaxLength(value as u32),
                "datatype" => {
                    let dt = item
                        .get("value")
                        .and_then(|v| v.as_str())
                        .unwrap_or("xsd:string");
                    let datatype = match dt {
                        "xsd:integer" | "integer" => ShaclDatatype::Integer,
                        "xsd:decimal" | "decimal" => ShaclDatatype::Decimal,
                        "xsd:boolean" | "boolean" => ShaclDatatype::Boolean,
                        "xsd:dateTime" | "dateTime" => ShaclDatatype::DateTime,
                        _ => ShaclDatatype::String,
                    };
                    ShaclConstraint::Datatype(datatype)
                }
                "in" => {
                    let mut values = [0u64; 8];
                    let mut count = 0u8;
                    if let Some(list) = item.get("values").and_then(|v| v.as_array()) {
                        for v in list.iter().take(8) {
                            if let Some(iri) = v.as_str() {
                                values[count as usize] = generate_60bit_token(iri.as_bytes());
                                count += 1;
                            }
                        }
                    }
                    ShaclConstraint::In { count, values }
                }
                _ => return Err(JsValue::from_str("unsupported shacl constraint kind")),
            };
        }
        Ok(validate_shacl_property(
            &self.quins,
            generate_60bit_token(subject_iri.as_bytes()),
            generate_60bit_token(path_iri.as_bytes()),
            &buf[..list.len()],
        ))
    }
}
