//! Deterministic game-authored heightfield above the two settlement tiles.
//! QualiaDB owns triangulation; these functions only define the world shape.

use qualia_core_db::render::assets::Mesh;
use qualia_core_db::domains::geospatial::dem::generate_terrain_mesh;

const SIDE: usize = 17;
const SIZE: f32 = 14.0;

pub fn height(x: f32, z: f32) -> f32 {
    let edge_x = ((x + 7.0) / 2.5).clamp(0.0, 1.0)
        * ((23.0 - x) / 2.5).clamp(0.0, 1.0);
    let edge_z = ((-7.0 - z) / 2.2).clamp(0.0, 1.0)
        * ((z + 21.0) / 2.2).clamp(0.0, 1.0);
    let hill = |cx: f32, cz: f32, rx: f32, rz: f32| {
        let dx = (x - cx) / rx;
        let dz = (z - cz) / rz;
        (-2.0 * (dx * dx + dz * dz)).exp()
    };
    let west = 3.3 * hill(-2.4, -15.6, 4.6, 4.8);
    let east = 3.8 * hill(17.9, -14.5, 4.4, 5.2);
    let knolls = 0.45 * hill(2.7, -18.2, 2.7, 2.2)
        + 0.35 * hill(13.1, -18.0, 3.0, 2.3);
    let rolling = 0.12 * ((x * 0.89).sin() * (z * 0.71).cos() + 1.0);
    ((west + east + knolls + rolling) * edge_x * edge_z).max(0.0)
}

/// A patch in the same world coordinate system as Kestrel and Saltwind.
pub fn patch(center_x: f32, center_z: f32) -> Mesh {
    let step = SIZE / (SIDE - 1) as f32;
    let heights: Vec<f32> = (0..SIDE)
        .flat_map(|row| (0..SIDE).map(move |column| {
            height(center_x - SIZE * 0.5 + column as f32 * step,
                   center_z - SIZE * 0.5 + row as f32 * step)
        }))
        .collect();
    let terrain = generate_terrain_mesh(&heights, SIDE, SIDE, step as f64);
    // Qualia's DEM is XY with elevation Z. Portal uses XZ with elevation Y.
    // The DEM centers by SIDE*step/2, hence the half-cell correction.
    let positions: Vec<[f32; 3]> = terrain.vertices.iter().map(|p| [
        p[0] + center_x + step * 0.5,
        p[2] - 0.02,
        p[1] + center_z + step * 0.5,
    ]).collect();
    let triangles: Vec<[u32; 3]> = terrain.indices.chunks_exact(3)
        .map(|tri| [tri[0], tri[2], tri[1]]).collect();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(p[axis]);
            max[axis] = max[axis].max(p[axis]);
        }
    }
    Mesh { positions, triangles, min, max }
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
    }
}
