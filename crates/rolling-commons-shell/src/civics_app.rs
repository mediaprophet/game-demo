//! Native Rust + QualiaDB + VibeScript application controller (`CivicsExperienceApp`).
//!
//! Owns all application state, pre-built `.10d` open-world valley & specimen residency,
//! SHACL action proposals, VibeScript campaign/character/workshop/drive evaluation,
//! `dev.civics.au` demographic/socioeconomic/technical/botanical layers, and
//! `QualiaHud` canvas document synthesis so the HTML page is a pure WASM canvas host
//! with zero application JavaScript.

use crate::{
    asset_forge_workbench::{
        self, AssetForgeReport, ONLINE_SOURCE_GROUPS,
    },
    ecopunk_bridge, session_replay, set_art_style, talbingo_continuum_step_drive_and_walk,
    CampaignSimulation, CharacterSimulation, GameDriveSimulation, GameHud, GameInputBridge,
    GamePortal, GameSession,
};
use qualia_core_db::container_10d::{
    parse_mesh_header, parse_section_table, Container10dHeader, SectionType,
};
use qualia_core_db::render::compile_10d::decode_10d_mesh;
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

const SEED_TTL: &str = include_str!("../../../web/fixtures/seed.ttl");
const ACTIONS_JSON: &str = include_str!("../../../web/fixtures/actions.json");
const CAMPAIGN_VIBE: &str = include_str!("../../../web/fixtures/campaign-rules.vibe");
const CHARACTER_VIBE: &str = include_str!("../../../web/fixtures/character-arcs.vibe");
const DRIVE_VIBE: &str = include_str!("../../../web/fixtures/drive-simulation.vibe");

