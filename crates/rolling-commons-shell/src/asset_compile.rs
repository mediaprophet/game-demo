//! Shared cold asset compilation for community and focused activity scenes.
use crate::asset_catalog;
use qualia_core_db::container_10d::provenance_section::ProvenanceSidecar;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, Compile10dExtras,
};
use qualia_core_db::render::scene_primitives::{assemble_into, recipe_write, Primitive};
use qualia_core_db::specialized_libs::computational_geometry::geometry_workspace::{
    Cancellation, GeometryWorkspace,
};
use wasm_bindgen::prelude::*;
/// Compile one authored asset recipe into its mesh and `.10d` receipt
/// object. The scene is composed of these compiled assets.
pub(crate) fn compile_asset(
    recipe: &asset_catalog::AssetRecipe,
) -> Result<(Mesh, JsValue), JsValue> {
    let (mesh, source, mime) = if let Some(spec) = &recipe.parametric {
        let mesh = spec
            .compile()
            .map_err(|e| JsValue::from_str(&format!("parametric geometry: {e}")))?;
        if let Some(src) = &recipe.vibe_source {
            (
                mesh,
                src.as_bytes().to_vec(),
                "application/vnd.rolling-commons.vibe-scene;version=1",
            )
        } else {
            (
                mesh,
                spec.source_bytes(),
                "application/vnd.rolling-commons.parametric-recipe;version=1",
            )
        }
    } else {
        let parts: &[Primitive] = &recipe.parts;
        let mut positions = vec![[0.0; 3]; parts.len() * 8];
        let mut triangles = vec![[0; 3]; parts.len() * 12];
        let mut arena = vec![0u8; 64 + parts.len() * 64];
        let cancel = Cancellation::new();
        let mut workspace = GeometryWorkspace::new(&mut arena, &cancel);
        let receipt = assemble_into(parts, &mut positions, &mut triangles, &mut workspace)
            .map_err(|e| {
                JsValue::from_str(&format!("scene geometry: {e:?} in recipe {}", recipe.id))
            })?;
        positions.truncate(receipt.vertex_count);
        triangles.truncate(receipt.triangle_count);
        let mesh = Mesh {
            positions,
            triangles,
            min: receipt.min,
            max: receipt.max,
        };
        let mut source = vec![0u8; 6 + parts.len() * 25];
        recipe_write(parts, &mut source)
            .map_err(|e| JsValue::from_str(&format!("scene recipe: {e:?}")))?;
        (
            mesh,
            source,
            "application/vnd.qualia.scene-primitives;version=1",
        )
    };
    let resident_binding = crate::resident_animation::author(recipe, &mesh.positions)
        .map_err(|e| JsValue::from_str(&e))?;
    let resident_binding = if resident_binding.is_some() {
        resident_binding
    } else {
        crate::vehicle_animation::author(recipe, &mesh.positions)
            .map_err(|e| JsValue::from_str(&e))?
    };
    let resident_binding = if resident_binding.is_some() {
        resident_binding
    } else {
        crate::fauna_animation::author(recipe, &mesh.positions)
            .map_err(|e| JsValue::from_str(&e))?
    };
    let resident_binding = if resident_binding.is_some() {
        resident_binding
    } else {
        crate::quadruped_animation::author(recipe, &mesh.positions)
            .map_err(|e| JsValue::from_str(&e))?
    };
    let prepared = crate::material_library::prepare_surface(recipe, mesh)
        .map_err(|e| JsValue::from_str(&e))?;
    let mesh = prepared.mesh;
    // The scene is original authored data with no external reuse grant.
    // This records that status; it does not alter the QualiaDB licence.
    let provenance =
        ProvenanceSidecar::new(source, mime, "All rights reserved (licence not assigned)");
    // The current Qualia .10d SRD1 section preserves authored colour on
    // each vertex. A deterministic palette reading gives broad surfaces
    // gentle height/position variation without a game-specific format.
    let terrain_surface = recipe.id.contains("terrain-");
    let landscape = terrain_surface
        || recipe.id.contains("ground")
        || recipe.id.contains("grass")
        || recipe.id.contains("hill")
        || recipe.id.contains("bed");
    let foliage = recipe.id.contains("crown")
        || recipe.id.contains("canopy")
        || recipe.id.contains("shoot")
        || recipe.id.contains("flower");
    let span_y = (mesh.max[1] - mesh.min[1]).max(0.01);
    let reading: Vec<[f32; 4]> = mesh
        .positions
        .iter()
        .map(|p| {
            let elevation = ((p[1] - mesh.min[1]) / span_y).clamp(0.0, 1.0);
            let ripple = (p[0] * 1.17 + p[2] * 0.63).sin() * 0.055
                + (p[0] * 0.31 - p[2] * 1.09).cos() * 0.035;
            let gain = if landscape {
                0.94 + ripple * 1.5
            } else {
                0.91 + elevation * 0.14 + ripple
            };
            let green = if terrain_surface {
                1.07 - elevation * 0.24
            } else if foliage || landscape {
                1.04
            } else {
                1.0
            };
            let red = if terrain_surface {
                1.0 + elevation * 0.18
            } else {
                1.0
            };
            [
                (recipe.color[0] * gain * red).clamp(0.0, 1.0),
                (recipe.color[1] * gain * green).clamp(0.0, 1.0),
                (recipe.color[2] * gain).clamp(0.0, 1.0),
                recipe.color[3],
            ]
        })
        .collect();
    let bytes = compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&provenance),
        Compile10dExtras::default(),
        Some(&reading),
        Some(&prepared.normals),
        None,
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
    // Family and factors are authored on each game recipe. The Qualia
    // scene loader retains this record when it merges the .10d meshes.
    let family = recipe.material_family;
    let material = crate::material_library::material(recipe).map_err(|e| JsValue::from_str(&e))?;
    let ranges = [qualia_core_db::container_10d::SubmeshRange {
        first_index: 0,
        index_count: (mesh.triangles.len() * 3) as u32,
        material_id: material.id,
        semantic_id: qualia_core_db::q_hash(&recipe.id),
    }];
    let bytes = qualia_core_db::container_10d::attach_material_section(
        &bytes,
        &[material],
        &ranges,
        ranges[0].index_count,
    )
    .map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    let obj = js_sys::Object::new();
    let bytes =
        qualia_core_db::container_10d::attach_texture_coordinate_section(&bytes, &prepared.uv)
            .map_err(|e| JsValue::from_str(&format!("UV01: {e:?}")))?;
    let bytes = if let Some(binding) = resident_binding {
        crate::resident_animation::attach(binding, &prepared.source_vertices, &bytes)
            .map_err(|e| JsValue::from_str(&e))?
    } else {
        bytes
    };
    js_sys::Reflect::set(&obj, &"id".into(), &JsValue::from_str(&recipe.id))?;
    js_sys::Reflect::set(
        &obj,
        &"renderRole".into(),
        &JsValue::from_str(&recipe.render_role),
    )?;
    js_sys::Reflect::set(
        &obj,
        &"materialFamily".into(),
        &JsValue::from_str(family.id),
    )?;
    js_sys::Reflect::set(
        &obj,
        &"bytes".into(),
        &js_sys::Uint8Array::from(bytes.as_slice()),
    )?;
    for (key, value) in ["r", "g", "b", "a"].iter().zip(recipe.color) {
        js_sys::Reflect::set(&obj, &(*key).into(), &JsValue::from_f64(value as f64))?;
    }
    Ok((mesh, obj.into()))
}
