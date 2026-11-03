//! Opt-in candidate review over canonical QualiaDB containers. No release,
//! source execution, graph adoption or default game asset replacement occurs.
use qualia_core_db::bundle::{BundleReader, BUNDLE_ENTRY_ALIGN, BUNDLE_HEADER_SIZE};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use wasm_bindgen::prelude::*;

const MAX_INPUT: usize = 32 * 1024 * 1024;
const MAX_ASSET: usize = 8 * 1024 * 1024;
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn inspect(bytes: &[u8], bundle: bool, selected: &str) -> Result<(Value, Option<Vec<u8>>), String> {
    if bytes.is_empty()
        || bytes.len() > MAX_INPUT
        || selected.len() > 512
        || selected.as_bytes().contains(&0)
    {
        return Err("Candidate file or selection exceeds review limits".into());
    }
    let mut assets = Vec::new();
    let mut declarations = Vec::new();
    let mut chosen = None;
    let input_sha = sha(bytes);
    if bundle {
        if bytes.len() < BUNDLE_HEADER_SIZE {
            return Err("Candidate HMC header is truncated".into());
        }
        let entries = u32::from_le_bytes(bytes[8..12].try_into().map_err(|_| "HMC count")?);
        let index_bytes = u64::from_le_bytes(bytes[20..28].try_into().map_err(|_| "HMC index")?);
        if entries == 0
            || entries > 256
            || index_bytes > 64 * 1024
            || bytes[40..64].iter().any(|v| *v != 0)
        {
            return Err("Candidate HMC index/header exceeds review limits".into());
        }
        let reader =
            BundleReader::parse(bytes).map_err(|e| format!("Canonical HMC admission: {e}"))?;
        if reader.flags() != 0 || reader.entries().len() != entries as usize {
            return Err("Candidate HMC flags/count mismatch".into());
        }
        let mut keys = BTreeSet::new();
        let mut ranges = Vec::new();
        for entry in reader.entries() {
            if entry.key.is_empty()
                || entry.key.len() > 512
                || entry.key.as_bytes().contains(&0)
                || !keys.insert(entry.key.as_str())
                || entry.offset % BUNDLE_ENTRY_ALIGN as u64 != 0
                || !reader.verify_entry(&entry.key)
            {
                return Err("Candidate HMC key/alignment/hash admission failed".into());
            }
            ranges.push((
                entry.offset,
                entry
                    .offset
                    .checked_add(entry.length)
                    .ok_or("Candidate range overflow")?,
            ));
            let data = reader.get(&entry.key).ok_or("Candidate resource missing")?;
            if entry.kind == "10d" && entry.key.ends_with(".10d") {
                if data.len() > MAX_ASSET {
                    return Err("Candidate .10d exceeds 8 MiB profile".into());
                }
                assets.push(json!({"key":entry.key,"sha256":sha(data),"bytes":data.len()}));
                if selected == entry.key {
                    chosen = Some(data.to_vec());
                }
            }
            if matches!(entry.key.as_str(), "manifest.json" | "review/manifest.json") {
                if data.len() > 64 * 1024 {
                    return Err("Candidate declaration exceeds 64 KiB".into());
                }
                let value: Value = serde_json::from_slice(data)
                    .map_err(|e| format!("Candidate declaration JSON: {e}"))?;
                declarations.push(
                    json!({"resource":entry.key,"sha256":sha(data),"declaration_only":value}),
                );
            }
        }
        ranges.sort_unstable();
        if ranges.windows(2).any(|r| r[0].1 > r[1].0) {
            return Err("Candidate HMC resources overlap".into());
        }
    } else {
        if bytes.len() > MAX_ASSET {
            return Err("Standalone .10d exceeds 8 MiB profile".into());
        }
        assets.push(json!({"key":"standalone.10d","sha256":input_sha,"bytes":bytes.len()}));
        if selected == "standalone.10d" {
            chosen = Some(bytes.to_vec());
        }
    }
    if assets.is_empty() {
        return Err("Candidate contains no explicit .10d mesh resource".into());
    }
    if !selected.is_empty() {
        let data = chosen
            .as_ref()
            .ok_or("Select an enumerated .10d resource explicitly")?;
        let materials = super::pack_validation::verify_10d_asset(data)?;
        if materials.iter().any(|m| {
            [
                m.base_color_texture,
                m.normal_texture,
                m.metallic_roughness_texture,
                m.occlusion_texture,
                m.emissive_texture,
                m.stylized_ramp_texture,
            ]
            .iter()
            .any(|d| *d != [0; 32])
        }) {
            return Err("Textured candidate requires the full game-scene texture residency route; this bounded importer refuses incomplete preview".into());
        }
    }
    Ok((
        json!({"schema":"game-demo.candidate-import/1","input_sha256":input_sha,
        "input_bytes":bytes.len(),"assets":assets,"selected_key":selected,
        "declarations":declarations,"scope":"Opt-in candidate mesh inspection; source/master declarations unaccepted; no game-release or species acceptance"}),
        chosen,
    ))
}

