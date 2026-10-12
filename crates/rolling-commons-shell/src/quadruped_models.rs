//! Original species proportions and articulated source meshes. Qualia's
//! parametric primitives compile the geometry and its surface attributes.
use crate::asset_catalog::{
    material_family_for, AssetRecipe, LimbSegment, ParametricRecipe as Shape,
};
#[derive(Clone, Copy)]
pub(crate) struct Profile {
    pub height: f32,
    pub width: f32,
    pub length: f32,
    pub head_y: f32,
    pub coat: [f32; 4],
    pub wombat: bool,
}
pub(crate) fn profile(id: &str) -> Option<Profile> {
    let (height, width, length, head_y, coat, wombat) = if id.contains("goat") {
        (0.78, 0.26, 0.62, 1.18, [0.72, 0.65, 0.49, 1.0], false)
    } else if id.contains("horse") {
        (1.30, 0.39, 0.78, 1.98, [0.45, 0.25, 0.14, 1.0], false)
    } else if id.contains("deer") {
        (1.05, 0.27, 0.62, 1.63, [0.51, 0.34, 0.22, 1.0], false)
    } else if id.contains("wombat") {
        (0.36, 0.30, 0.43, 0.39, [0.43, 0.40, 0.34, 1.0], true)
    } else {
        return None;
    };
    Some(Profile {
        height,
        width,
        length,
        head_y,
        coat,
        wombat,
    })
}
pub(crate) fn joints(p: Profile) -> [[f32; 3]; 19] {
    let mut points = [[0.0; 3]; 19];
    points[1] = [0.0, p.height, 0.0];
    points[2] = [0.0, p.height * 1.03, -p.length * 0.56];
    points[3] = [0.0, p.head_y, -p.length * 0.95];
    points[4] = [-p.width * 0.32, p.head_y + 0.12, -p.length * 0.88];
    points[5] = [p.width * 0.32, p.head_y + 0.12, -p.length * 0.88];
    points[6] = [0.0, p.height * 1.08, p.length * 0.82];
    for (i, (side, end)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        let base = 7 + i * 3;
        let z = end * p.length * 0.64;
        points[base] = [side * p.width * 0.78, p.height * 0.92, z];
        points[base + 1] = [
            side * p.width * 0.87,
            p.height * 0.47,
            z + if end < 0.0 { 0.035 } else { -0.055 },
        ];
        points[base + 2] = [side * p.width * 0.90, 0.055, z - 0.035];
    }
    points
}
fn ellipsoid(center: [f32; 3], radii: [f32; 3]) -> Shape {
    Shape::Ellipsoid {
        center,
        radii,
        latitude: 18,
        longitude: 28,
    }
}
fn limbs(list: &[([f32; 3], [f32; 3], f32, f32)]) -> Shape {
    Shape::LimbCluster {
        center: [0.0; 3],
        radial_segments: 12,
        limbs: list
            .iter()
            .map(|(a, b, r, s)| LimbSegment {
                start: *a,
                end: *b,
                start_radius: *r,
                end_radius: *s,
            })
            .collect(),
    }
}
pub(crate) fn models(prefix: &str, origin: [f32; 3]) -> Vec<AssetRecipe> {
    let p = profile(prefix).expect("authored quadruped profile");
    let j = joints(p);
    let mut out = Vec::new();
    let mut add = |part: &str, color: [f32; 4], source: Shape| {
        let id = format!("rc:asset/fauna/{prefix}/{part}");
        out.push(AssetRecipe {
            material_family: material_family_for(&id),
            id,
            parts: Vec::new(),
            parametric: Some(Shape::Transform {
                source: Box::new(source),
                pivot: [0.0; 3],
                target: origin,
                scale: [1.0; 3],
            }),
            color,
            vibe_source: None,
            render_role: "opaque".into(),
        });
    };
    add(
        if p.wombat { "body" } else { "coat" },
        p.coat,
        ellipsoid(
            [0.0, p.height, 0.0],
            [
                p.width,
                if p.wombat { 0.25 } else { p.height * 0.32 },
                p.length,
            ],
        ),
    );
    if p.wombat {
        add(
            "shoulders",
            p.coat,
            ellipsoid(
                [0.0, p.height, -p.length * 0.40],
                [p.width * 0.91, 0.19, p.length * 0.58],
            ),
        );
    }
    add(
        "neck",
        p.coat,
        limbs(&[(j[2], j[3], p.width * 0.65, p.width * 0.43)]),
    );
    add(
        "head",
        p.coat,
        ellipsoid(
            j[3],
            [
                p.width * 0.52,
                if p.wombat { 0.13 } else { 0.18 },
                p.length * 0.29,
            ],
        ),
    );
    let nose_z = j[3][2] - p.length * 0.28;
    add(
        if p.wombat { "nose" } else { "muzzle" },
        [0.25, 0.20, 0.16, 1.0],
        ellipsoid(
            [0.0, p.head_y - 0.055, nose_z],
            [p.width * 0.40, 0.085, p.length * 0.21],
        ),
    );
    for (index, side) in ["left", "right"].into_iter().enumerate() {
        let sign = if index == 0 { -1.0 } else { 1.0 };
        add(
            &format!("ear-{side}"),
            p.coat,
            ellipsoid(
                j[4 + index],
                [
                    if p.wombat { 0.045 } else { 0.06 },
                    if p.wombat { 0.055 } else { 0.13 },
                    0.035,
                ],
            ),
        );
        add(
            &format!("eye-{side}"),
            [0.045, 0.036, 0.028, 1.0],
            ellipsoid(
                [sign * p.width * 0.48, p.head_y + 0.035, j[3][2] - 0.025],
                [0.025, 0.026, 0.023],
            ),
        );
        if prefix.contains("goat") {
            let a = [sign * 0.08, p.head_y + 0.13, j[3][2] + 0.05];
            let b = [sign * 0.12, p.head_y + 0.37, j[3][2] + 0.14];
            add(
                if index == 0 { "horns" } else { "horns-right" },
                [0.31, 0.24, 0.16, 1.0],
                limbs(&[(a, b, 0.035, 0.008)]),
            );
        }
        if prefix.contains("deer") {
            let a = [sign * 0.10, p.head_y + 0.12, j[3][2] + 0.04];
            let b = [sign * 0.27, p.head_y + 0.57, j[3][2] + 0.14];
            add(
                &format!("antler-{side}"),
                [0.64, 0.55, 0.39, 1.0],
                limbs(&[(a, b, 0.033, 0.008)]),
            );
            add(
                &format!("antler-tine-{side}"),
                [0.64, 0.55, 0.39, 1.0],
                limbs(&[(
                    [sign * 0.17, p.head_y + 0.32, j[3][2] + 0.09],
                    [sign * 0.30, p.head_y + 0.46, j[3][2] - 0.08],
                    0.02,
                    0.005,
                )]),
            );
        }
    }
    if !p.wombat {
        add(
            "tail",
            p.coat,
            limbs(&[(j[6], [0.03, p.height * 0.63, p.length * 1.35], 0.065, 0.025)]),
        );
    }
    if prefix.contains("horse") {
        add(
            "mane",
            [0.14, 0.105, 0.08, 1.0],
            limbs(&[(
                [0.0, p.height * 1.12, -p.length * 0.44],
                [0.0, p.head_y + 0.12, j[3][2] + 0.15],
                0.065,
                0.035,
            )]),
        );
    }
    let mut feet = Vec::new();
    for (index, (end, side)) in [
        ("front", "left"),
        ("front", "right"),
        ("rear", "left"),
        ("rear", "right"),
    ]
    .into_iter()
    .enumerate()
    {
        let b = 7 + index * 3;
        let radius = if p.wombat { 0.10 } else { p.width * 0.27 };
        add(
            &format!("leg-{end}-{side}"),
            p.coat,
            limbs(&[
                (j[b], j[b + 1], radius, radius * 0.70),
                (j[b + 1], j[b + 2], radius * 0.70, radius * 0.42),
            ]),
        );
        feet.push(crate::asset_catalog::EllipsoidInstance {
            center: [j[b + 2][0], 0.035, j[b + 2][2] - 0.02],
            radii: [radius * 0.70, 0.035, radius * 0.9],
            pitch_deg: 0.0,
            yaw_deg: 0.0,
            roll_deg: 0.0,
        });
    }
    add(
        if p.wombat { "paws" } else { "hooves" },
        [0.19, 0.17, 0.14, 1.0],
        Shape::EllipsoidCluster {
            instances: feet,
            latitude: 12,
            longitude: 16,
        },
    );
    out
}
