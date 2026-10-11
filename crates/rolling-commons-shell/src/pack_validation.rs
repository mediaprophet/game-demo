//! Cold-path validation for game HMC packs using QualiaDB's container readers.
//!
//! This is deliberately a verifier over the engine formats, not a game-owned
//! `.10d` or HMC parser. It runs while exporting/loading a pack, never per frame.

use std::collections::BTreeSet;

use qualia_core_db::bundle::BundleReader;
use qualia_core_db::container_10d::{
    compute_whole_file_crc32c, decode_mesh_section, material_container_counts, parse_mesh_header,
    parse_section_table, read_material_section_into, Container10dHeader, MaterialRecord,
    SectionType, SubmeshRange,
};

const MAX_PACK_ENTRIES: usize = 4096;
const MAX_MATERIALS_PER_ASSET: usize = 256;
const MAX_RANGES_PER_ASSET: usize = 4096;

pub fn verify_game_pack(bytes: &[u8]) -> Result<u32, String> {
    let reader = BundleReader::parse(bytes).map_err(|error| format!("HMC: {error:?}"))?;
    if reader.entries().is_empty() || reader.entries().len() > MAX_PACK_ENTRIES + 1 {
        return Err("HMC entry count is outside game bounds".into());
    }

    let mut keys = BTreeSet::new();
    let mut manifest_count = 0usize;
    for entry in reader.entries() {
        if !keys.insert(entry.key.as_str()) {
            return Err(format!("duplicate HMC entry key: {}", entry.key));
        }
        if !reader.verify_entry(&entry.key) {
            return Err(format!("HMC digest mismatch: {}", entry.key));
        }
        match entry.kind.as_str() {
            "manifest" if entry.key == "manifest.json" => {
                manifest_count += 1;
            }
            "10d" if entry.key.starts_with("10d/") && entry.key.ends_with(".10d") => {
                let asset = reader
                    .get(&entry.key)
                    .ok_or_else(|| format!("HMC asset is missing: {}", entry.key))?;
                verify_10d_asset(asset).map_err(|error| format!("{}: {error}", entry.key))?;
            }
            _ => return Err(format!("unsupported game HMC entry: {}", entry.key)),
        }
    }
    if manifest_count != 1 {
        return Err("game HMC must contain exactly one manifest.json".into());
    }
    Ok(reader.entries().len() as u32)
}

fn verify_10d_asset(bytes: &[u8]) -> Result<(), String> {
    let header = Container10dHeader::parse(bytes).map_err(|error| format!("header: {error:?}"))?;
    let actual_crc = compute_whole_file_crc32c(bytes);
    if actual_crc != header.header_crc32c {
        return Err(format!(
            "whole-file CRC mismatch: expected {:#010x}, got {actual_crc:#010x}",
            header.header_crc32c
        ));
    }

    let sections =
        parse_section_table(bytes, &header).map_err(|error| format!("section table: {error:?}"))?;
    let mut mesh_sections = sections
        .iter()
        .filter(|section| section.typ() == Some(SectionType::QuantizedMesh));
    let mesh_section = mesh_sections
        .next()
        .ok_or_else(|| "quantized mesh section is missing".to_owned())?;
    if mesh_sections.next().is_some() {
        return Err("multiple quantized mesh sections are unsupported".into());
    }

    let start = mesh_section.byte_offset as usize;
    let end = start
        .checked_add(mesh_section.byte_length as usize)
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| "quantized mesh section is out of bounds".to_owned())?;
    let mesh_bytes = &bytes[start..end];
    let (mesh_header, encoded_len) =
        parse_mesh_header(mesh_bytes).map_err(|error| format!("mesh header: {error:?}"))?;
    if encoded_len != mesh_bytes.len()
        || mesh_header.vertex_count == 0
        || mesh_header.triangle_count == 0
        || mesh_header
            .min
            .iter()
            .chain(mesh_header.max.iter())
            .any(|value| !value.is_finite())
        || (0..3).any(|axis| mesh_header.min[axis] > mesh_header.max[axis])
    {
        return Err("mesh payload has invalid bounds, counts, or trailing bytes".into());
    }

    let mesh =
        decode_mesh_section(mesh_bytes).map_err(|error| format!("mesh decode: {error:?}"))?;
    if mesh.positions.len() != mesh_header.vertex_count as usize
        || mesh.triangles.len() != mesh_header.triangle_count as usize
        || mesh
            .positions
            .iter()
            .flatten()
            .any(|value| !value.is_finite())
        || mesh
            .triangles
            .iter()
            .flatten()
            .any(|&index| index as usize >= mesh.positions.len())
    {
        return Err("decoded mesh has non-finite vertices or invalid triangle indices".into());
    }

    let mesh_index_count = mesh_header
        .triangle_count
        .checked_mul(3)
        .ok_or_else(|| "mesh index count overflow".to_owned())?;
    let (material_count, range_count) =
        material_container_counts(bytes).map_err(|error| format!("material section: {error:?}"))?;
    if material_count == 0
        || material_count > MAX_MATERIALS_PER_ASSET
        || range_count == 0
        || range_count > MAX_RANGES_PER_ASSET
    {
        return Err("material or submesh count is outside game bounds".into());
    }
    let mut materials = vec![MaterialRecord::legacy_default(); material_count];
    let mut ranges = vec![
        SubmeshRange {
            first_index: 0,
            index_count: 0,
            material_id: 0,
            semantic_id: 0,
        };
        range_count
    ];
    read_material_section_into(bytes, mesh_index_count, &mut materials, &mut ranges)
        .map_err(|error| format!("material bindings: {error:?}"))?;
    Ok(())
}