/// Enumerate with an empty key; select an explicit returned key to validate and
/// return exact .10d bytes for the existing portal. Unknown HMC resources stay
/// inert; this does not evaluate Vibe, grant rights or replace a game release.
#[wasm_bindgen]
pub fn inspect_candidate_import(
    bytes: &[u8],
    is_bundle: bool,
    selected_key: &str,
) -> Result<JsValue, JsValue> {
    let (receipt, asset) =
        inspect(bytes, is_bundle, selected_key).map_err(|e| JsValue::from_str(&e))?;
    let result = js_sys::Object::new();
    let value = js_sys::JSON::parse(&receipt.to_string())?;
    js_sys::Reflect::set(&result, &"receipt".into(), &value)?;
    if let Some(asset) = asset {
        js_sys::Reflect::set(
            &result,
            &"bytes".into(),
            &js_sys::Uint8Array::from(asset.as_slice()).into(),
        )?;
    }
    Ok(result.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::bundle::BundleWriter;
    #[test]
    fn explicit_candidate_selection_returns_exact_canonical_mesh_bytes() {
        use qualia_core_db::{
            container_10d::{attach_material_section, MaterialRecord, SubmeshRange},
            render::{assets::Mesh, compile_10d::compile_mesh_to_10d},
        };
        let mesh = Mesh {
            positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            triangles: vec![[0, 1, 2]],
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 0.0],
        };
        let material = MaterialRecord::legacy_default();
        let encoded = attach_material_section(
            &compile_mesh_to_10d(&mesh).unwrap(),
            &[material],
            &[SubmeshRange {
                first_index: 0,
                index_count: 3,
                material_id: material.id,
                semantic_id: 0,
            }],
            3,
        )
        .unwrap();
        let (standalone, chosen) = inspect(&encoded, false, "standalone.10d").unwrap();
        assert_eq!(chosen.as_deref(), Some(encoded.as_slice()));
        assert_eq!(standalone["input_sha256"], sha(&encoded));
        let mut writer = BundleWriter::new();
        writer
            .add_file("review/model.10d", "10d", encoded.clone(), None)
            .unwrap();
        writer
            .add_file(
                "review/manifest.json",
                "manifest",
                br#"{"master":"declared-master","accepted":false}"#.to_vec(),
                None,
            )
            .unwrap();
        let pack = writer.build().unwrap();
        let (receipt, selected) = inspect(&pack, true, "review/model.10d").unwrap();
        assert_eq!(selected.as_deref(), Some(encoded.as_slice()));
        assert_eq!(receipt["assets"][0]["sha256"], sha(&encoded));
        assert_eq!(
            receipt["declarations"][0]["declaration_only"]["accepted"],
            false
        );
    }
    #[test]
    fn selection_is_explicit_and_unknown_resources_stay_inert() {
        let mut writer = BundleWriter::new();
        writer
            .add_file("candidate.10d", "10d", vec![1, 2, 3], None)
            .unwrap();
        writer
            .add_file("source.vibe", "vibe", b"never execute this".to_vec(), None)
            .unwrap();
        let bytes = writer.build().unwrap();
        let (receipt, selected) = inspect(&bytes, true, "").unwrap();
        assert!(selected.is_none());
        assert_eq!(receipt["assets"][0]["key"], "candidate.10d");
        assert!(inspect(&bytes, true, "absent.10d").is_err());
        assert!(inspect(&bytes, true, "candidate.10d").is_err()); // invalid canonical asset
        let mut corrupt = bytes;
        corrupt[64] ^= 1;
        assert!(inspect(&corrupt, true, "").is_err());
    }
}
