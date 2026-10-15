//! Deterministic game-authored heightfield above the two settlement tiles.
//! QualiaDB owns triangulation; these functions only define the world shape.

use qualia_core_db::domains::geospatial::dem::generate_terrain_mesh;
use qualia_core_db::render::assets::Mesh;

const SIDE: usize = 17;
const SIZE: f32 = 14.0;

pub fn settlement_height(center_x: f32, x: f32, z: f32) -> f32 {
    let edge = (7.0 - (x - center_x).abs()).min(7.0 - z.abs()).max(0.0);
    let t = (edge / 1.5).clamp(0.0, 1.0);
    let eased = t * t * (3.0 - 2.0 * t);
    let gentle = 0.018 * (x * 0.72).sin() * (z * 0.57).cos();
    -0.28 * (1.0 - eased) + gentle * eased
}

pub fn height(x: f32, z: f32) -> f32 {
    let edge_x = ((x + 7.0) / 2.5).clamp(0.0, 1.0) * ((23.0 - x) / 2.5).clamp(0.0, 1.0);
    let edge_z = ((-7.0 - z) / 2.2).clamp(0.0, 1.0) * ((z + 21.0) / 2.2).clamp(0.0, 1.0);
    let hill = |cx: f32, cz: f32, rx: f32, rz: f32| {
        let dx = (x - cx) / rx;
        let dz = (z - cz) / rz;
        (-2.0 * (dx * dx + dz * dz)).exp()
    };
    let west = 3.3 * hill(-2.4, -15.6, 4.6, 4.8);
    let east = 3.8 * hill(17.9, -14.5, 4.4, 5.2);
    let knolls = 0.45 * hill(2.7, -18.2, 2.7, 2.2) + 0.35 * hill(13.1, -18.0, 3.0, 2.3);
    let rolling = 0.12 * ((x * 0.89).sin() * (z * 0.71).cos() + 1.0);
    (west + east + knolls + rolling) * edge_x * edge_z - 0.28 * (1.0 - edge_z)
}

/// A patch in the same world coordinate system as Kestrel and Saltwind.
pub fn patch(center_x: f32, center_z: f32) -> Mesh {
    build_patch(center_x, center_z, SIDE, height)
}

pub fn settlement_patch(center_x: f32) -> Mesh {
    build_patch(center_x, 0.0, 33, |x, z| settlement_height(center_x, x, z))
}

/// A graded landform under each technical tile replaces the vertical box
/// walls. The apron overlaps adjoining land below the playable heightfield,
/// leaving the settlement and ridge samples authoritative for movement.
pub fn apron(center_x: f32, center_z: f32) -> Mesh {
    build_patch_sized(center_x, center_z, 41, 16.0, |x, z| {
        let dx = (x - center_x).abs();
        let dz = (z - center_z).abs();
        let edge = ((dx.max(dz) - 6.6) / 1.4).clamp(0.0, 1.0);
        let eased = edge * edge * (3.0 - 2.0 * edge);
        let underlying = if center_z < -7.0 {
            height(x, z) - 0.12
        } else {
            settlement_height(center_x, x, z) - 0.12
        };
        let erosion = 0.035 * (x * 1.8 + z * 0.7).sin() + 0.02 * (z * 2.1 - x * 0.5).cos();
        underlying * (1.0 - eased) + (-0.64 + erosion) * eased
    })
}

fn build_patch(
    center_x: f32,
    center_z: f32,
    side: usize,
    sample: impl Fn(f32, f32) -> f32 + Copy,
) -> Mesh {
    build_patch_sized(center_x, center_z, side, SIZE, sample)
}

fn build_patch_sized(
    center_x: f32,
    center_z: f32,
    side: usize,
    size: f32,
    sample: impl Fn(f32, f32) -> f32 + Copy,
) -> Mesh {
    let step = size / (side - 1) as f32;
    let heights: Vec<f32> = (0..side)
        .flat_map(|row| {
            (0..side).map(move |column| {
                sample(
                    center_x - size * 0.5 + column as f32 * step,
                    center_z - size * 0.5 + row as f32 * step,
                )
            })
        })
        .collect();
    let terrain = generate_terrain_mesh(&heights, side, side, step as f64);
    // Qualia's DEM is XY with elevation Z. Portal uses XZ with elevation Y.
    // The DEM centers by SIDE*step/2, hence the half-cell correction.
    let positions: Vec<[f32; 3]> = terrain
        .vertices
        .iter()
        .map(|p| {
            [
                p[0] + center_x + step * 0.5,
                p[2] - 0.02,
                p[1] + center_z + step * 0.5,
            ]
        })
        .collect();
    let triangles: Vec<[u32; 3]> = terrain
        .indices
        .chunks_exact(3)
        .map(|tri| [tri[0], tri[2], tri[1]])
        .collect();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(p[axis]);
            max[axis] = max[axis].max(p[axis]);
        }
    }
    Mesh {
        positions,
        triangles,
        min,
        max,
    }
}

