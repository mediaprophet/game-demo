//! Continuous topological terrain elevation and horizon skirt for Talbingo camping sectors.
//! Grounded in surveyed Kosciuszko National Park geography and NSW Seamless Geology stratigraphy.

use crate::asset_catalog::{material_family_for, AssetRecipe, ParametricRecipe};

/// Returns the authentic continuous topological terrain elevation at any (x, z) coordinate.
/// Includes the Hermite radial horizon skirt to smoothly eliminate edge voids.
pub fn elevation(site: u8, x: f32, z: f32) -> f32 {
    let local = match site {
        // Sector 0 — The Pines: Alluvial terrace descending into Blowering Reservoir lake shelf
        0 => {
            let lake_shelf = if z < -1.5 {
                -0.05 * (z + 1.5) * (z + 1.5)
            } else {
                0.0
            };
            lake_shelf + 0.04 * (x - 8.0) + 0.08 * (0.35 * x).sin() * (0.45 * z).cos()
        }
        // Sector 1 — Humes Crossing: Undulating meander terrace incised by an inlet swale
        1 => {
            let swale = -0.55 * (-((z - 0.35 * x + 1.2).powi(2) / 2.8)).exp();
            swale + 0.12 * (0.32 * x).sin()
        }
        // Sector 2 — Jounama Creek: Asymmetric V-gorge, rising to south, dipping to gravel bed
        2 => {
            let bank = if z > 0.0 { 0.18 * z } else { 0.0 };
            let bed = -0.75 * (-((z + 3.5).powi(2) / 2.2)).exp();
            bank + bed + 0.05 * (0.85 * x).cos()
        }
        // Sector 3 — Yarrangobilly Village: Karst limestone canyon, northern rim bluff
        3 => {
            let bluff = 2.4 / (1.0 + (-1.2 * (z - 2.8)).exp());
            let river = -0.42 * (-((z + 3.0).powi(2) / 2.4)).exp();
            bluff + river
        }
        // Sector 4 — O'Hares Rest Area: Talbingo Mountain scarp, knife-edge drop to west
        4 => {
            let scarp = if x < 6.0 {
                -0.12 * (6.0 - x).powi(2)
            } else {
                0.0
            };
            scarp + 0.32 * (0.24 * z).sin()
        }
        // Sector 5 — Three Mile Dam: High alpine plateau, rolling tussock and tarn depression
        _ => {
            let tarn = -0.48 * (-(((x - 18.0).powi(2) + (z + 2.5).powi(2)) / 22.0)).exp();
            0.22 * (0.22 * x).sin() * (0.28 * z).cos() + tarn
        }
    };

    // Hermite transition from campsite center (R <= 16m)
    // into continuous surrounding Kosciuszko alpine ridgelines without edge void or clamping.
    let r = ((x - 8.2).powi(2) + z.powi(2)).sqrt();
    if r <= 16.0 {
        local
    } else {
        let t = ((r - 16.0) / 40.0).clamp(0.0, 1.0);
        let hermite = t * t * (3.0 - 2.0 * t);
        let ridge = 0.65 + 0.045 * r + 0.85 * (0.12 * x).sin() * (0.12 * z).cos()
            + 0.35 * (0.045 * x).sin() * (0.045 * z).sin();
        (1.0 - hermite) * local + hermite * ridge
    }
}

/// Builds the continuous watertight terrain heightfield asset recipe for a sector.
pub fn build_terrain_asset(site: u8) -> AssetRecipe {
    let cols = 48;
    let rows = 36;
    let min_x = -24.0;
    let max_x = 40.0;
    let min_z = -22.0;
    let max_z = 22.0;

    let mut heights = Vec::with_capacity(cols * rows);
    let dx = (max_x - min_x) / (cols - 1) as f32;
    let dz = (max_z - min_z) / (rows - 1) as f32;

    for r in 0..rows {
        let z = min_z + r as f32 * dz;
        for c in 0..cols {
            let x = min_x + c as f32 * dx;
            heights.push(elevation(site, x, z));
        }
    }

    let recipe = ParametricRecipe::TerrainHeightfield {
        min_x,
        max_x,
        min_z,
        max_z,
        cols,
        rows,
        heights,
    };

    let id = "talbingo-ground".to_string();
    let material_family = material_family_for(&id);
    AssetRecipe {
        id,
        parts: Vec::new(),
        parametric: Some(recipe),
        color: [0.26, 0.38, 0.19, 1.0], // Subalpine rich grass tone
        material_family,
        vibe_source: None,
        render_role: "opaque".to_string(),
    }
}
