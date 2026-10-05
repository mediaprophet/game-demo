//! Original stylized ant geometry assembled from Qualia authored surfaces.

use qualia_core_db::render::assets::Mesh;
use qualia_core_db::specialized_libs::computational_geometry::authoring;

fn rotate(x: f32, z: f32, heading: f32) -> (f32, f32) {
    (
        x * heading.cos() - z * heading.sin(),
        x * heading.sin() + z * heading.cos(),
    )
}

fn combine(parts: Vec<Mesh>) -> Mesh {
    let mut positions = Vec::new();
    let mut triangles = Vec::new();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for part in parts {
        let offset = positions.len() as u32;
        for p in part.positions {
            for axis in 0..3 {
                min[axis] = min[axis].min(p[axis]);
                max[axis] = max[axis].max(p[axis]);
            }
            positions.push(p);
        }
        triangles.extend(
            part.triangles
                .into_iter()
                .map(|t| [t[0] + offset, t[1] + offset, t[2] + offset]),
        );
    }
    Mesh {
        positions,
        triangles,
        min,
        max,
    }
}

fn garden_scale(mut mesh: Mesh, center: [f32; 3]) -> Mesh {
    const SCALE: f32 = 0.40;
    const GROUND_Y: f32 = 0.09;
    mesh.min = [f32::INFINITY; 3];
    mesh.max = [f32::NEG_INFINITY; 3];
    for p in &mut mesh.positions {
        for axis in 0..3 {
            p[axis] = center[axis] + (p[axis] - center[axis]) * SCALE;
        }
        p[1] += GROUND_Y - center[1];
        for axis in 0..3 {
            mesh.min[axis] = mesh.min[axis].min(p[axis]);
            mesh.max[axis] = mesh.max[axis].max(p[axis]);
        }
    }
    mesh
}

fn body_part(
    center: [f32; 3],
    heading: f32,
    offset: [f32; 3],
    radii: [f32; 3],
) -> Result<Mesh, String> {
    let sphere = authoring::uv_sphere(1.0, 8, 12).map_err(|e| e.to_string())?;
    let (dx, dz) = rotate(offset[0], offset[2], heading);
    let matrix = authoring::compose_trs(
        (center[0] + dx) as f64,
        (center[1] + offset[1]) as f64,
        (center[2] + dz) as f64,
        0.0,
        heading as f64,
        0.0,
        radii[0] as f64,
        radii[1] as f64,
        radii[2] as f64,
    );
    Ok(authoring::transform_mesh(&sphere, &matrix))
}

pub fn ant(center: [f32; 3], heading: f32) -> Result<Mesh, String> {
    let mut parts = vec![
        body_part(center, heading, [0.17, 0.0, 0.0], [0.16, 0.115, 0.12])?,
        body_part(center, heading, [0.0, 0.0, 0.0], [0.09, 0.085, 0.09])?,
        body_part(center, heading, [-0.16, 0.025, 0.0], [0.115, 0.105, 0.11])?,
    ];
    let leg = authoring::cylinder(0.014, 0.25, 7).map_err(|e| e.to_string())?;
    for row in [-0.11_f32, 0.0, 0.11] {
        for side in [-1.0_f32, 1.0] {
            let (dx, dz) = rotate(row, side * 0.13, heading);
            let m = authoring::compose_trs(
                (center[0] + dx) as f64,
                (center[1] - 0.10) as f64,
                (center[2] + dz) as f64,
                (side * 0.75) as f64,
                heading as f64,
                ((row / 0.11) * 0.30) as f64,
                1.0,
                1.0,
                1.0,
            );
            parts.push(authoring::transform_mesh(&leg, &m));
        }
    }
    let antenna = authoring::cylinder(0.010, 0.15, 6).map_err(|e| e.to_string())?;
    for side in [-1.0_f32, 1.0] {
        let (dx, dz) = rotate(-0.23, side * 0.055, heading);
        let m = authoring::compose_trs(
            (center[0] + dx) as f64,
            (center[1] + 0.16) as f64,
            (center[2] + dz) as f64,
            (side * 0.38) as f64,
            heading as f64,
            -0.36,
            1.0,
            1.0,
            1.0,
        );
        parts.push(authoring::transform_mesh(&antenna, &m));
    }
    Ok(garden_scale(combine(parts), center))
}

pub fn eyes(center: [f32; 3], heading: f32) -> Result<Mesh, String> {
    Ok(garden_scale(
        combine(
            [-1.0_f32, 1.0]
                .into_iter()
                .map(|side| {
                    body_part(
                        center,
                        heading,
                        [-0.245, 0.075, side * 0.06],
                        [0.024, 0.025, 0.024],
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
        ),
        center,
    ))
}
