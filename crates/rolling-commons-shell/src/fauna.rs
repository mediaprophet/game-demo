//! Place-scoped, original Australian fauna models for ecology and road scenes.
//!
//! Distribution comes from a regional ecology profile. These models do not
//! imply that a species occurs everywhere in Australia or at a specific road.

use qualia_core_db::render::scene_primitives::Primitive;

use crate::asset_catalog::{material_family_for, AssetRecipe, ParametricRecipe};

const fn stem(start: [f32; 3], end: [f32; 3], radius: f32) -> ParametricRecipe {
    ParametricRecipe::Stem {
        start,
        end,
        radius,
        segments: 8,
    }
}

const fn body(center: [f32; 3], radii: [f32; 3]) -> ParametricRecipe {
    ParametricRecipe::Ellipsoid {
        center,
        radii,
        latitude: 10,
        longitude: 16,
    }
}

fn mesh(id: &str, color: [f32; 4], shape: ParametricRecipe) -> AssetRecipe {
    let asset_id = format!("rc:asset/fauna/{id}");
    let material_family = material_family_for(&asset_id);
    AssetRecipe {
        id: asset_id,
        parts: Vec::new(),
        parametric: Some(shape),
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".to_string(),
    }
}

fn primitives(id: &str, color: [f32; 4], parts: Vec<Primitive>) -> AssetRecipe {
    let asset_id = format!("rc:asset/fauna/{id}");
    let material_family = material_family_for(&asset_id);
    AssetRecipe {
        id: asset_id,
        parts,
        parametric: None,
        color,
        material_family,
        vibe_source: None,
        render_role: "opaque".to_string(),
    }
}

fn goat(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    crate::quadruped_models::models(prefix, origin)
}

pub(crate) fn horse(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    crate::quadruped_models::models(prefix, origin)
}

fn rabbit(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    let [x, y, z] = origin;
    let mut result = vec![
        mesh(
            &format!("{prefix}/body"),
            [0.56, 0.39, 0.26, 1.0],
            body([x, y + 0.39, z], [0.30, 0.29, 0.43]),
        ),
        mesh(
            &format!("{prefix}/haunches"),
            [0.62, 0.46, 0.31, 1.0],
            body([x, y + 0.31, z + 0.22], [0.30, 0.28, 0.30]),
        ),
        mesh(
            &format!("{prefix}/head"),
            [0.66, 0.51, 0.36, 1.0],
            body([x, y + 0.50, z - 0.37], [0.20, 0.19, 0.21]),
        ),
        mesh(
            &format!("{prefix}/ear-left"),
            [0.64, 0.47, 0.33, 1.0],
            stem(
                [x - 0.09, y + 0.59, z - 0.40],
                [x - 0.14, y + 0.91, z - 0.43],
                0.055,
            ),
        ),
        mesh(
            &format!("{prefix}/ear-right"),
            [0.64, 0.47, 0.33, 1.0],
            stem(
                [x + 0.09, y + 0.59, z - 0.40],
                [x + 0.14, y + 0.91, z - 0.43],
                0.055,
            ),
        ),
        mesh(
            &format!("{prefix}/tail"),
            [0.87, 0.82, 0.72, 1.0],
            body([x, y + 0.46, z + 0.46], [0.11, 0.11, 0.12]),
        ),
    ];
    for (side, dx) in [("left", -0.17), ("right", 0.17)] {
        for (end, dz) in [("front", -0.27), ("rear", 0.34)] {
            result.push(mesh(
                &format!("{prefix}/leg-{end}-{side}"),
                [0.43, 0.31, 0.22, 1.0],
                stem(
                    [x + dx, y + 0.29, z + dz],
                    [x + dx, y + 0.06, z + dz],
                    0.055,
                ),
            ));
        }
    }
    result
}

pub(crate) fn macropod(prefix: &str, origin: [f32; 3], wallaby: bool) -> Vec<AssetRecipe> {
    crate::macropod_models::models(prefix, origin, wallaby)
}
pub(crate) fn wombat(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    crate::quadruped_models::models(prefix, origin)
}

pub(crate) fn deer(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    crate::quadruped_models::models(prefix, origin)
}

fn snake(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    let [x, y, z] = origin;
    let mut result = Vec::with_capacity(9);
    for index in 0..9 {
        let phase = index as f32 / 8.0;
        let center = [
            x + phase * 0.82,
            y + 0.09,
            z + (phase * 6.283_185_5 * 1.4).sin() * 0.18,
        ];
        result.push(mesh(
            &format!("{prefix}/coil-{index}"),
            if index % 2 == 0 {
                [0.42, 0.40, 0.26, 1.0]
            } else {
                [0.23, 0.24, 0.18, 1.0]
            },
            body(center, [0.075, 0.07, 0.09]),
        ));
    }
    result.push(mesh(
        &format!("{prefix}/head"),
        [0.42, 0.40, 0.26, 1.0],
        body([x + 0.90, y + 0.11, z], [0.11, 0.08, 0.13]),
    ));
    result
}

pub(crate) fn goanna(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    let [x, y, z] = origin;
    let mut result = vec![
        mesh(
            &format!("{prefix}/body"),
            [0.39, 0.40, 0.29, 1.0],
            body([x, y + 0.26, z], [0.25, 0.24, 0.75]),
        ),
        mesh(
            &format!("{prefix}/head"),
            [0.45, 0.43, 0.31, 1.0],
            body([x, y + 0.24, z - 0.77], [0.17, 0.16, 0.29]),
        ),
        mesh(
            &format!("{prefix}/tail"),
            [0.34, 0.35, 0.25, 1.0],
            stem(
                [x, y + 0.19, z + 0.52],
                [x + 0.03, y + 0.08, z + 1.44],
                0.10,
            ),
        ),
    ];
    for (side, dx) in [("left", -0.20), ("right", 0.20)] {
        for (fore, dz) in [("fore", -0.43), ("hind", 0.39)] {
            result.push(mesh(
                &format!("{prefix}/leg-{fore}-{side}"),
                [0.40, 0.40, 0.29, 1.0],
                stem(
                    [x + dx, y + 0.22, z + dz],
                    [x + dx * 1.32, y + 0.08, z + dz - 0.10],
                    0.07,
                ),
            ));
        }
    }
    result
}

