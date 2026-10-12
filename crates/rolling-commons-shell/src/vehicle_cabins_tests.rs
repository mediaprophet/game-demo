//! Authored geometric aperture and material checks; not yet executed.
use super::*;
use wasm_bindgen_test::wasm_bindgen_test;
use qualia_core_db::specialized_libs::computational_geometry::{Point3,ray_triangle_intersect_3d,RayTriangleResult};

fn body_part(id:&str)->bool {
    matches!(id.rsplit('/').next().unwrap(),"body"|"rear-body"|"cabin"|"roof-cap"|
        "window-walls"|"caravan-body"|"caravan-window-walls"|"caravan-roof-cap")
}
#[wasm_bindgen_test]
fn side_windows_have_clear_body_apertures_instead_of_glass_on_solid_lofts() {
    for (rig,y,z) in [(0,1.25,4.65+0.12-0.72),(1,1.37,4.65-0.05),
        (2,1.06,4.65-0.05),(3,1.37,4.65-0.05),(1,1.26,4.65+3.39-0.80)] {
        let models=crate::vehicles::rig_assets(rig);
        let origin=Point3::new(2.35+1.25,y,z);let direction=Point3::new(-1.0,0.0,0.0);
        for model in models.iter().filter(|model|body_part(&model.id)) {
            let mesh=model.parametric.as_ref().unwrap().compile().unwrap();
            for triangle in &mesh.triangles {
                let point=|index:u32|{let p=mesh.positions[index as usize];Point3::new(p[0] as f64,p[1] as f64,p[2] as f64)};
                if let RayTriangleResult::Hit(hit)=ray_triangle_intersect_3d(origin,direction,
                    point(triangle[0]),point(triangle[1]),point(triangle[2])) {
                    assert!(hit.t>2.5,"opaque body blocks aperture: {}",model.id);
                }
            }
        }
    }
}
#[wasm_bindgen_test]
fn glazing_and_interiors_are_separate_material_and_semantic_parts() {
    for rig in 0..4 {
        let models=crate::vehicles::rig_assets(rig);
        let mut ids=std::collections::BTreeSet::new();
        for model in &models {assert!(ids.insert(&model.id),"duplicate model identity");}
        assert!(models.iter().any(|model|model.id.ends_with("steering-wheel")));
        assert!(models.iter().any(|model|model.id.ends_with("seat-0-back")));
        for model in models.iter().filter(|model|model.id.ends_with("glazing")) {
            assert_eq!(model.material_family.id,"framed-glass");
            let material=crate::material_library::material(model).unwrap();
            assert_eq!(material.opacity_mode,qualia_core_db::container_10d::OpacityMode::Blend);
            assert!(material.double_sided);assert!(!material.casts_shadow);
            assert!(material.base_color[3]<1.0);
        }
    }
}

#[wasm_bindgen_test]
fn upper_tyre_quadrants_have_body_clearance_at_the_outer_sidewalls() {
    for (rig,axles,radius,wheel_x) in [
        (0,vec![4.77-1.23,4.77+1.12],0.37,0.94),
        (1,vec![4.65-0.77,4.65+0.78],0.34,0.86),
        (2,vec![4.65-0.84,4.65+0.82],0.30,0.86),
        (3,vec![4.65-0.77,4.65+0.78],0.34,0.86),
    ] {
        let models=crate::vehicles::rig_assets(rig);
        for axle in axles {for angle in [45.0_f64,90.0,135.0] {
            let angle=angle.to_radians();
            let origin=Point3::new(2.35+wheel_x+0.15,0.29+radius*angle.sin(),axle+radius*angle.cos());
            let direction=Point3::new(-1.0,0.0,0.0);
            for model in models.iter().filter(|model|body_part(&model.id)||model.id.contains("guard")) {
                let mesh=model.parametric.as_ref().unwrap().compile().unwrap();
                for triangle in &mesh.triangles {
                    let point=|index:u32|{let p=mesh.positions[index as usize];Point3::new(p[0] as f64,p[1] as f64,p[2] as f64)};
                    if let RayTriangleResult::Hit(hit)=ray_triangle_intersect_3d(origin,direction,
                        point(triangle[0]),point(triangle[1]),point(triangle[2])) {
                        assert!(hit.t>0.30,"sidewall crosses tyre clearance: {}",model.id);
                    }
                }
            }
        }}
    }
}