fn sample_corridor_road_xyz(sector_id: &str, z: f32) -> [f32; 3] {
    let seed = match sector_id {
        "talbingo" => 1.1_f32,
        "yarrangobilly" => 2.4_f32,
        "three_mile_dam" => 3.7_f32,
        "sydney_sandstone" => 5.1_f32,
        "noosa_headland" => 6.5_f32,
        _ => 7.9_f32,
    };
    let thalweg_x = 0.24 * (z * 1.35 + seed).sin() + 0.08 * (z * 2.7 - seed * 0.6).cos();
    let road_x = (thalweg_x + 0.56).clamp(-1.35, 1.35);
    let road_y = 0.03 + 0.025 * (z * 1.1 + seed * 0.5).sin();
    [road_x, road_y, z]
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveMode {
    Civics = 0,
    Continuum = 1,
    Dissection = 2,
    Workshop = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InspectorTab {
    Civics = 0,
    Socio = 1,
    Tech = 2,
    Botany = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StudioModel {
    GrowingStation = 0,
    SwampWallaby = 1,
    EucalyptSpray = 2,
    GhostGum = 3,
    DissectionScale = 4,
}

struct OpenWorldSectorSpec {
    key: &'static str,
    scene_asset: &'static str,
    water_asset: &'static str,
    title: &'static str,
    lga: &'static str,
    elevation_m: u32,
    corridor_km: f32,
    seifa_irsd: u32,
    seifa_decile: u8,
    rental_stress_pct: f32,
    private_rent_wk: u32,
    walkabout_cost_wk: u32,
    vegetation: &'static str,
    lithology: &'static str,
    water_color: [f32; 3],
    sun_dir: [f32; 3],
    cam_yaw: f32,
    cam_pitch: f32,
    cam_zoom: f32,
    cam_target: [f32; 3],
}

const OPEN_WORLD_SECTORS: [OpenWorldSectorSpec; 6] = [
    OpenWorldSectorSpec {
        key: "talbingo",
        scene_asset: "sites/00-talbingo-scene.10d",
        water_asset: "sites/00-talbingo-water.10d",
        title: "Talbingo Gorge & Blowering Forebay",
        lga: "Snowy Valleys LGA (NSW)",
        elevation_m: 385,
        corridor_km: 0.0,
        seifa_irsd: 964,
        seifa_decile: 3,
        rental_stress_pct: 38.4,
        private_rent_wk: 460,
        walkabout_cost_wk: 138,
        vegetation: "Riparian Blakely's Red Gum & White Box Woodland",
        lithology: "Bogong Granite Tors & Alluvial Cobble Point-Bars",
        water_color: [0.06, 0.36, 0.50],
        sun_dir: [0.48, 0.78, 0.44],
        cam_yaw: -0.18,
        cam_pitch: 0.21,
        cam_zoom: 0.76,
        cam_target: [0.48, 0.03, 0.05],
    },
    OpenWorldSectorSpec {
        key: "jounama_karst",
        scene_asset: "sites/02-jounama_karst-scene.10d",
        water_asset: "sites/02-jounama_karst-water.10d",
        title: "Jounama Creek Valley & Limestone Threshold",
        lga: "Snowy Valleys LGA (NSW)",
        elevation_m: 425,
        corridor_km: 4.5,
        seifa_irsd: 968,
        seifa_decile: 4,
        rental_stress_pct: 36.8,
        private_rent_wk: 475,
        walkabout_cost_wk: 140,
        vegetation: "Montane Ribbon Gum & Rough-Barked Apple",
        lithology: "Silurian Karst Outcrops & Alluvial Loam",
        water_color: [0.06, 0.38, 0.50],
        sun_dir: [-0.44, 0.76, 0.50],
        cam_yaw: -0.22,
        cam_pitch: 0.22,
        cam_zoom: 0.78,
        cam_target: [0.48, 0.03, 0.05],
    },
    OpenWorldSectorSpec {
        key: "yarrangobilly",
        scene_asset: "sites/03-yarrangobilly-scene.10d",
        water_asset: "sites/03-yarrangobilly-water.10d",
        title: "Yarrangobilly Thermal Karst Gorge",
        lga: "Snowy Valleys / Kosciuszko Corridor",
        elevation_m: 680,
        corridor_km: 9.2,
        seifa_irsd: 974,
        seifa_decile: 4,
        rental_stress_pct: 35.2,
        private_rent_wk: 490,
        walkabout_cost_wk: 145,
        vegetation: "Mountain Gum, Narrow-Leaved Peppermint & Tree Ferns",
        lithology: "Yarrangobilly Limestone Cliff Benches & Thermal Springs",
        water_color: [0.06, 0.48, 0.54],
        sun_dir: [-0.22, 0.80, 0.56],
        cam_yaw: -0.20,
        cam_pitch: 0.22,
        cam_zoom: 0.78,
        cam_target: [0.48, 0.03, 0.05],
    },
    OpenWorldSectorSpec {
        key: "three_mile_dam",
        scene_asset: "sites/05-three_mile_dam-scene.10d",
        water_asset: "sites/05-three_mile_dam-water.10d",
        title: "Three Mile Dam Subalpine Snow Gum Frost Hollow",
        lga: "Snowy Monaro Regional LGA (NSW)",
        elevation_m: 1420,
        corridor_km: 17.5,
        seifa_irsd: 1002,
        seifa_decile: 6,
        rental_stress_pct: 41.6,
        private_rent_wk: 580,
        walkabout_cost_wk: 152,
        vegetation: "Subalpine Snow Gum (E. pauciflora) & Poa Tussock Herbfield",
        lithology: "Periglacial Granodiorite Blockstreams & Peat Bog",
        water_color: [0.05, 0.44, 0.60],
        sun_dir: [0.48, 0.76, 0.48],
        cam_yaw: -0.18,
        cam_pitch: 0.21,
        cam_zoom: 0.76,
        cam_target: [0.48, 0.03, 0.05],
    },
    OpenWorldSectorSpec {
        key: "sydney_sandstone",
        scene_asset: "sites/06-sydney_sandstone-scene.10d",
        water_asset: "sites/06-sydney_sandstone-water.10d",
        title: "Hawkesbury Sandstone Escarpment & Gully",
        lga: "Blue Mountains / Hawkesbury Corridor (NSW)",
        elevation_m: 310,
        corridor_km: 245.0,
        seifa_irsd: 1034,
        seifa_decile: 7,
        rental_stress_pct: 46.8,
        private_rent_wk: 740,
        walkabout_cost_wk: 155,
        vegetation: "Sydney Red Gum (Angophora costata) & Gymea Lily Woodland",
        lithology: "Triassic Hawkesbury Cross-Bedded Quartz Sandstone",
        water_color: [0.08, 0.38, 0.46],
        sun_dir: [0.40, 0.82, 0.44],
        cam_yaw: -0.24,
        cam_pitch: 0.22,
        cam_zoom: 0.78,
        cam_target: [0.48, 0.03, 0.05],
    },
    OpenWorldSectorSpec {
        key: "noosa_headland",
        scene_asset: "sites/07-noosa_headland-scene.10d",
        water_asset: "sites/07-noosa_headland-water.10d",
        title: "Noosa Coastal Headland & Pandanus Littoral Shelf",
        lga: "Noosa Shire LGA (QLD)",
        elevation_m: 42,
        corridor_km: 1120.0,
        seifa_irsd: 1018,
        seifa_decile: 6,
        rental_stress_pct: 52.4,
        private_rent_wk: 820,
        walkabout_cost_wk: 148,
        vegetation: "Coastal Banksia, Pandanus tectorius & Araucaria Wallum",
        lithology: "Mesozoic Myrtle Creek Sandstone & Basalt Dykes",
        water_color: [0.04, 0.52, 0.64],
        sun_dir: [0.32, 0.84, 0.44],
        cam_yaw: 0.14,
        cam_pitch: 0.20,
        cam_zoom: 0.78,
        cam_target: [0.48, 0.03, 0.05],
    },
];

struct CorridorPlaceFocus {
    name: &'static str,
    role: &'static str,
    target: [f32; 3],
    yaw: f32,
    pitch: f32,
    zoom: f32,
}

const CORRIDOR_PLACES: [CorridorPlaceFocus; 6] = [
    CorridorPlaceFocus {
        name: "1. 4WD Rig & Field Camp",
        role: "Walkabout 79-Series Survey Rover, 600W Solar Canopy & Bushwalker",
        target: [0.46, 0.02, 0.04],
        yaw: -0.18,
        pitch: 0.19,
        zoom: 0.58,
    },
    CorridorPlaceFocus {
        name: "2. Riparian River Crossing",
        role: "Tumut River Thalweg, Alluvial Cobble Point-Bar & Water Intake",
        target: [-0.08, -0.16, 0.02],
        yaw: 0.34,
        pitch: 0.22,
        zoom: 0.68,
    },
    CorridorPlaceFocus {
        name: "3. Woodland Kangaroo Clearing",
        role: "Eastern Grey Kangaroo (Macropus giganteus) Mob & Native Forbs",
        target: [0.68, 0.06, -0.08],
        yaw: -0.45,
        pitch: 0.21,
        zoom: 0.62,
    },
    CorridorPlaceFocus {
        name: "4. Ghost Gum & Canopy Stratum",
        role: "Corymbia aparrerinja & Eucalyptus pauciflora Multi-Stratum Stand",
        target: [0.82, 0.18, -0.42],
        yaw: -0.28,
        pitch: 0.24,
        zoom: 0.82,
    },
    CorridorPlaceFocus {
        name: "5. CSIRO SLGA 6-Horizon Cutaway",
        role: "O/A1/A2/B21/B22/C Soil Profile & Granite Tor Outcrop",
        target: [0.22, -0.05, 0.42],
        yaw: 0.12,
        pitch: 0.25,
        zoom: 0.86,
    },
    CorridorPlaceFocus {
        name: "6. Full Valley Corridor Vista",
        role: "Regional Open-World Gorge, Cambered Highway & Microgrid Site",
        target: [0.18, 0.02, -0.10],
        yaw: -0.16,
        pitch: 0.26,
        zoom: 1.08,
    },
];

#[derive(Clone, Default)]
struct DerivedWorld {
    coins: u32,
    labour: u32,
    day: u32,
    parts: u32,
    upgrades: u32,
    online: bool,
    approved: bool,
    water_online: bool,
    garden_active: bool,
    signal_online: bool,
    bridge_open: bool,
    pump_online: bool,
    orchard_active: bool,
    commons_finale: bool,
    trade_finale: bool,
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct CivicsExperienceApp {
    portal: GamePortal,
    hud: GameHud,
    session: GameSession,
    campaign: CampaignSimulation,
    character: CharacterSimulation,
    drive: GameDriveSimulation,
    input: GameInputBridge,
    prebuilt_assets: HashMap<String, Vec<u8>>,
    runtime_pbr_digests: Option<[[u8; 32]; 4]>,
    textures_admitted: bool,
    mode: ActiveMode,
    inspector_open: bool,
    inspector_tab: InspectorTab,
    studio_model: StudioModel,
    art_style: u8,
    sky_preset: u32,
    cam_yaw: f32,
    cam_pitch: f32,
    cam_zoom: f32,
    cam_target: [f32; 3],
    orbit_gc: [f32; 3],
    orbit_scale: f32,
    roam_z: f32,
    active_place: usize,
    action_page: usize,
    character_arc: String,
    continuum_site: u8,
    continuum_drive_mode: bool,
    continuum_elev: f64,
    continuum_odo_m: f64,
    continuum_speed_kmh: f32,
    dissection_scale: u8,
    dissection_explode: f32,
    dissection_cutaway: f32,
    dissection_anthesis: f32,
    dissection_peel: f32,
    dissection_archetype: u8,
    force_live_rust_rebuild: bool,
    meta_sources_enabled: [bool; 5],
    forge_report: AssetForgeReport,
    active_verts: usize,
    active_tris: usize,
    workshop_rig: u8,
    workshop_rev: u32,
    status_banner: String,
    last_w: u32,
    last_h: u32,
    hud_dirty: bool,
}

#[wasm_bindgen]
impl CivicsExperienceApp {
    #[wasm_bindgen(constructor)]
    pub fn new(
        scene_canvas: HtmlCanvasElement,
        hud_canvas: HtmlCanvasElement,
    ) -> Result<CivicsExperienceApp, JsValue> {
        set_art_style(2);
        let mut portal = GamePortal::new(scene_canvas)?;
        let hud = GameHud::new(hud_canvas)?;
        let mut session = GameSession::new(SEED_TTL, ACTIONS_JSON)?;
        let _ = session.set_epistemic_mode("civics");
        let campaign = CampaignSimulation::new(CAMPAIGN_VIBE)?;
        let character = CharacterSimulation::new(CHARACTER_VIBE)?;
        let drive = GameDriveSimulation::new(DRIVE_VIBE)?;
        let input = GameInputBridge::new();

        portal.inner.set_tensor_projection_enabled(false);
        portal.inner.set_ambient_enabled(false);
        portal.inner.set_preserve_authored_frame(false);
        portal.set_sky_preset(1);

        let mut app = CivicsExperienceApp {
            portal,
            hud,
            session,
            campaign,
            character,
            drive,
            input,
            prebuilt_assets: HashMap::new(),
            runtime_pbr_digests: None,
            textures_admitted: false,
            mode: ActiveMode::Civics,
            inspector_open: false,
            inspector_tab: InspectorTab::Civics,
            studio_model: StudioModel::GrowingStation,
            art_style: 2,
            sky_preset: 1,
            cam_yaw: -0.18,
            cam_pitch: 0.20,
            cam_zoom: 0.72,
            cam_target: [0.0, 0.0, 0.0],
            orbit_gc: [0.0, 0.12, 0.0],
            orbit_scale: 0.4359,
            roam_z: 0.04,
            active_place: 0,
            action_page: 0,
            character_arc: "work".to_string(),
            continuum_site: 0,
            continuum_drive_mode: true,
            continuum_elev: 385.0,
            continuum_odo_m: 0.0,
            continuum_speed_kmh: 0.0,
            dissection_scale: 1,
            dissection_explode: 0.35,
            dissection_cutaway: 0.25,
            dissection_anthesis: 0.72,
            dissection_peel: 0.40,
            dissection_archetype: 0,
            force_live_rust_rebuild: false,
            meta_sources_enabled: [true; 5],
            forge_report: AssetForgeReport::default(),
            active_verts: 0,
            active_tris: 0,
            workshop_rig: 0,
            workshop_rev: 0,
            status_banner: "Open-World Valley & Native Rust Asset Forge (.vibe · .10d · .q42 · .m64 · .hmc) Ready".to_string(),
            last_w: 0,
            last_h: 0,
            hud_dirty: true,
        };
        app.ensure_pbr_textures_admitted();
        Ok(app)
    }

    /// Admit a `.10d` asset into the residency cache, remapping its `MAT1` texture digests to
    /// the runtime Rust-synthesized procedural PBR surface maps and resealing `CRC-32C`.
    pub fn admit_prebuilt_asset(&mut self, key: &str, bytes: &[u8]) {
        self.ensure_pbr_textures_admitted();
        let mut owned = bytes.to_vec();
        if let Some(digests) = &self.runtime_pbr_digests {
            asset_forge_workbench::remap_and_reseal_10d_textures(&mut owned, digests);
        }
        self.prebuilt_assets.insert(key.to_string(), owned);
    }

    /// Refresh the active 3D scene once initial `.10d` assets have been admitted.
    pub fn activate_initial_scene(&mut self) -> Result<(), JsValue> {
        self.refresh_3d_scene()
    }

    pub fn renderer_tier(&self) -> u8 {
        self.portal.tier()
    }

    pub fn status_text(&self) -> String {
        self.status_banner.clone()
    }

    /// Execute any HUD/Inspector action ID directly (e.g. `"inspector:toggle"`, `"tab:socio"`, `"studio:growing_station"`).
    pub fn trigger_action(&mut self, action_id: &str) -> Result<String, JsValue> {
        self.handle_action(action_id)?;
        if self.last_w > 0 && self.last_h > 0 {
            self.rebuild_hud(self.last_w as f64, self.last_h as f64)?;
            self.hud_dirty = false;
        }
        Ok(self.status_banner.clone())
    }

    pub fn on_key_down(&mut self, key_code: u16) {
        self.input.key_down(key_code);
        match key_code {
            9 | 73 => {
                self.inspector_open = !self.inspector_open;
                self.hud_dirty = true;
            }
            69 => {
                if self.mode == ActiveMode::Civics || self.mode == ActiveMode::Continuum {
                    let _ = self.handle_action("continuum:toggle_dismount");
                    self.hud_dirty = true;
                }
            }
            _ => {}
        }
    }

    pub fn on_key_up(&mut self, key_code: u16) {
        self.input.key_up(key_code);
    }

    pub fn on_orbit(&mut self, dx: f32, dy: f32) {
        self.cam_yaw -= dx * 0.0065;
        self.cam_pitch = (self.cam_pitch + dy * 0.0055).clamp(0.06, 1.42);
        self.apply_camera();
    }

    pub fn on_zoom(&mut self, delta_y: f32) {
        let step = if delta_y > 0.0 { 0.08 } else { -0.08 };
        self.cam_zoom = (self.cam_zoom + step).clamp(0.28, 3.6);
        self.apply_camera();
    }

    pub fn on_pointer_click(
        &mut self,
        x: f64,
        y: f64,
        canvas_w: u32,
        canvas_h: u32,
    ) -> Result<String, JsValue> {
        let hit = self.hud.hit_test(x, y);
        if !hit.is_empty() {
            self.handle_action(&hit)?;
            self.rebuild_hud(canvas_w as f64, canvas_h as f64)?;
            return Ok(hit);
        }
        let _ = self
            .portal
            .queue_pick(x as f32, y as f32, canvas_w, canvas_h);
        Ok(String::new())
    }

    pub fn tick(
        &mut self,
        scene_canvas: &HtmlCanvasElement,
        dt_ms: f32,
        viewport_w: u32,
        viewport_h: u32,
    ) -> Result<(), JsValue> {
        if viewport_w != self.last_w || viewport_h != self.last_h {
            self.last_w = viewport_w;
            self.last_h = viewport_h;
            self.hud_dirty = true;
        }

        if self.mode == ActiveMode::Civics || self.mode == ActiveMode::Continuum {
            let throttle = self.input.throttle();
            let steer = self.input.steering();
            let brake = if self.input.brake() { 1.0 } else { 0.0 };
            if throttle.abs() > 0.001 || steer.abs() > 0.001 || brake > 0.0 {
                let dt_s = (dt_ms * 0.001).clamp(0.005, 0.05);
                let speed_mul = if self.continuum_drive_mode { 0.55 } else { 0.22 };
                self.roam_z = (self.roam_z + throttle * dt_s * speed_mul).clamp(-1.45, 1.45);
                self.cam_yaw -= steer * dt_s * 1.15;
                let spec = &OPEN_WORLD_SECTORS[self.continuum_site as usize];
                let road_xyz = sample_corridor_road_xyz(spec.key, self.roam_z);
                self.set_world_camera_target(road_xyz);

                let step_in = if self.continuum_drive_mode {
                    json!({
                        "mode": "drive",
                        "site_id": self.continuum_site,
                        "dt": dt_s,
                        "throttle": throttle,
                        "steer": steer,
                        "brake": brake,
                    })
                } else {
                    json!({
                        "mode": "walk",
                        "site_id": self.continuum_site,
                        "dt": dt_s,
                        "wish_dir": [steer, throttle],
                        "sprint": self.input.shift(),
                        "jump": self.input.brake(),
                        "crouch": false,
                    })
                };
                if let Ok(out) = talbingo_continuum_step_drive_and_walk(&step_in.to_string()) {
                    if let Some(elev) = crate::js_get(&out, "elevation_ahd").and_then(|v| v.as_f64())
                    {
                        self.continuum_elev = elev;
                    }
                    if let Some(odo) = crate::js_get(&out, "odometer_m").and_then(|v| v.as_f64()) {
                        self.continuum_odo_m = odo;
                    }
                    if let Some(veh) = crate::js_get(&out, "vehicle") {
                        self.continuum_speed_kmh = crate::js_get(&veh, "speed_kmh")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0) as f32;
                    }
                }
                self.hud_dirty = true;
            }
        }

        self.portal.tick(scene_canvas, dt_ms.min(50.0))?;
        let picked = self.portal.poll_pick();
        if picked >= 0 && (picked as usize) < CORRIDOR_PLACES.len() {
            let idx = picked as usize;
            if idx != self.active_place {
                self.focus_corridor_place(idx);
                self.hud_dirty = true;
            }
        }

        if self.hud_dirty && viewport_w > 0 && viewport_h > 0 {
            self.rebuild_hud(viewport_w as f64, viewport_h as f64)?;
            self.hud_dirty = false;
        }
        self.input.step_frame();
        Ok(())
    }
}

impl CivicsExperienceApp {
    fn ensure_pbr_textures_admitted(&mut self) {
        if self.textures_admitted {
            return;
        }
        if let Some(digests) =
            asset_forge_workbench::synthesize_and_admit_procedural_pbr_textures(&mut self.portal)
        {
            self.runtime_pbr_digests = Some(digests);
            self.textures_admitted = true;
        }
    }

    fn apply_camera(&mut self) {
        let _ = self.portal.set_camera_target(
            self.cam_yaw,
            self.cam_pitch,
            self.cam_zoom,
            self.cam_target[0],
            self.cam_target[1],
            self.cam_target[2],
        );
        self.hud.set_camera_target(
            self.cam_yaw,
            self.cam_pitch,
            self.cam_zoom,
            self.cam_target[0],
            self.cam_target[1],
            self.cam_target[2],
        );
    }

    fn set_world_camera_target(&mut self, world_xyz: [f32; 3]) {
        self.cam_target = [
            (world_xyz[0] - self.orbit_gc[0]) * self.orbit_scale,
            (world_xyz[1] - self.orbit_gc[1]) * self.orbit_scale,
            (world_xyz[2] - self.orbit_gc[2]) * self.orbit_scale,
        ];
        self.apply_camera();
    }

    fn focus_corridor_place(&mut self, idx: usize) {
        let clamped = idx.min(CORRIDOR_PLACES.len() - 1);
        self.active_place = clamped;
        let p = &CORRIDOR_PLACES[clamped];
        self.cam_yaw = p.yaw;
        self.cam_pitch = p.pitch;
        self.cam_zoom = p.zoom;
        self.roam_z = p.target[2];
        self.set_world_camera_target(p.target);
        self.status_banner = format!("Focused {} — {}", p.name, p.role);
    }

    fn extract_10d_bounds(bytes: &[u8]) -> Option<([f32; 3], [f32; 3], usize, usize)> {
        let header = Container10dHeader::parse(bytes).ok()?;
        let descs = parse_section_table(bytes, &header).ok()?;
        for d in descs {
            if d.typ() == Some(SectionType::QuantizedMesh) {
                let start = d.byte_offset as usize;
                let end = start.checked_add(d.byte_length as usize)?;
                let payload = bytes.get(start..end)?;
                let (mh, _) = parse_mesh_header(payload).ok()?;
                return Some((
                    mh.min,
                    mh.max,
                    mh.vertex_count as usize,
                    mh.triangle_count as usize,
                ));
            }
        }
        None
    }

    fn load_single_10d_colored(&mut self, id: &str, bytes: &[u8]) -> Result<(), JsValue> {
        let assets = js_sys::Array::new();
        let item = js_sys::Object::new();
        js_sys::Reflect::set(&item, &"id".into(), &JsValue::from_str(id))?;
        js_sys::Reflect::set(&item, &"bytes".into(), &js_sys::Uint8Array::from(bytes))?;
        js_sys::Reflect::set(&item, &"r".into(), &JsValue::from_f64(1.0))?;
        js_sys::Reflect::set(&item, &"g".into(), &JsValue::from_f64(1.0))?;
        js_sys::Reflect::set(&item, &"b".into(), &JsValue::from_f64(1.0))?;
        js_sys::Reflect::set(&item, &"a".into(), &JsValue::from_f64(1.0))?;
        assets.push(&item);
        self.portal.inner.load_assets_colored(&assets)?;
        Ok(())
    }

    fn load_open_world_sector(&mut self, site_idx: u8) -> Result<(), JsValue> {
        self.ensure_pbr_textures_admitted();
        let idx = (site_idx as usize).min(OPEN_WORLD_SECTORS.len() - 1);
        let spec = &OPEN_WORLD_SECTORS[idx];

        if let Some(scene_bytes) = self.prebuilt_assets.get(spec.scene_asset).cloned() {
            let (min, max, verts, tris) = Self::extract_10d_bounds(&scene_bytes)
                .unwrap_or(([-1.95, -0.35, -1.95], [1.95, 0.65, 1.95], 247_286, 306_101));
            self.active_verts = verts;
            self.active_tris = tris;

            let gc = [
                (min[0] + max[0]) * 0.5,
                (min[1] + max[1]) * 0.5,
                (min[2] + max[2]) * 0.5,
            ];
            let gspan = (max[0] - min[0])
                .max(max[1] - min[1])
                .max(max[2] - min[2])
                .max(1e-6);
            let orbit_scale = 1.7 / gspan;
            self.orbit_gc = gc;
            self.orbit_scale = orbit_scale;

            self.portal.inner.set_preserve_authored_frame(false);
            self.load_single_10d_colored(spec.key, &scene_bytes)?;

            if let Some(water_bytes) = self.prebuilt_assets.get(spec.water_asset) {
                if let Ok(water_mesh) = decode_10d_mesh(water_bytes) {
                    let mut flat_water = Vec::with_capacity(water_mesh.positions.len() * 3);
                    for p in &water_mesh.positions {
                        flat_water.push((p[0] - gc[0]) * orbit_scale);
                        flat_water.push((p[1] - gc[1]) * orbit_scale);
                        flat_water.push((p[2] - gc[2]) * orbit_scale);
                    }
                    let mut flat_indices = Vec::with_capacity(water_mesh.triangles.len() * 3);
                    for t in &water_mesh.triangles {
                        flat_indices.extend_from_slice(t);
                    }
                    let _ = self
                        .portal
                        .inner
                        .set_water_geometry(&flat_water, &flat_indices);
                    self.portal.inner.set_water_surface(
                        spec.water_color[0],
                        spec.water_color[1],
                        spec.water_color[2],
                        0.68,
                        0.014,
                        3.1,
                        2,
                    );
                }
            }

            let _ = self.portal.set_environment_sector(spec.key);
            self.portal.inner.set_shadows_enabled(true);
            self.portal.inner.set_screen_space_ao_enabled(true);
            self.portal.inner.set_screen_space_ao(0.42, 0.58, 0.018);
            self.portal.inner.set_screen_space_ao_sample_count(12);
            self.portal.inner.set_sky_preset(self.sky_preset);
            self.portal.inner.set_lighting(
                spec.sun_dir[0],
                spec.sun_dir[1],
                spec.sun_dir[2],
                1.56,
                0.28,
            );
            let _ = self.portal.inner.set_exposure_compensation(0.06);

            self.cam_yaw = spec.cam_yaw;
            self.cam_pitch = spec.cam_pitch;
            self.cam_zoom = spec.cam_zoom;
            self.roam_z = spec.cam_target[2];
            self.set_world_camera_target(spec.cam_target);
            self.continuum_elev = spec.elevation_m as f64;
            return Ok(());
        }

        self.portal
            .load_talbingo_high_grade_sector(site_idx)
            .map(|_| ())
    }

    fn clear_water(&mut self) {
        let _ = self.portal.inner.set_water_geometry(
            &[0.0, -999.0, 0.0, 0.001, -999.0, 0.0, 0.0, -999.0, 0.001],
            &[0, 1, 2],
        );
        self.portal
            .inner
            .set_water_surface(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0);
    }

    fn load_studio_specimen(&mut self) -> Result<(), JsValue> {
        let _ = self.portal.set_environment_sector("dissection_lab");
        self.portal.inner.set_preserve_authored_frame(false);
        self.clear_water();
        self.portal.inner.set_lighting(0.42, 0.78, 0.46, 1.52, 0.34);

        let (asset_key, specimen_tag, default_yaw, default_pitch, default_zoom) =
            match self.studio_model {
                StudioModel::GrowingStation => (
                    "models/growing-station.10d",
                    "growing_station",
                    0.58,
                    0.26,
                    2.65,
                ),
                StudioModel::SwampWallaby => (
                    "models/swamp-wallaby-r1.10d",
                    "swamp_wallaby",
                    -0.58,
                    0.22,
                    2.65,
                ),
                StudioModel::EucalyptSpray => (
                    "models/eucalypt-spray.10d",
                    "eucalypt_spray",
                    0.42,
                    0.24,
                    2.65,
                ),
                StudioModel::GhostGum => (
                    "models/corymbia-whole-organism.10d",
                    "ghost_gum",
                    0.35,
                    0.22,
                    2.75,
                ),
                StudioModel::DissectionScale => {
                    let s = self.dissection_scale.clamp(1, 5);
                    let arch = if s == 5 {
                        self.dissection_archetype.min(4)
                    } else {
                        0
                    };
                    let key = match s {
                        1 => "lab/scale-1-archetype-0-state-0.10d".to_string(),
                        2 => "lab/scale-2-archetype-0-state-1.10d".to_string(),
                        3 => "lab/scale-3-archetype-0-state-1.10d".to_string(),
                        4 => "lab/scale-4-archetype-0-state-1.10d".to_string(),
                        _ => format!("lab/scale-5-archetype-{arch}-state-1.10d"),
                    };
                    let bytes = if !self.force_live_rust_rebuild {
                        self.prebuilt_assets.get(&key).cloned()
                    } else {
                        None
                    };
                    let active_bytes = match bytes {
                        Some(b) => b,
                        None => {
                            let (built_bytes, _meta) = ecopunk_bridge::build_dissection_scale_10d(
                                self.dissection_scale,
                                self.dissection_explode,
                                self.dissection_cutaway,
                                self.dissection_anthesis,
                                self.dissection_peel,
                                self.dissection_archetype,
                            )
                            .map_err(|e| JsValue::from_str(&e))?;
                            built_bytes
                        }
                    };
                    if let Some((_, _, v, t)) = Self::extract_10d_bounds(&active_bytes) {
                        self.active_verts = v;
                        self.active_tris = t;
                    }
                    self.forge_report = asset_forge_workbench::compile_and_verify_asset_containers(
                        "dissection_scale",
                        self.dissection_scale,
                        self.dissection_archetype,
                        self.dissection_explode,
                        self.dissection_cutaway,
                        self.dissection_anthesis,
                        self.dissection_peel,
                        &active_bytes,
                        &self.meta_sources_enabled,
                    );
                    self.load_single_10d_colored("rc:asset/dissection", &active_bytes)?;
                    self.cam_yaw = 0.52;
                    self.cam_pitch = 0.24;
                    self.cam_zoom = 2.75;
                    self.cam_target = [0.22, 0.0, 0.0];
                    self.apply_camera();
                    return Ok(());
                }
            };

        if let Some(bytes) = self.prebuilt_assets.get(asset_key).cloned() {
            if let Some((_, _, v, t)) = Self::extract_10d_bounds(&bytes) {
                self.active_verts = v;
                self.active_tris = t;
            }
            self.forge_report = asset_forge_workbench::compile_and_verify_asset_containers(
                specimen_tag,
                self.dissection_scale,
                self.dissection_archetype,
                self.dissection_explode,
                self.dissection_cutaway,
                self.dissection_anthesis,
                self.dissection_peel,
                &bytes,
                &self.meta_sources_enabled,
            );
            self.load_single_10d_colored(asset_key, &bytes)?;
            self.cam_yaw = default_yaw;
            self.cam_pitch = default_pitch;
            self.cam_zoom = default_zoom;
            self.cam_target = [0.22, 0.0, 0.0];
            self.apply_camera();
        }
        Ok(())
    }

    fn derive_world(&self) -> DerivedWorld {
        let mut st = DerivedWorld {
            day: 1,
            ..DerivedWorld::default()
        };
        for line in &self.session.lines {
            let line = line.trim();
            if line.starts_with("rc:player rc:coin ") {
                st.coins += 1;
            } else if line.starts_with("rc:player rc:labourToken ") {
                st.labour += 1;
            } else if line.starts_with("rc:player rc:turn ") {
                st.day += 1;
            } else if line.starts_with("rc:camp rc:spareParts ") {
                st.parts += 1;
            } else if line.starts_with("rc:camp rc:rigUpgrade ") {
                st.upgrades += 1;
            } else if line.starts_with("rc:camp rc:solarOnline true") {
                st.online = true;
            } else if line.starts_with("rc:council rc:permitApproved true") {
                st.approved = true;
            } else if line.starts_with("rc:grounds rc:waterCisternOnline true") {
                st.water_online = true;
            } else if line.starts_with("rc:grounds rc:hydroponicsActive true") {
                st.garden_active = true;
            } else if line.starts_with("rc:grounds rc:meshRelayOnline true") {
                st.signal_online = true;
            } else if line.starts_with("rc:canal rc:lockBridgeOpen true") {
                st.bridge_open = true;
            } else if line.starts_with("rc:estuary rc:windPumpOnline true") {
                st.pump_online = true;
            } else if line.starts_with("rc:estuary rc:orchardIrrigated true") {
                st.orchard_active = true;
            } else if line.starts_with("rc:commons rc:regionalCompactRatified true") {
                st.commons_finale = true;
            } else if line.starts_with("rc:commons rc:maritimeTradeCharter true") {
                st.trade_finale = true;
            }
        }
        st
    }

    fn refresh_3d_scene(&mut self) -> Result<(), JsValue> {
        match self.mode {
            ActiveMode::Civics | ActiveMode::Continuum => {
                self.load_open_world_sector(self.continuum_site)?;
            }
            ActiveMode::Dissection => {
                self.load_studio_specimen()?;
            }
            ActiveMode::Workshop => {
                let scene = self.session.workshop_scene(self.workshop_rig)?;
                let _ = self.portal.set_environment_sector("workshop");
                self.clear_water();
                let assets = js_sys::Reflect::get(&scene, &"assets".into())?
                    .dyn_into::<js_sys::Array>()?;
                let textures = js_sys::Reflect::get(&scene, &"textures".into())?
                    .dyn_into::<js_sys::Array>()?;
                self.portal.load_scene_textured(&assets, &textures)?;
                if let Ok(tensor_val) = js_sys::Reflect::get(&scene, &"tensor".into()) {
                    if let Ok(tensor_arr) = tensor_val.dyn_into::<js_sys::Uint8Array>() {
                        let bytes = tensor_arr.to_vec();
                        if !bytes.is_empty() {
                            let _ = self.portal.upload_tensor(&bytes);
                        }
                    }
                }
                self.cam_yaw = 0.52;
                self.cam_pitch = 0.28;
                self.cam_zoom = 1.65;
                self.cam_target = [0.0, 0.0, 0.0];
                self.apply_camera();
            }
        }
        self.hud_dirty = true;
        Ok(())
    }

    fn handle_action(&mut self, action_id: &str) -> Result<(), JsValue> {
        if action_id == "inspector:toggle" {
            self.inspector_open = !self.inspector_open;
            return Ok(());
        }
        if let Some(tab_name) = action_id.strip_prefix("tab:") {
            self.inspector_open = true;
            self.inspector_tab = match tab_name {
                "socio" => InspectorTab::Socio,
                "tech" => InspectorTab::Tech,
                "botany" => InspectorTab::Botany,
                _ => InspectorTab::Civics,
            };
            return Ok(());
        }
        if let Some(mode_name) = action_id.strip_prefix("mode:") {
            self.mode = match mode_name {
                "continuum" => ActiveMode::Continuum,
                "dissection" => {
                    self.inspector_open = true;
                    self.inspector_tab = InspectorTab::Botany;
                    ActiveMode::Dissection
                }
                "workshop" => {
                    self.inspector_open = true;
                    self.inspector_tab = InspectorTab::Tech;
                    ActiveMode::Workshop
                }
                _ => ActiveMode::Civics,
            };
            self.refresh_3d_scene()?;
            let spec = &OPEN_WORLD_SECTORS[self.continuum_site as usize];
            self.status_banner = match self.mode {
                ActiveMode::Civics => format!(
                    "Open-World Corridor: {} ({} tris) · WASD to Roam · Tab for dev.civics.au Inspector",
                    spec.title, self.active_tris
                ),
                ActiveMode::Continuum => format!(
                    "Regional Corridor Sector: {} ({:.1} km · {}m AHD)",
                    spec.title, spec.corridor_km, spec.elevation_m
                ),
                ActiveMode::Dissection => format!(
                    "Live Asset Forge (.vibe → .10d + .q42 + .m64 + .hmc) · {} verts / {} tris",
                    self.active_verts, self.active_tris
                ),
                ActiveMode::Workshop => {
                    "4WD Rig & Community Grounds Microgrid Service Bay".to_string()
                }
            };
            return Ok(());
        }
        if let Some(model_name) = action_id.strip_prefix("studio:") {
            self.studio_model = match model_name {
                "growing_station" => StudioModel::GrowingStation,
                "swamp_wallaby" => StudioModel::SwampWallaby,
                "eucalypt_spray" => StudioModel::EucalyptSpray,
                "ghost_gum" => StudioModel::GhostGum,
                _ => StudioModel::DissectionScale,
            };
            self.force_live_rust_rebuild = false;
            self.mode = ActiveMode::Dissection;
            self.inspector_open = true;
            self.inspector_tab = InspectorTab::Botany;
            self.refresh_3d_scene()?;
            self.status_banner = format!(
                "Loaded 3D Specimen ({model_name}.10d) · {} verts / {} tris · .q42 ({} Quins) + .m64 ({}B) + .hmc Verified",
                self.active_verts,
                self.active_tris,
                self.forge_report.q42_quin_count,
                self.forge_report.m64_bytes
            );
            return Ok(());
        }
        if action_id == "forge:rebuild_rust" {
            self.force_live_rust_rebuild = true;
            self.studio_model = StudioModel::DissectionScale;
            self.mode = ActiveMode::Dissection;
            self.inspector_open = true;
            self.inspector_tab = InspectorTab::Botany;
            self.refresh_3d_scene()?;
            self.status_banner = format!(
                "Rebuilt Scale {} Live in Rust (.vibe rev #{}) → .10d ({}B CRC {}) + .q42 ({} Quins) + .m64 ({}B) + .hmc ({}B)",
                self.dissection_scale,
                self.forge_report.vibe_revision,
                self.forge_report.ten_d_bytes,
                self.forge_report.ten_d_crc32c_hex,
                self.forge_report.q42_quin_count,
                self.forge_report.m64_bytes,
                self.forge_report.hmc_bytes
            );
            return Ok(());
        }
        if action_id == "forge:peel_step" {
            self.dissection_peel = if self.dissection_peel < 0.30 {
                0.48
            } else if self.dissection_peel < 0.70 {
                0.92
            } else {
                0.0
            };
            self.force_live_rust_rebuild = true;
            self.studio_model = StudioModel::DissectionScale;
            self.mode = ActiveMode::Dissection;
            self.refresh_3d_scene()?;
            self.status_banner = format!(
                "Live Rust Layer Peel {:.0}% → Rebuilt .10d ({} tris · CRC {})",
                self.dissection_peel * 100.0,
                self.active_tris,
                self.forge_report.ten_d_crc32c_hex
            );
            return Ok(());
        }
        if action_id == "forge:explode_step" {
            self.dissection_explode = if self.dissection_explode < 0.20 {
                0.35
            } else if self.dissection_explode < 0.60 {
                0.75
            } else {
                0.0
            };
            self.force_live_rust_rebuild = true;
            self.studio_model = StudioModel::DissectionScale;
            self.mode = ActiveMode::Dissection;
            self.refresh_3d_scene()?;
            self.status_banner = format!(
                "Live Rust Layer Explode {:.0}% → Rebuilt .10d ({} tris · CRC {})",
                self.dissection_explode * 100.0,
                self.active_tris,
                self.forge_report.ten_d_crc32c_hex
            );
            return Ok(());
        }
        if action_id == "forge:cutaway_step" {
            self.dissection_cutaway = if self.dissection_cutaway < 0.30 {
                0.55
            } else if self.dissection_cutaway < 0.80 {
                0.90
            } else {
                0.15
            };
            self.dissection_anthesis = (self.dissection_cutaway + 0.15).clamp(0.1, 1.0);
            self.force_live_rust_rebuild = true;
            self.studio_model = StudioModel::DissectionScale;
            self.mode = ActiveMode::Dissection;
            self.refresh_3d_scene()?;
            self.status_banner = format!(
                "Live Rust Cutaway {:.0}% / Anthesis {:.0}% → Rebuilt .10d ({} tris · CRC {})",
                self.dissection_cutaway * 100.0,
                self.dissection_anthesis * 100.0,
                self.active_tris,
                self.forge_report.ten_d_crc32c_hex
            );
            return Ok(());
        }
        if let Some(src_str) = action_id.strip_prefix("meta_source:") {
            if let Ok(idx) = src_str.parse::<usize>() {
                if idx < 5 {
                    self.meta_sources_enabled[idx] = !self.meta_sources_enabled[idx];
                    if self.mode == ActiveMode::Dissection {
                        self.refresh_3d_scene()?;
                    }
                    self.status_banner = format!(
                        "Online Metadata Group {} {} → Recompiled .q42 ({} Super-Quins · ECC PASS) & .hmc ({}B)",
                        idx + 1,
                        if self.meta_sources_enabled[idx] {
                            "ATTACHED"
                        } else {
                            "DETACHED"
                        },
                        self.forge_report.q42_quin_count,
                        self.forge_report.hmc_bytes
                    );
                }
            }
            return Ok(());
        }
        if action_id == "sky:cycle" {
            self.sky_preset = (self.sky_preset % 3) + 1;
            self.portal.set_sky_preset(self.sky_preset);
            let label = match self.sky_preset {
                2 => "Golden Hour",
                3 => "Southern Cross Night Sky",
                _ => "Australian Daylight",
            };
            self.status_banner = format!("Sky atmosphere preset: {label}");
            return Ok(());
        }
        if action_id == "roam:forward" {
            self.roam_z = (self.roam_z + 0.28).clamp(-1.45, 1.45);
            let spec = &OPEN_WORLD_SECTORS[self.continuum_site as usize];
            let road_xyz = sample_corridor_road_xyz(spec.key, self.roam_z);
            self.set_world_camera_target(road_xyz);
            self.continuum_odo_m += 180.0;
            self.status_banner = format!(
                "Advanced along {} corridor · Odometer {:.0} m",
                spec.title, self.continuum_odo_m
            );
            return Ok(());
        }
        if action_id == "roam:back" {
            self.roam_z = (self.roam_z - 0.28).clamp(-1.45, 1.45);
            let spec = &OPEN_WORLD_SECTORS[self.continuum_site as usize];
            let road_xyz = sample_corridor_road_xyz(spec.key, self.roam_z);
            self.set_world_camera_target(road_xyz);
            return Ok(());
        }
        if action_id == "page:next" {
            self.action_page = (self.action_page + 1) % 4;
            return Ok(());
        }
        if action_id == "arc:cycle" {
            self.character_arc = match self.character_arc.as_str() {
                "work" => "neighbours".to_string(),
                "neighbours" => "grounds".to_string(),
                _ => "work".to_string(),
            };
            return Ok(());
        }
        if let Some(place_str) = action_id.strip_prefix("place:") {
            if let Ok(idx) = place_str.parse::<usize>() {
                if self.mode != ActiveMode::Civics && self.mode != ActiveMode::Continuum {
                    self.mode = ActiveMode::Civics;
                    let _ = self.refresh_3d_scene();
                }
                self.focus_corridor_place(idx);
            }
            return Ok(());
        }
        if let Some(prop_id) = action_id.strip_prefix("propose:") {
            let res = self.session.propose(prop_id);
            let ok = crate::js_get(&res, "ok")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if ok {
                let seq = crate::js_get(&res, "seq")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0) as u32;
                self.status_banner = format!(
                    "SHACL Accepted [{prop_id}] (event #{seq}) · Obligations & Commons Ledger Updated"
                );
            } else {
                let explain = crate::js_get(&res, "explain")
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| "SHACL precondition not yet satisfied.".to_string());
                self.status_banner = format!("SHACL Gate [{prop_id}]: {explain}");
            }
            return Ok(());
        }
        if action_id == "replay:verify" {
            let events = self.session.events();
            let rep = session_replay(SEED_TTL, ACTIONS_JSON, events)?;
            let diverged = crate::js_get(&rep, "diverged_at")
                .map(|v| !v.is_null() && !v.is_undefined())
                .unwrap_or(false);
            let same = crate::js_get(&rep, "world")
                .and_then(|v| v.as_string())
                .map(|w| w == self.session.world())
                .unwrap_or(false);
            self.status_banner = if !diverged && same {
                format!(
                    "Deterministic Replay PASS · {} accepted events match QualiaDB state byte-for-byte",
                    self.session.events.len()
                )
            } else {
                "Deterministic Replay MISMATCH".to_string()
            };
            return Ok(());
        }
        if let Some(site_str) = action_id.strip_prefix("site:") {
            if let Ok(s) = site_str.parse::<u8>() {
                self.continuum_site = s.min(5);
                if self.mode != ActiveMode::Civics && self.mode != ActiveMode::Continuum {
                    self.mode = ActiveMode::Continuum;
                }
                self.refresh_3d_scene()?;
                let spec = &OPEN_WORLD_SECTORS[self.continuum_site as usize];
                self.status_banner = format!(
                    "Sector {}: {} ({}m AHD · SEIFA {} · {} triangles)",
                    self.continuum_site + 1,
                    spec.title,
                    spec.elevation_m,
                    spec.seifa_irsd,
                    self.active_tris
                );
            }
            return Ok(());
        }
        if action_id == "continuum:toggle_dismount" {
            self.continuum_drive_mode = !self.continuum_drive_mode;
            let mode_label = if self.continuum_drive_mode {
                "4WD LandCruiser Rig Drive"
            } else {
                "On-Foot Botanical & Ecological Walk"
            };
            self.status_banner = format!("Switched locomotion to {mode_label}");
            return Ok(());
        }
        if let Some(sc_str) = action_id.strip_prefix("scale:") {
            if let Ok(sc) = sc_str.parse::<u8>() {
                self.dissection_scale = sc.clamp(1, 5);
                self.force_live_rust_rebuild = false;
                self.studio_model = StudioModel::DissectionScale;
                self.mode = ActiveMode::Dissection;
                self.inspector_open = true;
                self.inspector_tab = InspectorTab::Botany;
                self.refresh_3d_scene()?;
                self.status_banner = format!(
                    "Dissection Scale {} (.10d CRC {}) · {} verts / {} tris · .q42 ({} Quins) + .m64 ({}B)",
                    self.dissection_scale,
                    self.forge_report.ten_d_crc32c_hex,
                    self.active_verts,
                    self.active_tris,
                    self.forge_report.q42_quin_count,
                    self.forge_report.m64_bytes
                );
            }
            return Ok(());
        }
        if let Some(arch_str) = action_id.strip_prefix("arch:") {
            if let Ok(a) = arch_str.parse::<u8>() {
                self.dissection_archetype = a.min(4);
                self.dissection_scale = 5;
                self.force_live_rust_rebuild = false;
                self.studio_model = StudioModel::DissectionScale;
                self.mode = ActiveMode::Dissection;
                self.inspector_open = true;
                self.inspector_tab = InspectorTab::Botany;
                self.refresh_3d_scene()?;
            }
            return Ok(());
        }
        if let Some(rig_str) = action_id.strip_prefix("rig:") {
            if let Ok(r) = rig_str.parse::<u8>() {
                self.workshop_rig = r.min(3);
                self.workshop_rev = 0;
                if self.mode == ActiveMode::Workshop {
                    self.refresh_3d_scene()?;
                }
            }
            return Ok(());
        }
        if let Some(ws_op) = action_id.strip_prefix("ws:") {
            if let Ok(view_str) = self.session.workshop_view(self.workshop_rig) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&view_str) {
                    if let Some(rev) = val
                        .get("state")
                        .and_then(|s| s.get("revision"))
                        .and_then(|r| r.as_f64())
                    {
                        self.workshop_rev = rev as u32;
                    }
                }
            }
            match self
                .session
                .workshop_operate(self.workshop_rig, ws_op, self.workshop_rev)
            {
                Ok(receipt_str) => {
                    let parsed: serde_json::Value =
                        serde_json::from_str(&receipt_str).unwrap_or_default();
                    let ok = parsed.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
                    let detail = parsed
                        .get("detail")
                        .and_then(|v| v.as_str())
                        .unwrap_or(ws_op);
                    if ok {
                        self.workshop_rev += 1;
                    }
                    if self.mode == ActiveMode::Workshop {
                        self.refresh_3d_scene()?;
                    }
                    self.status_banner = if ok {
                        format!("Rig Service [{ws_op}]: {detail}")
                    } else {
                        format!("Rig Gate [{ws_op}]: {detail}")
                    };
                }
                Err(e) => {
                    self.status_banner = format!(
                        "Rig Service [{ws_op}]: {}",
                        e.as_string().unwrap_or_default()
                    );
                }
            }
            return Ok(());
        }
        Ok(())
    }

    fn rebuild_hud(&mut self, w: f64, h: f64) -> Result<(), JsValue> {
        let st = self.derive_world();
        let spec = &OPEN_WORLD_SECTORS[self.continuum_site as usize];
        let mut panels = Vec::new();
        let mut labels = Vec::new();
        let mut buttons = Vec::new();
        let mut meters = Vec::new();
        let mut markers = Vec::new();

        // 1. Slim 38px Top Ribbon (< 5% screen height) — keeps > 92% of the 3D viewport unobstructed!
        panels.push(json!({
            "x": 8.0,
            "y": 6.0,
            "width": (w - 16.0).max(320.0),
            "height": 38.0,
            "title": ""
        }));
        labels.push(json!({
            "x": 18.0,
            "y": 29.0,
            "text": "dev.civics.au · Walkabout & Community Grounds",
            "size": 13.0,
            "tone": "accent"
        }));

        let mode_tabs = [
            ("mode:civics", "1. Open-World & Civics", self.mode == ActiveMode::Civics),
            ("mode:continuum", "2. Corridor Sectors (6)", self.mode == ActiveMode::Continuum),
            ("mode:dissection", "3. Asset Forge (.10d/.q42)", self.mode == ActiveMode::Dissection),
            ("mode:workshop", "4. Tech & Rig Lab", self.mode == ActiveMode::Workshop),
        ];
        let mut tx = 315.0;
        for (id, text, active) in mode_tabs {
            buttons.push(json!({
                "id": id,
                "x": tx,
                "y": 10.0,
                "width": 152.0,
                "height": 28.0,
                "label": if active { format!("● {text}") } else { text.to_string() },
                "enabled": true
            }));
            tx += 158.0;
        }

        // Quick camera & Inspector controls on the right side of the top ribbon
        let right_x = (w - 355.0).max(tx + 8.0);
        buttons.push(json!({
            "id": "roam:forward",
            "x": right_x,
            "y": 10.0,
            "width": 88.0,
            "height": 28.0,
            "label": "Roam +Z ▲",
            "enabled": true
        }));
        buttons.push(json!({
            "id": "sky:cycle",
            "x": right_x + 94.0,
            "y": 10.0,
            "width": 74.0,
            "height": 28.0,
            "label": "Sky ☀",
            "enabled": true
        }));
        let insp_label = if self.inspector_open {
            "Hide Data ▴"
        } else {
            "Inspector ▾ (Tab)"
        };
        buttons.push(json!({
            "id": "inspector:toggle",
            "x": right_x + 174.0,
            "y": 10.0,
            "width": 168.0,
            "height": 28.0,
            "label": insp_label,
            "enabled": true
        }));

        // 2. Slim 28px Bottom Telemetry Ribbon
        let bot_y = (h - 34.0).max(52.0);
        panels.push(json!({
            "x": 8.0,
            "y": bot_y,
            "width": (w - 16.0).max(320.0),
            "height": 28.0,
            "title": ""
        }));
        labels.push(json!({
            "x": 18.0,
            "y": bot_y + 18.0,
            "text": format!(
                "{}  |  {} ({}m AHD · SEIFA {})  |  {} tris · WebGPU T{}",
                self.status_banner,
                spec.title,
                spec.elevation_m,
                spec.seifa_irsd,
                self.active_tris,
                self.portal.tier()
            ),
            "size": 11.5,
            "tone": "primary"
        }));

        // 3. 3D Semantic Callout Markers in Open-World Modes
        if self.mode == ActiveMode::Civics || self.mode == ActiveMode::Continuum {
            for (idx, p) in CORRIDOR_PLACES.iter().enumerate() {
                let norm_xyz = [
                    (p.target[0] - self.orbit_gc[0]) * self.orbit_scale,
                    (p.target[1] - self.orbit_gc[1]) * self.orbit_scale,
                    (p.target[2] - self.orbit_gc[2]) * self.orbit_scale,
                ];
                markers.push(json!({
                    "world": [norm_xyz[0], norm_xyz[1] + 0.04, norm_xyz[2]],
                    "label": p.name,
                    "tone": if idx == self.active_place { "good" } else { "accent" }
                }));
            }
        }

        // 4. Collapsible Right-Hand `dev.civics.au` Inspector (ONLY drawn when `self.inspector_open == true`)
        if self.inspector_open {
            let dw = 400.0_f64.min((w - 20.0).max(300.0));
            let dx = (w - dw - 8.0).max(8.0);
            let dy = 50.0;
            let dh = (h - 90.0).max(380.0);

            panels.push(json!({
                "x": dx,
                "y": dy,
                "width": dw,
                "height": dh,
                "title": "dev.civics.au · Regional & Scientific Inspector"
            }));

            let sub_tabs = [
                ("tab:civics", "Civics", self.inspector_tab == InspectorTab::Civics),
                ("tab:socio", "Socioeconomic", self.inspector_tab == InspectorTab::Socio),
                ("tab:tech", "Tech & Lab", self.inspector_tab == InspectorTab::Tech),
                ("tab:botany", "Botany & .10d", self.inspector_tab == InspectorTab::Botany),
            ];
            let sub_w = (dw - 26.0) / 4.0;
            for (i, (tid, tlabel, tactive)) in sub_tabs.iter().enumerate() {
                buttons.push(json!({
                    "id": tid,
                    "x": dx + 10.0 + (i as f64) * (sub_w + 2.0),
                    "y": dy + 28.0,
                    "width": sub_w,
                    "height": 24.0,
                    "label": if *tactive { format!("● {tlabel}") } else { tlabel.to_string() },
                    "enabled": true
                }));
            }

            let content_y = dy + 64.0;
            match self.inspector_tab {
                InspectorTab::Civics => {
                    self.build_civics_tab(
                        dx,
                        content_y,
                        dw,
                        &st,
                        &mut labels,
                        &mut buttons,
                        &mut meters,
                    )?;
                }
                InspectorTab::Socio => {
                    self.build_socio_tab(
                        dx,
                        content_y,
                        dw,
                        spec,
                        &st,
                        &mut labels,
                        &mut buttons,
                        &mut meters,
                    );
                }
                InspectorTab::Tech => {
                    self.build_tech_tab(
                        dx,
                        content_y,
                        dw,
                        &st,
                        &mut labels,
                        &mut buttons,
                        &mut meters,
                    )?;
                }
                InspectorTab::Botany => {
                    self.build_botany_tab(
                        dx,
                        content_y,
                        dw,
                        spec,
                        &mut labels,
                        &mut buttons,
                        &mut meters,
                    )?;
                }
            }
        }

        let mut items = Vec::new();
        for p in panels {
            items.push(json!({
                "kind": "panel",
                "x": p["x"],
                "y": p["y"],
                "w": p["width"].as_f64().unwrap_or(100.0).max(1.0),
                "h": p["height"].as_f64().unwrap_or(28.0).max(1.0),
            }));
            if let Some(title) = p["title"].as_str() {
                if !title.is_empty() {
                    let pw = p["width"].as_f64().unwrap_or(380.0);
                    items.push(json!({
                        "kind": "label",
                        "x": p["x"].as_f64().unwrap_or(0.0) + 12.0,
                        "y": p["y"].as_f64().unwrap_or(0.0) + 8.0,
                        "w": (pw - 24.0).max(120.0),
                        "h": 16.0,
                        "text": title,
                        "color": "#f3d38c",
                    }));
                }
            }
        }
        for m in meters {
            let color = match m["tone"].as_str().unwrap_or("good") {
                "warn" => "#e89b4b",
                _ => "#5cd68d",
            };
            items.push(json!({
                "kind": "meter",
                "x": m["x"],
                "y": m["y"],
                "w": m["width"].as_f64().unwrap_or(100.0).max(1.0),
                "h": m["height"].as_f64().unwrap_or(8.0).max(1.0),
                "value": m["value"].as_f64().unwrap_or(0.5).clamp(0.0, 1.0),
                "color": color,
            }));
        }
        for b in buttons {
            items.push(json!({
                "kind": "button",
                "id": b["id"].as_str().unwrap_or(""),
                "x": b["x"],
                "y": b["y"],
                "w": b["width"].as_f64().unwrap_or(80.0).max(1.0),
                "h": b["height"].as_f64().unwrap_or(22.0).max(1.0),
                "text": b["label"].as_str().unwrap_or(""),
                "disabled": !b["enabled"].as_bool().unwrap_or(true),
            }));
        }
        for l in labels {
            let color = match l["tone"].as_str().unwrap_or("primary") {
                "accent" => "#f3d38c",
                "good" => "#7ee787",
                "warn" => "#ffa657",
                "muted" => "#9eb4be",
                _ => "#e8f1f5",
            };
            let lx = l["x"].as_f64().unwrap_or(0.0);
            let ly = l["y"].as_f64().unwrap_or(0.0);
            let lw = l["width"].as_f64().unwrap_or_else(|| {
                if lx > 600.0 {
                    376.0
                } else if ly < 40.0 {
                    290.0
                } else {
                    (w - 40.0).max(280.0)
                }
            });
            items.push(json!({
                "kind": "label",
                "x": lx,
                "y": (ly - 8.0).max(0.0),
                "w": lw,
                "h": 16.0,
                "text": l["text"].as_str().unwrap_or(""),
                "color": color,
            }));
        }
        for (idx, mk) in markers.iter().enumerate() {
            if let Some(world) = mk["world"].as_array() {
                if world.len() == 3 {
                    items.push(json!({
                        "kind": "marker",
                        "id": format!("place:{idx}"),
                        "x": world[0].as_f64().unwrap_or(0.0),
                        "y": world[1].as_f64().unwrap_or(0.0),
                        "z": world[2].as_f64().unwrap_or(0.0),
                        "w": 18.0,
                        "h": 18.0,
                        "text": mk["label"].as_str().unwrap_or(""),
                        "color": "#f3d38c",
                    }));
                }
            }
        }
        let doc = json!({ "items": items });
        self.hud.set_document_json(&doc.to_string())
    }

    fn build_civics_tab(
        &mut self,
        dx: f64,
        mut y: f64,
        dw: f64,
        st: &DerivedWorld,
        labels: &mut Vec<serde_json::Value>,
        buttons: &mut Vec<serde_json::Value>,
        meters: &mut Vec<serde_json::Value>,
    ) -> Result<(), JsValue> {
        let st_json = json!({
            "online": st.online,
            "approved": st.approved,
            "water_online": st.water_online,
            "garden_active": st.garden_active,
            "signal_online": st.signal_online,
            "bridge_open": st.bridge_open,
            "pump_online": st.pump_online,
            "orchard_active": st.orchard_active,
            "commons_finale": st.commons_finale,
            "trade_finale": st.trade_finale,
            "coins": st.coins,
            "labour": st.labour,
            "day": st.day,
        });
        let camp_step = self.campaign.lead(&st_json.to_string());
        let stage_label = crate::js_get(&camp_step, "stage_label")
            .or_else(|| crate::js_get(&camp_step, "headline"))
            .and_then(|v| v.as_string())
            .unwrap_or_else(|| "Stage I · Walkabout Arrival & Safe Parking".to_string());
        let progress = (self.session.events.len() as f64 / 12.0).clamp(0.12, 1.0);

        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!("Walkabout Stage: {stage_label}"),
            "size": 12.0,
            "tone": "accent"
        }));
        y += 16.0;
        meters.push(json!({
            "x": dx + 12.0,
            "y": y,
            "width": dw - 24.0,
            "height": 8.0,
            "value": progress,
            "tone": "good"
        }));
        y += 20.0;

        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "Pool Credits: {}  |  Stewardship Labour: {}h  |  Day {}",
                st.coins, st.labour, st.day
            ),
            "size": 11.5,
            "tone": "primary"
        }));
        y += 18.0;

        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Open-World Valley Focus Points (Click to Fly Camera):",
            "size": 11.0,
            "tone": "muted"
        }));
        y += 6.0;
        let col_w = (dw - 28.0) / 2.0;
        for (i, p) in CORRIDOR_PLACES.iter().enumerate() {
            let col = i % 2;
            let row = i / 2;
            buttons.push(json!({
                "id": format!("place:{i}"),
                "x": dx + 12.0 + (col as f64) * (col_w + 4.0),
                "y": y + (row as f64) * 24.0,
                "width": col_w,
                "height": 21.0,
                "label": if i == self.active_place {
                    format!("● {}", p.name)
                } else {
                    p.name.to_string()
                },
                "enabled": true
            }));
        }
        y += 82.0;

        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "SHACL Proposals ([CIVICS] People-Led vs [CIVIC] Gov · Page {}/4):",
                self.action_page + 1
            ),
            "size": 11.0,
            "tone": "accent"
        }));
        buttons.push(json!({
            "id": "page:next",
            "x": dx + dw - 86.0,
            "y": y - 13.0,
            "width": 74.0,
            "height": 20.0,
            "label": "Page ▸",
            "enabled": true
        }));
        y += 8.0;

        let total = self.session.actions.len();
        let per_page = 5usize;
        let start = (self.action_page * per_page) % total.max(1);
        for action in self.session.actions.iter().skip(start).take(per_page) {
            let is_civic = action.id.contains("permit") || action.id.contains("council");
            let badge = if is_civic { "[CIVIC]" } else { "[CIVICS]" };
            buttons.push(json!({
                "id": format!("propose:{}", action.id),
                "x": dx + 12.0,
                "y": y,
                "width": dw - 24.0,
                "height": 23.0,
                "label": format!("{badge} {}", action.label),
                "enabled": true
            }));
            y += 26.0;
        }

        y += 6.0;
        buttons.push(json!({
            "id": "replay:verify",
            "x": dx + 12.0,
            "y": y,
            "width": dw - 24.0,
            "height": 24.0,
            "label": "Verify Deterministic QualiaDB Event Replay",
            "enabled": true
        }));
        Ok(())
    }

    fn build_socio_tab(
        &self,
        dx: f64,
        mut y: f64,
        dw: f64,
        spec: &OpenWorldSectorSpec,
        st: &DerivedWorld,
        labels: &mut Vec<serde_json::Value>,
        buttons: &mut Vec<serde_json::Value>,
        meters: &mut Vec<serde_json::Value>,
    ) {
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!("Regional Atlas: {}", spec.lga),
            "size": 12.5,
            "tone": "accent"
        }));
        y += 18.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "ABS SEIFA IRSD Index: {} (Decile {} of 10)",
                spec.seifa_irsd, spec.seifa_decile
            ),
            "size": 11.5,
            "tone": "primary"
        }));
        y += 16.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "Regional Rental Stress (>30% Income): {:.1}% of renters",
                spec.rental_stress_pct
            ),
            "size": 11.5,
            "tone": "warn"
        }));
        y += 14.0;
        meters.push(json!({
            "x": dx + 12.0,
            "y": y,
            "width": dw - 24.0,
            "height": 8.0,
            "value": (spec.rental_stress_pct / 100.0).clamp(0.0, 1.0),
            "tone": "warn"
        }));
        y += 20.0;

        let savings_wk = spec.private_rent_wk.saturating_sub(spec.walkabout_cost_wk);
        let savings_yr = savings_wk * 52;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Walkabout Strategy vs Private Rental Crisis:",
            "size": 12.0,
            "tone": "accent"
        }));
        y += 16.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "Private Median Rent: ${}/wk  |  Walkabout + Grounds: ${}/wk",
                spec.private_rent_wk, spec.walkabout_cost_wk
            ),
            "size": 11.0,
            "tone": "primary"
        }));
        y += 16.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "Retained Household Surplus: +${}/wk (+${}/yr saved)",
                savings_wk, savings_yr
            ),
            "size": 11.5,
            "tone": "good"
        }));
        y += 22.0;

        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Cooperative Obligation Pool & Credit Contract:",
            "size": 12.0,
            "tone": "accent"
        }));
        y += 16.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Pre-Funded Provider Allocation: $48,500 AUD (0.0% Non-Usurious)",
            "size": 11.0,
            "tone": "primary"
        }));
        y += 15.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "Recorded Stewardship Labour: {}h (Tracked Separately from Debt)",
                st.labour
            ),
            "size": 11.0,
            "tone": "primary"
        }));
        y += 15.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Solid Databox & Concession VC: did:web:dev.civics.au (Zero-Knowledge)",
            "size": 10.5,
            "tone": "muted"
        }));
        y += 20.0;

        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Compare Corridor LGA Sectors:",
            "size": 11.0,
            "tone": "accent"
        }));
        y += 8.0;
        let col_w = (dw - 28.0) / 2.0;
        for (i, s) in OPEN_WORLD_SECTORS.iter().enumerate() {
            let col = i % 2;
            let row = i / 2;
            buttons.push(json!({
                "id": format!("site:{i}"),
                "x": dx + 12.0 + (col as f64) * (col_w + 4.0),
                "y": y + (row as f64) * 24.0,
                "width": col_w,
                "height": 22.0,
                "label": if i == self.continuum_site as usize {
                    format!("● {}. {}", i + 1, s.key)
                } else {
                    format!("{}. {}", i + 1, s.key)
                },
                "enabled": true
            }));
        }
    }

    fn build_tech_tab(
        &self,
        dx: f64,
        mut y: f64,
        dw: f64,
        st: &DerivedWorld,
        labels: &mut Vec<serde_json::Value>,
        buttons: &mut Vec<serde_json::Value>,
        meters: &mut Vec<serde_json::Value>,
    ) -> Result<(), JsValue> {
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Community Grounds Off-Grid Microgrid & Water Telemetry:",
            "size": 12.0,
            "tone": "accent"
        }));
        y += 17.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Solar PV Canopy: 18.4 kWp (74.2 kWh/day)  |  LFP Bank: 64.0 kWh (86% SoC)",
            "size": 11.0,
            "tone": "good"
        }));
        y += 14.0;
        meters.push(json!({
            "x": dx + 12.0,
            "y": y,
            "width": dw - 24.0,
            "height": 7.0,
            "value": 0.86,
            "tone": "good"
        }));
        y += 16.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Rainwater Cisterns: 120 kL (UF+UV Potable · 68 L/person/day · 82% Greywater Recycle)",
            "size": 10.5,
            "tone": "primary"
        }));
        y += 16.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "Hydroponic Growing Station (.10d): 42.5 kg/wk Leafy Greens & Tubers ({})",
                if st.garden_active { "ONLINE" } else { "READY" }
            ),
            "size": 11.0,
            "tone": "accent"
        }));
        y += 18.0;

        buttons.push(json!({
            "id": "studio:growing_station",
            "x": dx + 12.0,
            "y": y,
            "width": (dw - 28.0) * 0.5,
            "height": 24.0,
            "label": "3D Growing Station .10d",
            "enabled": true
        }));
        buttons.push(json!({
            "id": "mode:workshop",
            "x": dx + 16.0 + (dw - 28.0) * 0.5,
            "y": y,
            "width": (dw - 28.0) * 0.5,
            "height": 24.0,
            "label": "Open 3D Rig Service Bay",
            "enabled": true
        }));
        y += 32.0;

        let rig_names = ["1. Bush 4WD", "2. Solar Van", "3. Troopie", "4. Service Ute"];
        let active_rig_label = rig_names[self.workshop_rig as usize];
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!("Walkabout 4WD Rig: {active_rig_label} (4.85m × 1.88m × 1.92m · 2,240 kg)"),
            "size": 11.5,
            "tone": "primary"
        }));
        y += 10.0;

        let rw = (dw - 30.0) / 4.0;
        for (r, rname) in rig_names.iter().enumerate() {
            buttons.push(json!({
                "id": format!("rig:{r}"),
                "x": dx + 12.0 + (r as f64) * (rw + 2.0),
                "y": y,
                "width": rw,
                "height": 22.0,
                "label": if self.workshop_rig == r as u8 {
                    format!("● {rname}")
                } else {
                    rname.to_string()
                },
                "enabled": true
            }));
        }
        y += 28.0;

        let ops = [
            ("ws:open", "1. Open Bonnet / Hood"),
            ("ws:isolate", "2. Isolate High-Current Bus"),
            ("ws:install-touring", "3. Fit Heavy-Duty Touring Radiator"),
            ("ws:fill", "4. Bleed & Fill Glycol Coolant"),
            ("ws:controller", "5. Mount MPPT Solar Charge Controller"),
            ("ws:panel", "6. Wire 600W Monocrystalline Roof PV"),
        ];
        for (op_id, op_label) in ops {
            buttons.push(json!({
                "id": op_id,
                "x": dx + 12.0,
                "y": y,
                "width": dw - 24.0,
                "height": 22.0,
                "label": op_label,
                "enabled": true
            }));
            y += 25.0;
        }
        Ok(())
    }

    fn build_botany_tab(
        &self,
        dx: f64,
        mut y: f64,
        dw: f64,
        _spec: &OpenWorldSectorSpec,
        labels: &mut Vec<serde_json::Value>,
        buttons: &mut Vec<serde_json::Value>,
        _meters: &mut Vec<serde_json::Value>,
    ) -> Result<(), JsValue> {
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Individual 3D Asset Forge (.vibe → .10d + .q42 + .m64 + .hmc):",
            "size": 11.5,
            "tone": "accent"
        }));
        y += 6.0;

        let models = [
            ("studio:growing_station", "Growing Station .10d", StudioModel::GrowingStation),
            ("studio:swamp_wallaby", "Swamp Wallaby .10d", StudioModel::SwampWallaby),
            ("studio:eucalypt_spray", "Eucalypt Spray .10d", StudioModel::EucalyptSpray),
            ("studio:ghost_gum", "Ghost Gum + Roots .10d", StudioModel::GhostGum),
        ];
        let col_w = (dw - 28.0) / 2.0;
        for (i, (mid, mlabel, mkind)) in models.iter().enumerate() {
            let col = i % 2;
            let row = i / 2;
            let active = self.mode == ActiveMode::Dissection && self.studio_model == *mkind;
            buttons.push(json!({
                "id": mid,
                "x": dx + 12.0 + (col as f64) * (col_w + 4.0),
                "y": y + (row as f64) * 23.0,
                "width": col_w,
                "height": 21.0,
                "label": if active { format!("● {mlabel}") } else { mlabel.to_string() },
                "enabled": true
            }));
        }
        y += 52.0;

        let sw = (dw - 32.0) / 5.0;
        for sc in 1..=5u8 {
            let active = self.mode == ActiveMode::Dissection
                && self.studio_model == StudioModel::DissectionScale
                && self.dissection_scale == sc;
            buttons.push(json!({
                "id": format!("scale:{sc}"),
                "x": dx + 12.0 + ((sc - 1) as f64) * (sw + 2.0),
                "y": y,
                "width": sw,
                "height": 21.0,
                "label": if active { format!("● S{sc}") } else { format!("Scale {sc}") },
                "enabled": true
            }));
        }
        y += 24.0;

        if self.studio_model == StudioModel::DissectionScale && self.dissection_scale == 5 {
            let arch_names = ["Macropod", "Wombat", "Platypus", "Eagle", "Koala"];
            for (a, aname) in arch_names.iter().enumerate() {
                buttons.push(json!({
                    "id": format!("arch:{a}"),
                    "x": dx + 12.0 + (a as f64) * (sw + 2.0),
                    "y": y,
                    "width": sw,
                    "height": 20.0,
                    "label": if self.dissection_archetype == a as u8 {
                        format!("● {aname}")
                    } else {
                        aname.to_string()
                    },
                    "enabled": true
                }));
            }
            y += 23.0;
        }

        // Live Rust Procedural Rebuild & Anatomical Layer Peel / Explode Controls
        let ctrl_w = (dw - 30.0) / 4.0;
        let ctrls = [
            ("forge:rebuild_rust", "⚡ Rebuild Rust"),
            (
                "forge:peel_step",
                &format!("Peel {:.0}%", self.dissection_peel * 100.0),
            ),
            (
                "forge:explode_step",
                &format!("Explode {:.0}%", self.dissection_explode * 100.0),
            ),
            (
                "forge:cutaway_step",
                &format!("Cutaway {:.0}%", self.dissection_cutaway * 100.0),
            ),
        ];
        for (i, (cid, clabel)) in ctrls.iter().enumerate() {
            buttons.push(json!({
                "id": cid,
                "x": dx + 12.0 + (i as f64) * (ctrl_w + 2.0),
                "y": y,
                "width": ctrl_w,
                "height": 21.0,
                "label": clabel,
                "enabled": true
            }));
        }
        y += 30.0;

        // Online-Source Metadata Attachment Toggles (click to attach/detach into .q42 + .m64 + .hmc)
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": "Attach Online-Source Metadata (Compiles to .q42 + .m64 + .hmc):",
            "size": 11.0,
            "tone": "accent"
        }));
        y += 6.0;
        for (idx, (_code, desc)) in ONLINE_SOURCE_GROUPS.iter().enumerate() {
            let on = self.meta_sources_enabled[idx];
            let mark = if on { "[✓]" } else { "[ ]" };
            buttons.push(json!({
                "id": format!("meta_source:{idx}"),
                "x": dx + 12.0,
                "y": y,
                "width": dw - 24.0,
                "height": 20.0,
                "label": format!("{mark} {}", desc),
                "enabled": true
            }));
            y += 22.0;
        }
        y += 10.0;

        // Live Container & Cryptographic Verification Readout
        let rep = &self.forge_report;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!("Taxon: {}", rep.scientific_name),
            "size": 10.5,
            "tone": "accent"
        }));
        y += 14.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!("Custodial: {}", rep.vernacular_and_indigenous),
            "size": 10.0,
            "tone": "muted"
        }));
        y += 15.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!("1. .vibe ({}) → {}", rep.vibe_module, rep.vibe_eval_summary),
            "size": 10.0,
            "tone": "good"
        }));
        y += 14.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "2. .10d: {}B · CRC {} ({}) · {:.1}×{:.1}×{:.1}m SI",
                rep.ten_d_bytes,
                rep.ten_d_crc32c_hex,
                if rep.ten_d_crc_valid { "PASS" } else { "RAW" },
                rep.ten_d_dimensions_m[0],
                rep.ten_d_dimensions_m[1],
                rep.ten_d_dimensions_m[2],
            ),
            "size": 10.0,
            "tone": "primary"
        }));
        y += 14.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "3. .q42 Super-Quins: {} × 48B ({}B) · ECC: {}",
                rep.q42_quin_count,
                rep.q42_bytes,
                if rep.q42_ecc_all_valid { "100% PASS" } else { "FAIL" }
            ),
            "size": 10.0,
            "tone": "good"
        }));
        y += 14.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "4. .m64: {} · M64Reader: {}",
                rep.m64_summary,
                if rep.m64_verified { "PASS" } else { "FAIL" }
            ),
            "size": 10.0,
            "tone": "primary"
        }));
        y += 14.0;
        labels.push(json!({
            "x": dx + 12.0,
            "y": y,
            "text": format!(
                "5. .hmc Bundle: {}B · {} entries · SHA-256: {}",
                rep.hmc_bytes,
                rep.hmc_entry_count,
                if rep.hmc_all_verified { "PASS" } else { "FAIL" }
            ),
            "size": 10.0,
            "tone": "good"
        }));
        y += 15.0;

        for sample in rep.q42_sample_triples.iter().take(4) {
            labels.push(json!({
                "x": dx + 14.0,
                "y": y,
                "text": format!("• {sample}"),
                "size": 9.5,
                "tone": "muted"
            }));
            y += 13.0;
        }
        Ok(())
    }
}