/// Continuous multi-frequency landscape heightfield covering extended travel corridors (280m+).
/// Blends continental rolling swales, winding drainage creeks, and mountain foothills.
pub fn continuous_landscape_height(x: f32, z: f32) -> f32 {
    let macro_swale = 1.8 * (x * 0.015).sin() * (z * 0.012).cos();
    let rolling_hills = 0.85 * ((x * 0.045 + 0.3).sin() * (z * 0.038 - 0.4).cos());
    let micro_roughness = 0.08 * ((x * 0.25).sin() * (z * 0.22).cos());
    // Winding drainage depression / creek bed
    let creek_center_x = 8.0 + 4.0 * (z * 0.03).sin();
    let dist_to_creek = (x - creek_center_x).abs();
    let creek_bed = if dist_to_creek < 3.5 {
        let t = dist_to_creek / 3.5;
        -0.45 * (1.0 - t * t)
    } else {
        0.0
    };
    macro_swale + rolling_hills + micro_roughness + creek_bed
}

/// Organic radial distance feathering: eases elevation and density smoothly to horizon base
/// using a perturbed boundary rather than a hard square box edge.
pub fn organic_radial_feather(x: f32, z: f32, center_x: f32, center_z: f32, radius: f32) -> f32 {
    let dx = x - center_x;
    let dz = z - center_z;
    let dist = (dx * dx + dz * dz).sqrt();
    let angle = dz.atan2(dx);
    let perturbed_radius = radius * (1.0 + 0.12 * (angle * 5.0).sin() + 0.08 * (angle * 9.0).cos());
    if dist >= perturbed_radius {
        0.0
    } else if dist <= perturbed_radius * 0.75 {
        1.0
    } else {
        let t = (perturbed_radius - dist) / (perturbed_radius * 0.25);
        t * t * (3.0 - 2.0 * t) // C2 smoothstep
    }
}

/// Dynamic multi-resolution corridor patch generator.
/// - `tier 0`: High-density focal activity site (33x33 vertices)
/// - `tier 1`: Mid-range transit corridor (17x17 vertices)
/// - `tier 2`: Expansive macro corridor landscape (9x9 vertices)
pub fn build_adaptive_corridor_patch(
    center_x: f32,
    center_z: f32,
    width: f32,
    length: f32,
    resolution_tier: u8,
) -> Mesh {
    let side: usize = match resolution_tier {
        0 => 33,
        1 => 17,
        _ => 9,
    };
    let step_x = width / (side - 1) as f32;
    let step_z = length / (side - 1) as f32;

    let heights: Vec<f32> = (0..side)
        .flat_map(|row| {
            (0..side).map(move |col| {
                let x = center_x - width * 0.5 + col as f32 * step_x;
                let z = center_z - length * 0.5 + row as f32 * step_z;
                let base_h = continuous_landscape_height(x, z);
                let feather = organic_radial_feather(x, z, center_x, center_z, (width.max(length)) * 0.5);
                base_h * feather - 0.28 * (1.0 - feather)
            })
        })
        .collect();

    let terrain = generate_terrain_mesh(&heights, side, side, step_x.max(step_z) as f64);
    let positions: Vec<[f32; 3]> = terrain
        .vertices
        .iter()
        .map(|p| [p[0] + center_x, p[2] - 0.02, p[1] + center_z])
        .collect();
    let triangles: Vec<[u32; 3]> = terrain
        .indices
        .chunks_exact(3)
        .map(|tri| [tri[0], tri[2], tri[1]])
        .collect();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(p[axis]);
            max[axis] = max[axis].max(p[axis]);
        }
    }
    Mesh {
        positions,
        triangles,
        min,
        max,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_patches_leave_a_two_metre_creek_corridor() {
        let west = patch(0.0, -14.0);
        let east = patch(16.0, -14.0);
        assert_eq!(west.positions.len(), SIDE * SIDE);
        assert_eq!(west.triangles.len(), (SIDE - 1) * (SIDE - 1) * 2);
        for row in 0..SIDE {
            let left = west.positions[row * SIDE + SIDE - 1];
            let right = east.positions[row * SIDE];
            assert!((right[0] - left[0] - 2.0).abs() < 1e-5);
            assert!(left[1].is_finite() && right[1].is_finite());
            assert!((left[2] - right[2]).abs() < 1e-5);
        }
        assert!(height(-2.4, -15.6) > height(8.0, -15.6) + 1.0);
        assert!(settlement_height(0.0, 0.0, 0.0) > settlement_height(0.0, 7.0, 0.0) + 0.2);
    }
}