pub(crate) fn bird(prefix: &str, origin: [f32; 3], kookaburra: bool) -> Vec<AssetRecipe> {
    let [x, y, z] = origin;
    let (feathers, breast, size) = if kookaburra {
        ([0.38, 0.39, 0.33, 1.0], [0.78, 0.71, 0.56, 1.0], 1.0)
    } else {
        ([0.10, 0.12, 0.12, 1.0], [0.19, 0.20, 0.19, 1.0], 0.72)
    };
    vec![
        mesh(
            &format!("{prefix}/body"),
            feathers,
            body(
                [x, y + 0.47 * size, z],
                [0.27 * size, 0.27 * size, 0.40 * size],
            ),
        ),
        mesh(
            &format!("{prefix}/breast"),
            breast,
            body(
                [x, y + 0.43 * size, z - 0.20 * size],
                [0.21 * size, 0.22 * size, 0.25 * size],
            ),
        ),
        mesh(
            &format!("{prefix}/wing-left"),
            feathers,
            body(
                [x - 0.24 * size, y + 0.50 * size, z + 0.01 * size],
                [0.09 * size, 0.14 * size, 0.30 * size],
            ),
        ),
        mesh(
            &format!("{prefix}/wing-right"),
            feathers,
            body(
                [x + 0.24 * size, y + 0.50 * size, z + 0.01 * size],
                [0.09 * size, 0.14 * size, 0.30 * size],
            ),
        ),
        mesh(
            &format!("{prefix}/head"),
            feathers,
            body(
                [x, y + 0.72 * size, z - 0.26 * size],
                [0.16 * size, 0.16 * size, 0.18 * size],
            ),
        ),
        mesh(
            &format!("{prefix}/beak"),
            [0.80, 0.58, 0.21, 1.0],
            stem(
                [x, y + 0.70 * size, z - 0.38 * size],
                [x, y + 0.65 * size, z - 0.62 * size],
                0.06 * size,
            ),
        ),
        mesh(
            &format!("{prefix}/tail"),
            feathers,
            stem(
                [x, y + 0.43 * size, z + 0.33 * size],
                [x + 0.02 * size, y + 0.33 * size, z + 0.78 * size],
                0.055 * size,
            ),
        ),
    ]
}

fn roadkill(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    let [x, y, z] = origin;
    let mut result = vec![
        mesh(
            &format!("{prefix}/body"),
            [0.46, 0.34, 0.25, 1.0],
            body([x, y + 0.13, z], [0.28, 0.14, 0.58]),
        ),
        mesh(
            &format!("{prefix}/head"),
            [0.47, 0.35, 0.25, 1.0],
            body([x, y + 0.12, z - 0.55], [0.16, 0.12, 0.22]),
        ),
    ];
    for (side, dx) in [("left", -0.19), ("right", 0.19)] {
        result.push(mesh(
            &format!("{prefix}/leg-{side}"),
            [0.42, 0.31, 0.23, 1.0],
            stem(
                [x + dx, y + 0.13, z - 0.25],
                [x + dx * 1.8, y + 0.055, z - 0.51],
                0.04,
            ),
        ));
    }
    result.extend(bird(
        &format!("{prefix}/crow-a"),
        [x - 0.48, y, z + 0.10],
        false,
    ));
    result.extend(bird(
        &format!("{prefix}/crow-b"),
        [x + 0.45, y, z + 0.25],
        false,
    ));
    result
}

/// Return profile-selected fauna for one authored corridor demonstration.
/// Render keys are strings so place packs can grow without a fixed taxon mask.
pub fn road_hazards(selected: &[String]) -> Vec<AssetRecipe> {
    let has = |key: &str| selected.iter().any(|candidate| candidate.as_str() == key);
    let mut result = Vec::with_capacity(120);
    if has("goat") {
        result.extend(goat("inland-goat-01", [-1.65, 0.0, 0.48]));
    }
    if has("horse") {
        result.extend(horse("inland-horse-01", [17.0, 0.0, 0.49]));
    }
    if has("rabbit") {
        result.extend(rabbit("road-rabbit-01", [-4.0, 0.0, 0.50]));
    }
    if has("kangaroo") {
        result.extend(macropod("road-kangaroo-01", [2.3, 0.0, 0.50], false));
    }
    if has("wallaby") {
        result.extend(macropod("road-wallaby-01", [10.2, 0.0, 0.50], true));
    }
    if has("wombat") {
        result.extend(wombat("road-wombat-01", [13.0, 0.0, 0.50]));
    }
    if has("deer") {
        result.extend(deer("road-deer-01", [20.2, 0.0, 0.50]));
    }
    if has("roadkill-crows") {
        result.extend(roadkill("roadkill-crows-01", [7.35, 0.0, 0.50]));
    }
    if has("kookaburra") {
        result.extend(bird("road-kookaburra-01", [4.25, 0.0, 1.18], true));
    }
    if has("snake") {
        result.extend(snake("road-snake-01", [8.8, 0.0, 1.00]));
    }
    if has("goanna") {
        result.extend(goanna("road-goanna-01", [15.0, 0.0, 1.00]));
    }
    result
}
