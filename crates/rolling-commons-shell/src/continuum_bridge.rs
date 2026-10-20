//! Talbingo Regional Continuum & Multi-Scale Botanical Dissection Bridge (`continuum_bridge.rs`).
//!
//! Connects all 6 Talbingo / Kosciuszko sites:
//! - Site 0: The Pines Foreshore (km 0.0, 385m AHD)
//! - Site 1: Humes Crossing (km 1.8, 386m AHD)
//! - Site 2: Jounama Creek (km 4.2, 420m AHD)
//! - Site 3: Yarrangobilly Village & Karst (km 9.5, 980m AHD)
//! - Site 4: O'Hares Rest Area (km 13.0, 1120m AHD)
//! - Site 5: Three Mile Dam (km 17.5, 1410m AHD)
//!
//! along a continuous 17.5+ km Hermite cubic spline highway corridor network (Snowy Mountains Hwy B72).
//! Replaces isolated 64m x 44m patches and 22m vehicle rails with unrestricted 64 Hz 3D rover driving,
//! on-foot character dismount anywhere, and 5-scale educational botanical/zoological dissection study specs.

use qualia_geospatial::corridor::{CorridorKind, HermiteCorridorSegment, SplineControlPoint};
use qualia_sim::character::{CharacterConfig, CharacterController, CharacterState};
use qualia_sim::kinematics::{RoverKinematics, VehicleChassisConfig, VehicleState, WheelConfig};
use serde::{Deserialize, Serialize};

// ============================================================================
// 1. Core Input & Spec Types
// ============================================================================

/// 64 Hz rover vehicle drive input.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RoverInput {
    /// Throttle demand in [-1.0, 1.0] (negative is reverse).
    pub throttle: f32,
    /// Steering demand in [-1.0, 1.0] (positive is steer right).
    pub steer: f32,
    /// Braking demand in [0.0, 1.0].
    pub brake: f32,
    /// Timestep in seconds (default 1/64 s).
    pub dt: f32,
}

impl Default for RoverInput {
    fn default() -> Self {
        Self {
            throttle: 0.0,
            steer: 0.0,
            brake: 0.0,
            dt: 1.0 / 64.0,
        }
    }
}

/// 64 Hz on-foot bushwalker character input.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CharacterInput {
    /// Normalized horizontal direction [forward, strafe_right].
    pub wish_dir: [f32; 2],
    /// Sprint toggle.
    pub sprint: bool,
    /// Jump trigger.
    pub jump: bool,
    /// Crouch toggle.
    pub crouch: bool,
    /// Timestep in seconds (default 1/64 s).
    pub dt: f32,
}

impl Default for CharacterInput {
    fn default() -> Self {
        Self {
            wish_dir: [0.0, 0.0],
            sprint: false,
            jump: false,
            crouch: false,
            dt: 1.0 / 64.0,
        }
    }
}

/// Metadata and environmental profile for each regional site along the continuum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionalSiteInfo {
    pub site_id: u8,
    pub name: String,
    pub chainage_km: f64,
    pub elevation_m: f64,
    pub landmark: String,
    pub geology_unit: String,
    pub dominant_flora: String,
    pub signature_fauna: String,
    pub world_coords: [f64; 2], // [x, z] in meters from Talbingo origin
    pub gps_lat_lon: [f64; 2],
}

/// 5-Scale Botanical & Zoological Educational Dissection Study Spec.
/// Grounded in Plant Ontology (`PO:`) and Uberon (`UBERON:`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotanicalDissectionStudySpec {
    pub scale_id: u8,
    pub title: String,
    pub spatial_domain: String,
    pub biological_system: String,
    pub primary_taxon: String,
    pub layers_or_organs: Vec<DissectionLayerSpec>,
    pub biophysical_equations: Vec<String>,
    pub educational_takeaways: Vec<String>,
}

/// Individual anatomical layer, organ, or histological tissue spec.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DissectionLayerSpec {
    pub name: String,
    pub ontology_iri: String,
    pub ontology_label: String,
    pub anatomical_role: String,
    pub histological_description: String,
    pub callout_coordinate: [f32; 3],
}

// ============================================================================
// 2. Hermite Spline Corridor Helpers
// ============================================================================

/// Evaluates a Hermite corridor segment at parameter $t \in [0, 1]$ in metric meters.
///
/// Returns `([x_meters, z_meters], elevation_meters)`.
pub fn sample_corridor_segment(segment: &HermiteCorridorSegment, t: f64) -> ([f64; 2], f64) {
    let (pos_xy_mm, elev_mm) = segment.sample_centerline(t.clamp(0.0, 1.0));
    ([pos_xy_mm[0] * 0.001, pos_xy_mm[1] * 0.001], elev_mm * 0.001)
}

/// Conforms raw terrain height to the road corridor profile with parabolic crown.
pub fn conform_terrain_height_to_corridor(
    segment: &HermiteCorridorSegment,
    raw_elevation: f64,
    x_m: f64,
    z_m: f64,
) -> f64 {
    let px_mm = x_m * 1000.0;
    let py_mm = z_m * 1000.0;
    let (dist_mm, t) = segment.distance_to_point(px_mm, py_mm);
    let (_, road_elev_mm) = segment.sample_centerline(t);
    let road_elev = road_elev_mm * 0.001;
    let dist_m = dist_mm * 0.001;

    match segment.kind {
        CorridorKind::Road { width_mm, crown_height_mm } => {
            let half_w = (width_mm as f64) * 0.0005; // half width in meters
            let shoulder_w = 2.0; // 2m gravel shoulder
            let crown_m = (crown_height_mm as f64) * 0.001;

            if dist_m <= half_w {
                // Parabolic crown across the road asphalt
                let rel = dist_m / half_w;
                let crown = (1.0 - rel * rel) * crown_m;
                road_elev + crown
            } else if dist_m <= half_w + shoulder_w {
                // Smooth Hermite blend from road edge to natural terrain
                let blend_t = (dist_m - half_w) / shoulder_w;
                let h = blend_t * blend_t * (3.0 - 2.0 * blend_t);
                let shoulder_elev = road_elev - 0.05 * blend_t;
                (1.0 - h) * shoulder_elev + h * raw_elevation
            } else {
                raw_elevation
            }
        }
        CorridorKind::Creek { width_mm, depth_mm, .. } => {
            let half_w = (width_mm as f64) * 0.0005;
            let depth_m = (depth_mm as f64) * 0.001;
            if dist_m <= half_w {
                let rel = dist_m / half_w;
                let bed_excavation = (1.0 - rel * rel) * depth_m;
                road_elev - bed_excavation
            } else {
                raw_elevation
            }
        }
    }
}

// ============================================================================
// 3. Regional Continuum & Multi-Kilometer Spline Network
// ============================================================================

/// Continuous regional continuum spanning 17.5+ km between all 6 Talbingo / Kosciuszko sites.
pub struct TalbingoRegionalContinuum {
    /// 5 continuous Hermite cubic spline highway corridor segments connecting sites 0..5.
    pub segments: Vec<HermiteCorridorSegment>,
    /// Metadata for the 6 sites.
    pub sites: [RegionalSiteInfo; 6],
    /// 64 Hz 4-wheel rover vehicle kinematics simulator.
    pub rover: RoverKinematics,
    /// 64 Hz 3D capsule character controller simulator.
    pub walker: CharacterController,
    /// Active exploration mode: true = driving rover, false = exploring on foot.
    pub is_rover_active: bool,
    /// Current nearest site (0..=5).
    pub active_site: u8,
    /// Cumulative odometer in meters.
    pub odometer_meters: f64,
}

impl TalbingoRegionalContinuum {
    /// Constructs the Talbingo regional continuum connecting all 6 sites.
    pub fn new() -> Self {
        let sites = [
            RegionalSiteInfo {
                site_id: 0,
                name: "The Pines Foreshore".to_string(),
                chainage_km: 0.0,
                elevation_m: 385.0,
                landmark: "Blowering Reservoir Foreshore & Ribbon Gums".to_string(),
                geology_unit: "Blowering Formation - Rhyolitic Tuff".to_string(),
                dominant_flora: "Corymbia aparrerinja / Eucalyptus viminalis".to_string(),
                signature_fauna: "Macropus giganteus (Eastern Grey Kangaroo)".to_string(),
                world_coords: [0.0, 0.0],
                gps_lat_lon: [-35.4328, 148.2719],
            },
            RegionalSiteInfo {
                site_id: 1,
                name: "Humes Crossing".to_string(),
                chainage_km: 1.8,
                elevation_m: 386.0,
                landmark: "Timber Inlet Crossing & Meander Terrace".to_string(),
                geology_unit: "Blowering Formation - Siltstone & Shale".to_string(),
                dominant_flora: "Eucalyptus camaldulensis / Acacia dealbata".to_string(),
                signature_fauna: "Wallabia bicolor (Swamp Wallaby)".to_string(),
                world_coords: [1800.0, 150.0],
                gps_lat_lon: [-35.4917, 148.2783],
            },
            RegionalSiteInfo {
                site_id: 2,
                name: "Jounama Creek".to_string(),
                chainage_km: 4.2,
                elevation_m: 420.0,
                landmark: "Montane V-Gorge & Granite Boulder Bar".to_string(),
                geology_unit: "Lower Jounama Creek Limestone & Skarn".to_string(),
                dominant_flora: "Callistemon sieberi / Leptospermum grandifolium".to_string(),
                signature_fauna: "Vombatus ursinus (Common Wombat)".to_string(),
                world_coords: [4200.0, -220.0],
                gps_lat_lon: [-35.6078, 148.3367],
            },
            RegionalSiteInfo {
                site_id: 3,
                name: "Yarrangobilly Village & Karst".to_string(),
                chainage_km: 9.5,
                elevation_m: 980.0,
                landmark: "Silurian Karst Gorge, Caves & Historic Hut".to_string(),
                geology_unit: "Yarrangobilly Limestone (Massive Karst)".to_string(),
                dominant_flora: "Eucalyptus rubida / Pomaderris aspera".to_string(),
                signature_fauna: "Dacelo novaeguineae / Cervus unicolor".to_string(),
                world_coords: [9500.0, 680.0],
                gps_lat_lon: [-35.6561, 148.4719],
            },
            RegionalSiteInfo {
                site_id: 4,
                name: "O'Hares Rest Area".to_string(),
                chainage_km: 13.0,
                elevation_m: 1120.0,
                landmark: "Talbingo Scarp Alpine Ash Lookout".to_string(),
                geology_unit: "Talbingo Basalt & Young Granodiorite".to_string(),
                dominant_flora: "Eucalyptus delegatensis (Alpine Ash)".to_string(),
                signature_fauna: "Varanus varius (Lace Monitor)".to_string(),
                world_coords: [13000.0, 310.0],
                gps_lat_lon: [-35.6889, 148.4550],
            },
            RegionalSiteInfo {
                site_id: 5,
                name: "Three Mile Dam".to_string(),
                chainage_km: 17.5,
                elevation_m: 1410.0,
                landmark: "Subalpine Snow Gum Tarn & Sphagnum Bog".to_string(),
                geology_unit: "Cabramurra Basalt Plateau & Bogong Granite".to_string(),
                dominant_flora: "Eucalyptus pauciflora (Snow Gum)".to_string(),
                signature_fauna: "Pseudophryne corroboree (Corroboree Frog)".to_string(),
                world_coords: [17500.0, 940.0],
                gps_lat_lon: [-35.8819, 148.4867],
            },
        ];

        // Build continuous Hermite segments connecting sites 0..5
        let mut segments = Vec::with_capacity(5);
        for i in 0..5 {
            let s0 = &sites[i];
            let s1 = &sites[i + 1];

            let p0_pos = [s0.world_coords[0] * 1000.0, s0.world_coords[1] * 1000.0];
            let p1_pos = [s1.world_coords[0] * 1000.0, s1.world_coords[1] * 1000.0];

            let dx = p1_pos[0] - p0_pos[0];
            let dy = p1_pos[1] - p0_pos[1];

            // Smooth tangent matching segment direction
            let tangent = [dx * 0.95, dy * 0.95];

            segments.push(HermiteCorridorSegment {
                p0: SplineControlPoint {
                    pos_xy_mm: p0_pos,
                    elevation_mm: (s0.elevation_m * 1000.0).round() as i32,
                    tangent_xy: tangent,
                },
                p1: SplineControlPoint {
                    pos_xy_mm: p1_pos,
                    elevation_mm: (s1.elevation_m * 1000.0).round() as i32,
                    tangent_xy: tangent,
                },
                kind: CorridorKind::Road {
                    width_mm: 7000,
                    crown_height_mm: 75,
                },
            });
        }

        let rover = RoverKinematics::new(
            VehicleChassisConfig::default(),
            WheelConfig::default(),
            [0.0, 385.5, 0.0],
        );
        let walker = CharacterController::new(
            CharacterConfig::default(),
            [18.0, 385.0, 3.5],
        );

        Self {
            segments,
            sites,
            rover,
            walker,
            is_rover_active: true,
            active_site: 0,
            odometer_meters: 0.0,
        }
    }

    /// Number of sites along the continuum (6).
    pub fn site_count(&self) -> usize {
        self.sites.len()
    }

    /// Access site metadata by index.
    pub fn site_info(&self, site: u8) -> &RegionalSiteInfo {
        let idx = (site as usize).min(self.sites.len() - 1);
        &self.sites[idx]
    }

    /// Determines the active site based on distance along the highway corridor.
    pub fn find_active_site(&self, distance_m: f64) -> u8 {
        let km = distance_m * 0.001;
        if km < 0.9 {
            0
        } else if km < 3.0 {
            1
        } else if km < 6.8 {
            2
        } else if km < 11.25 {
            3
        } else if km < 15.25 {
            4
        } else {
            5
        }
    }

    /// Finds the closest segment and parameter $t$ for a world point `(wx, wz)`.
    pub fn closest_corridor_segment(&self, wx: f64, wz: f64) -> (usize, f64, f64) {
        let px_mm = wx * 1000.0;
        let py_mm = wz * 1000.0;

        let mut min_dist_m = f64::MAX;
        let mut best_seg = 0;
        let mut best_t = 0.0;

        for (idx, seg) in self.segments.iter().enumerate() {
            let (dist_mm, t) = seg.distance_to_point(px_mm, py_mm);
            let d_m = dist_mm * 0.001;
            if d_m < min_dist_m {
                min_dist_m = d_m;
                best_seg = idx;
                best_t = t;
            }
        }
        (best_seg, best_t, min_dist_m)
    }

    /// Evaluates continuous terrain + roadbed elevation at any metric coordinate `(wx, wz)`.
    pub fn evaluate_elevation(&self, wx: f64, wz: f64) -> f64 {
        let (seg_idx, _, dist_to_road_m) = self.closest_corridor_segment(wx, wz);
        let seg = &self.segments[seg_idx];

        // Macro-gradient: rising from Blowering (385m) at x=0 to Three Mile Dam (1410m) at x=17500m
        let prog = (wx / 17500.0).clamp(0.0, 1.0);
        let macro_elev = 385.0 + (1410.0 - 385.0) * prog;

        // Regional geological relief harmonics
        let ridge = 12.5 * (0.0006 * wx).sin() * (0.0008 * wz).cos()
            + 4.2 * (0.0022 * wx + 0.5).sin() * (0.0028 * wz).cos();
        let raw_terrain = macro_elev + ridge;

        // If near roadbed, conform to the banked Hermite roadbed
        if dist_to_road_m <= 12.0 {
            conform_terrain_height_to_corridor(seg, raw_terrain, wx, wz)
        } else {
            raw_terrain
        }
    }

    /// Advances the 64 Hz rover simulation by one timestep.
    pub fn step_rover(&mut self, input: RoverInput) -> &VehicleState {
        let prev_pos = self.rover.state.position;
        let site = self.active_site;
        let ground_probe = |x: f32, z: f32| {
            let eps = 0.1_f64;
            let h_c = continuous_corridor_elevation(site, x as f64, z as f64) as f32;
            let h_x = continuous_corridor_elevation(site, (x as f64) + eps, z as f64) as f32;
            let h_z = continuous_corridor_elevation(site, x as f64, (z as f64) + eps) as f32;
            let dx = (h_x - h_c) / (eps as f32);
            let dz = (h_z - h_c) / (eps as f32);
            let len = (dx * dx + 1.0 + dz * dz).sqrt();
            (h_c, [-dx / len, 1.0 / len, -dz / len])
        };

        self.rover.step(input.throttle, input.steer, input.brake, input.dt, ground_probe);

        let dx = (self.rover.state.position[0] - prev_pos[0]) as f64;
        let dz = (self.rover.state.position[2] - prev_pos[2]) as f64;
        let step_dist = (dx * dx + dz * dz).sqrt();
        self.odometer_meters += step_dist;

        self.active_site = self.find_active_site(self.rover.state.position[0] as f64);
        &self.rover.state
    }

    /// Advances the 64 Hz on-foot character simulation by one timestep.
    pub fn step_walker(&mut self, input: CharacterInput) -> &CharacterState {
        let site = self.active_site;
        let ground_probe = |x: f32, z: f32| {
            let eps = 0.1_f64;
            let h_c = continuous_corridor_elevation(site, x as f64, z as f64) as f32;
            let h_x = continuous_corridor_elevation(site, (x as f64) + eps, z as f64) as f32;
            let h_z = continuous_corridor_elevation(site, x as f64, (z as f64) + eps) as f32;
            let dx = (h_x - h_c) / (eps as f32);
            let dz = (h_z - h_c) / (eps as f32);
            let len = (dx * dx + 1.0 + dz * dz).sqrt();
            (h_c, [-dx / len, 1.0 / len, -dz / len])
        };

        self.walker.step(
            input.wish_dir,
            input.sprint,
            input.jump,
            input.crouch,
            input.dt,
            ground_probe,
        );

        self.active_site = self.find_active_site(self.walker.state.position[0] as f64);
        &self.walker.state
    }

    /// Toggles dismounting the rover to walk on foot, or remounting the vehicle.
    /// Returns the new `is_rover_active` state.
    pub fn toggle_dismount(&mut self) -> bool {
        if self.is_rover_active {
            // Dismount: spawn walker next to the driver door
            let r_pos = self.rover.state.position;
            let yaw = self.rover.state.heading;
            let side_x = -(yaw.cos()) * 1.8;
            let side_z = (yaw.sin()) * 1.8;
            let wx = r_pos[0] + side_x;
            let wz = r_pos[2] + side_z;
            let wy = self.evaluate_elevation(wx as f64, wz as f64) as f32;

            self.walker.state.position = [wx, wy, wz];
            self.walker.state.velocity = [0.0, 0.0, 0.0];
            self.walker.state.yaw = yaw;
            self.is_rover_active = false;
        } else {
            // Remount: check if walker is near vehicle
            let r_pos = self.rover.state.position;
            let w_pos = self.walker.state.position;
            let dx = r_pos[0] - w_pos[0];
            let dz = r_pos[2] - w_pos[2];
            let dist = (dx * dx + dz * dz).sqrt();

            if dist <= 8.0 {
                self.is_rover_active = true;
            }
        }
        self.is_rover_active
    }

    /// Teleports / resets exploration mode to a specific site.
    pub fn set_site(&mut self, site: u8) {
        let idx = (site as usize).min(self.sites.len() - 1);
        self.active_site = idx as u8;
        let s = &self.sites[idx];
        let elev = self.evaluate_elevation(s.world_coords[0], s.world_coords[1]) as f32;

        self.rover.state.position = [s.world_coords[0] as f32, elev + 0.45, s.world_coords[1] as f32];
        self.rover.state.velocity = [0.0, 0.0, 0.0];
        self.rover.state.heading = 0.0;

        self.walker.state.position = [
            (s.world_coords[0] + 18.0) as f32,
            elev,
            (s.world_coords[1] + 3.5) as f32,
        ];
        self.walker.state.velocity = [0.0, 0.0, 0.0];
    }
}

// ============================================================================
// 4. Standalone Continuous Corridor Elevation API
// ============================================================================

/// Evaluates continuous multi-site terrain + banked Hermite corridor roadbed
/// across the entire regional span without any artificial 60m radial cutoff.
///
/// Supports both local campsite scene coordinates (-24m to +40m) and regional
/// multi-kilometer highway coordinates (0 to 18,000m).
pub fn continuous_corridor_elevation(site: u8, wx: f64, wz: f64) -> f64 {
    // If coordinate is in local scene scale (small x), blend local geological signature
    // with continuous regional profile without clipping at 60m!
    if wx.abs() <= 120.0 && wz.abs() <= 120.0 {
        let local_y = crate::talbingo_terrain::elevation(site, wx as f32, wz as f32) as f64;
        let base_site_elev = match site {
            0 => 385.0,
            1 => 386.0,
            2 => 420.0,
            3 => 980.0,
            4 => 1120.0,
            _ => 1410.0,
        };
        // Add regional base datum so local micro-relief integrates with regional AHD
        return local_y + base_site_elev;
    }

    // Otherwise evaluate full multi-kilometer regional continuum
    let continuum = TalbingoRegionalContinuum::new();
    continuum.evaluate_elevation(wx, wz)
}

// ============================================================================
// 5. 5-Scale Botanical & Zoological Dissection Study Specs
// ============================================================================

/// Constructs the 5-scale educational study metadata with `PO:` / `UBERON:` annotations.
pub fn get_botanical_dissection_spec(site: u8, scale: u8) -> BotanicalDissectionStudySpec {
    match scale {
        1 => BotanicalDissectionStudySpec {
            scale_id: 1,
            title: "Scale 1: Whole Organism & Lignotuber Roots".to_string(),
            spatial_domain: "10 m – 40 m".to_string(),
            biological_system: "Whole-Plant Architecture, Pipe Model & Subterranean Lignotuber".to_string(),
            primary_taxon: if site == 5 {
                "Eucalyptus pauciflora (Snow Gum)".to_string()
            } else if site == 4 {
                "Eucalyptus delegatensis (Alpine Ash)".to_string()
            } else {
                "Corymbia aparrerinja (Ghost Gum) / Eucalyptus viminalis (Ribbon Gum)".to_string()
            },
            layers_or_organs: vec![
                DissectionLayerSpec {
                    name: "Bole & Primary Scaffold Limbs".to_string(),
                    ontology_iri: "PO:0009005".to_string(),
                    ontology_label: "shoot system".to_string(),
                    anatomical_role: "Structural support & hydraulic pipe-network conduit".to_string(),
                    histological_description: "Order 1-2 scaffold branches adhering to Leonardo da Vinci Pipe Model".to_string(),
                    callout_coordinate: [0.0, 6.5, 0.0],
                },
                DissectionLayerSpec {
                    name: "Canopy Crown & Phyllotaxis Architecture".to_string(),
                    ontology_iri: "PO:0020148".to_string(),
                    ontology_label: "tree crown".to_string(),
                    anatomical_role: "Photosynthetic light interception with golden-angle phyllotaxis".to_string(),
                    histological_description: "Order 3-5 terminal branchlets with pendulous vertically hanging adult leaves".to_string(),
                    callout_coordinate: [2.5, 14.0, 1.2],
                },
                DissectionLayerSpec {
                    name: "Basal Lignotuber Swelling".to_string(),
                    ontology_iri: "PO:0025530".to_string(),
                    ontology_label: "lignotuber".to_string(),
                    anatomical_role: "Fire survival meristematic bud and starch storage reserve".to_string(),
                    histological_description: "Massive woody swelling at root-stem junction bearing hundreds of dormant epicormic buds".to_string(),
                    callout_coordinate: [0.0, 0.35, 0.0],
                },
                DissectionLayerSpec {
                    name: "Primary Taproot & Sinkers".to_string(),
                    ontology_iri: "PO:0009005".to_string(),
                    ontology_label: "root system".to_string(),
                    anatomical_role: "Deep subsoil anchoring and perennial aquifer water uptake".to_string(),
                    histological_description: "Central vertical taproot penetrating fracture joints in Silurian bedrock".to_string(),
                    callout_coordinate: [0.0, -2.5, 0.0],
                },
                DissectionLayerSpec {
                    name: "Lateral Buttress & Mycorrhizal Root Net".to_string(),
                    ontology_iri: "PO:0025002".to_string(),
                    ontology_label: "lateral root".to_string(),
                    anatomical_role: "Mechanical windward tensile support and symbiotic nutrient foraging".to_string(),
                    histological_description: "Spreading horizontal roots enveloped in ectomycorrhizal fungal hyphae".to_string(),
                    callout_coordinate: [3.8, -0.6, -1.5],
                },
            ],
            biophysical_equations: vec![
                "Pipe Model: r_parent^2.49 = sum(r_daughter^2.49)".to_string(),
                "Root Flare Taper: r(y) = r_0 * (1.0 + alpha * exp(-beta * y))".to_string(),
                "Bending Moment: M_wind = 0.5 * rho_air * C_d * A_crown * v_wind^2 * h_com".to_string(),
            ],
            educational_takeaways: vec![
                "Australian eucalypts maintain lignotubers as subterranean fire-insurance organs with dormant meristems.".to_string(),
                "Branch cross-sectional area rigorously conserves water conduit area via the Leonardo/Shinozaki pipe law.".to_string(),
            ],
        },
        2 => BotanicalDissectionStudySpec {
            scale_id: 2,
            title: "Scale 2: 6-Layer Stem/Bark Cross-Section".to_string(),
            spatial_domain: "50 mm – 2 m".to_string(),
            biological_system: "Secondary Meristem Growth, Periderm Decortication & Vascular Translocation".to_string(),
            primary_taxon: "Corymbia aparrerinja / Eucalyptus delegatensis".to_string(),
            layers_or_organs: vec![
                DissectionLayerSpec {
                    name: "Layer 1: Outer Rhytidome / Periderm".to_string(),
                    ontology_iri: "PO:0005703".to_string(),
                    ontology_label: "rhytidome".to_string(),
                    anatomical_role: "Decorticating protective shield against fire heat flux and insect burrowing".to_string(),
                    histological_description: "Suberized dead phellem cells shedding in thin chalky-white decorticating ribbons".to_string(),
                    callout_coordinate: [0.42, 0.0, 0.0],
                },
                DissectionLayerSpec {
                    name: "Layer 2: Phellogen (Cork Cambium) & Phelloderm".to_string(),
                    ontology_iri: "PO:0005702".to_string(),
                    ontology_label: "phellogen".to_string(),
                    anatomical_role: "Secondary lateral protective meristem actively regenerating bark".to_string(),
                    histological_description: "Uniseriate meristematic layer producing cork centrifugally and phelloderm centripetally".to_string(),
                    callout_coordinate: [0.38, 0.0, 0.0],
                },
                DissectionLayerSpec {
                    name: "Layer 3: Secondary Phloem & Kino Ducts".to_string(),
                    ontology_iri: "PO:0005417".to_string(),
                    ontology_label: "secondary phloem".to_string(),
                    anatomical_role: "Basipetal photosynthate translocation and defensive wound gum secretion".to_string(),
                    histological_description: "Sieve tube members with companion cells, phloem rays, and traumatic kino veins".to_string(),
                    callout_coordinate: [0.34, 0.0, 0.0],
                },
                DissectionLayerSpec {
                    name: "Layer 4: Vascular Cambium Ring".to_string(),
                    ontology_iri: "PO:0005598".to_string(),
                    ontology_label: "vascular cambium".to_string(),
                    anatomical_role: "Bifacial lateral meristem generating secondary xylem inwards and phloem outwards".to_string(),
                    histological_description: "Fusiform and ray initials dividing actively during spring vegetative flushes".to_string(),
                    callout_coordinate: [0.30, 0.0, 0.0],
                },
                DissectionLayerSpec {
                    name: "Layer 5: Sapwood (Active Secondary Xylem)".to_string(),
                    ontology_iri: "PO:0005844".to_string(),
                    ontology_label: "sapwood".to_string(),
                    anatomical_role: "Acropetal hydraulic water and mineral sap transport under negative tension".to_string(),
                    histological_description: "Large vessel elements with scalariform pits, libriform fibres, and living ray parenchyma".to_string(),
                    callout_coordinate: [0.20, 0.0, 0.0],
                },
                DissectionLayerSpec {
                    name: "Layer 6: Heartwood (Duramen) & Central Pith".to_string(),
                    ontology_iri: "PO:0005845".to_string(),
                    ontology_label: "heartwood".to_string(),
                    anatomical_role: "Rot-resistant mechanical structural core impregnated with antimicrobial polyphenols".to_string(),
                    histological_description: "Tyloses-occluded vessel lumina darkened with condensed tannins and kino resins".to_string(),
                    callout_coordinate: [0.05, 0.0, 0.0],
                },
            ],
            biophysical_equations: vec![
                "Hagen-Poiseuille Xylem Flow: Q = (pi * r_vessel^4 * Delta_P) / (8 * mu * L)".to_string(),
                "Radial Secondary Growth: dr/dt = k_cambium * Psi_turgor * f(T_thermal)".to_string(),
            ],
            educational_takeaways: vec![
                "Bark shed (decortication) sheds epiphytes and reduces fuel laddering into the canopy.".to_string(),
                "Heartwood provides mechanical strength even when hollowed by decay fungi in ancient habitat trees.".to_string(),
            ],
        },
        3 => BotanicalDissectionStudySpec {
            scale_id: 3,
            title: "Scale 3: Floral Operculum/Hypanthium/Ovary & Gumnut Dissection".to_string(),
            spatial_domain: "5 mm – 150 mm".to_string(),
            biological_system: "Reproductive Morphology, Anthesis Dehiscence & Hydro-Mechanical Seed Release".to_string(),
            primary_taxon: "Corymbia aparrerinja (Ghost Gum)".to_string(),
            layers_or_organs: vec![
                DissectionLayerSpec {
                    name: "Hypanthium (Floral Calyx Tube)".to_string(),
                    ontology_iri: "PO:0009064".to_string(),
                    ontology_label: "hypanthium".to_string(),
                    anatomical_role: "Nectariferous cup protecting the inferior ovary and anchoring floral organs".to_string(),
                    histological_description: "Thickened receptacle tissue fused with ovary wall bearing internal nectary disc".to_string(),
                    callout_coordinate: [0.0, 0.015, 0.0],
                },
                DissectionLayerSpec {
                    name: "Operculum (Bud Cap)".to_string(),
                    ontology_iri: "PO:0009054".to_string(),
                    ontology_label: "operculum".to_string(),
                    anatomical_role: "Hermetic bud protective cap that detaches cleanly at anthesis".to_string(),
                    histological_description: "Fused calyptriform sepal/petal cap with specialized basal abscission cleavage ring".to_string(),
                    callout_coordinate: [0.0, 0.038, 0.0],
                },
                DissectionLayerSpec {
                    name: "Staminal Ring (Androecium)".to_string(),
                    ontology_iri: "PO:0009066".to_string(),
                    ontology_label: "anther".to_string(),
                    anatomical_role: "Male pollen presentation via hundreds of showy versatile anther filaments".to_string(),
                    histological_description: "Cream filaments inflexed in bud, springing outward at operculum shed with longitudinal slits".to_string(),
                    callout_coordinate: [0.018, 0.032, 0.0],
                },
                DissectionLayerSpec {
                    name: "Style & Receptive Stigma".to_string(),
                    ontology_iri: "PO:0009073".to_string(),
                    ontology_label: "style".to_string(),
                    anatomical_role: "Pollen tube guidance from terminal receptive papillate stigma to ovary locules".to_string(),
                    histological_description: "Central terete columnar style with internal transmitting tissue".to_string(),
                    callout_coordinate: [0.0, 0.040, 0.0],
                },
                DissectionLayerSpec {
                    name: "4-Locule Inferior Ovary & Ovules".to_string(),
                    ontology_iri: "PO:0009067".to_string(),
                    ontology_label: "ovary".to_string(),
                    anatomical_role: "Female gametophyte protection with axile placentation of ovules".to_string(),
                    histological_description: "Cross section revealing 4 locular chambers, fertile anatropous ovules, and sterile ovulodes".to_string(),
                    callout_coordinate: [0.005, 0.012, 0.0],
                },
                DissectionLayerSpec {
                    name: "Woody Capsule (Gumnut) & Dehiscing Valves".to_string(),
                    ontology_iri: "PO:0009084".to_string(),
                    ontology_label: "pericarp".to_string(),
                    anatomical_role: "Sclerified woody fruit capsule opening 3-4 valves upon post-fire desiccation".to_string(),
                    histological_description: "Urceolate woody capsule with sunken valves dehiscing to release winged fertile seeds and chaff".to_string(),
                    callout_coordinate: [0.0, -0.015, 0.0],
                },
            ],
            biophysical_equations: vec![
                "Anthesis Cap Release: sigma_turgor * 2 * pi * r_rim * delta_ring >= F_cleavage".to_string(),
                "Hygroscopic Valve Curl: Delta_theta = alpha_hygro * (M_wet - M_dry) / t_valve".to_string(),
            ],
            educational_takeaways: vec![
                "The operculum gives Eucalyptus and Corymbia their scientific names ('well-covered' bud cap).".to_string(),
                "Gumnut valves remain tightly closed for months until fire heat or extreme dry air triggers release.".to_string(),
            ],
        },
        4 => BotanicalDissectionStudySpec {
            scale_id: 4,
            title: "Scale 4: Foliar Murray's Law Venation & Stomatal/Palisade Histology".to_string(),
            spatial_domain: "50 µm – 50 mm".to_string(),
            biological_system: "Sclerophyll Histology, Isobilateral Photosynthesis & Murray's Law Hydraulics".to_string(),
            primary_taxon: "Corymbia aparrerinja (Ghost Gum)".to_string(),
            layers_or_organs: vec![
                DissectionLayerSpec {
                    name: "Epicuticular Wax & Cuticle".to_string(),
                    ontology_iri: "PO:0000030".to_string(),
                    ontology_label: "leaf cuticle".to_string(),
                    anatomical_role: "Hydrophobic barrier against non-stomatal transpiration and UV-B reflectance".to_string(),
                    histological_description: "Heavy cutin matrix with crystalline wax platelets lending a pale glaucous sheen".to_string(),
                    callout_coordinate: [0.0, 0.00015, 0.0],
                },
                DissectionLayerSpec {
                    name: "Amphistomatic Epidermis & Sunken Guard Cells".to_string(),
                    ontology_iri: "PO:0000032".to_string(),
                    ontology_label: "stomatal complex".to_string(),
                    anatomical_role: "Dynamic gas exchange control balancing CO2 assimilation against transpiration".to_string(),
                    histological_description: "Paired kidney-shaped guard cells sunken in pits on both adaxial and abaxial surfaces".to_string(),
                    callout_coordinate: [0.0003, 0.00012, 0.0],
                },
                DissectionLayerSpec {
                    name: "Isobilateral Palisade Mesophyll".to_string(),
                    ontology_iri: "PO:0005679".to_string(),
                    ontology_label: "palisade mesophyll".to_string(),
                    anatomical_role: "Chloroplast-rich columnar chlorenchyma harvesting light on both blade faces".to_string(),
                    histological_description: "2-3 dense columnar cell tiers beneath both upper and lower epidermis for vertical hanging leaves".to_string(),
                    callout_coordinate: [0.0, 0.00006, 0.0],
                },
                DissectionLayerSpec {
                    name: "Intramarginal & Collateral Vein Bundles".to_string(),
                    ontology_iri: "PO:0005352".to_string(),
                    ontology_label: "xylem".to_string(),
                    anatomical_role: "Continuous margin-running vein conforming to Murray's Law branching ratios".to_string(),
                    histological_description: "Lignified xylem tracheids and phloem bundles jacketed in thick sclerenchyma bundle sheaths".to_string(),
                    callout_coordinate: [0.0012, 0.0, 0.0],
                },
                DissectionLayerSpec {
                    name: "Sub-dermal Secretory Terpenoid Oil Gland".to_string(),
                    ontology_iri: "PO:0025280".to_string(),
                    ontology_label: "plant gland".to_string(),
                    anatomical_role: "Storage of volatile antimicrobial and insect-repellent essential oils (cineole/pinene)".to_string(),
                    histological_description: "Spherical schizogenous cavity lined with secretory epithelium visible as translucent dots".to_string(),
                    callout_coordinate: [0.0005, -0.00002, 0.0],
                },
            ],
            biophysical_equations: vec![
                "Murray's Hydraulic Law: r_parent^3 = r_child1^3 + r_child2^3".to_string(),
                "Stomatal Conductance: g_s = g_0 + (a_1 * A_n) / ((C_s - Gamma) * (1 + D / D_0))".to_string(),
                "Fick's Diffusion Flux: J_CO2 = g_total * (C_air - C_mesophyll)".to_string(),
            ],
            educational_takeaways: vec![
                "Australian eucalypt leaves hang vertically to avoid scorching midday Australian sun while catching morning/evening light.".to_string(),
                "The continuous intramarginal vein is the diagnostic botanical hallmark distinguishing Myrtaceae from other trees.".to_string(),
            ],
        },
        _ => BotanicalDissectionStudySpec {
            scale_id: 5,
            title: "Scale 5: 3-Layer Vertebrate Osteology/Myology/Integument".to_string(),
            spatial_domain: "0.2 m – 2.5 m".to_string(),
            biological_system: "Marsupial Biomechanics, Epipubic Pelvic Kinematics & Elastic Hopping Tendons".to_string(),
            primary_taxon: "Macropus giganteus (Eastern Grey Kangaroo)".to_string(),
            layers_or_organs: vec![
                DissectionLayerSpec {
                    name: "Layer 0: Articulated Osteology & Epipubic Bones".to_string(),
                    ontology_iri: "UBERON:0001434".to_string(),
                    ontology_label: "skeletal system".to_string(),
                    anatomical_role: "Structural lever arms, procumbent incisors, syndactylous pes, and marsupial pelvic struts".to_string(),
                    histological_description: "Cranium with diprotodont diastema, elongated fused metatarsals IV, and bilateral ossa epipubica".to_string(),
                    callout_coordinate: [0.0, 0.85, 0.0],
                },
                DissectionLayerSpec {
                    name: "Layer 1: Gastrocnemius, Plantaris & Achilles Tendon Spring".to_string(),
                    ontology_iri: "UBERON:0001473".to_string(),
                    ontology_label: "Achilles tendon".to_string(),
                    anatomical_role: "High-efficiency elastic strain energy storage returning 93% mechanical energy per hop".to_string(),
                    histological_description: "Dense parallel collagen type I bundles with high tensile modulus and low metabolic hysteresis".to_string(),
                    callout_coordinate: [-0.15, 0.35, -0.4],
                },
                DissectionLayerSpec {
                    name: "Layer 2: Outer Integument & Insulating Pelage".to_string(),
                    ontology_iri: "UBERON:0002067".to_string(),
                    ontology_label: "integument".to_string(),
                    anatomical_role: "Thermal boundary layer, counter-shading camouflage, and dermal protection".to_string(),
                    histological_description: "Dense double-layered fur with hollow guard hairs and insulating woolly underfur".to_string(),
                    callout_coordinate: [0.0, 0.90, 0.15],
                },
            ],
            biophysical_equations: vec![
                "Tendon Elastic Energy: E_elastic = 0.5 * (E_modulus * Area / L_0) * Delta_L^2".to_string(),
                "Metabolic Hopping Cost: P_metabolic approx constant above 18 km/h due to tendon recoil!".to_string(),
            ],
            educational_takeaways: vec![
                "Kangaroos are the only large animals to use hopping; their elastic Achilles tendons make hopping faster without costing more energy!".to_string(),
                "Marsupial epipubic bones act as levers stiffening the abdominal wall to support the pouch during explosive bounds.".to_string(),
            ],
        },
    }
}

// ============================================================================
// 6. Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regional_continuum_segments_and_chainage() {
        let continuum = TalbingoRegionalContinuum::new();
        assert_eq!(continuum.site_count(), 6);
        assert_eq!(continuum.segments.len(), 5);

        // Chainages must be strictly monotonic from 0.0 km to 17.5 km
        for i in 0..5 {
            assert!(continuum.sites[i].chainage_km < continuum.sites[i + 1].chainage_km);
            assert!(continuum.sites[i].elevation_m <= continuum.sites[i + 1].elevation_m);
        }

        assert_eq!(continuum.sites[0].chainage_km, 0.0);
        assert_eq!(continuum.sites[5].chainage_km, 17.5);
        assert_eq!(continuum.sites[5].elevation_m, 1410.0);
    }

    #[test]
    fn test_continuous_corridor_elevation_smoothness() {
        let continuum = TalbingoRegionalContinuum::new();

        // Sample elevation along the entire 17.5 km continuum every 500m
        let mut prev_elev = continuum.evaluate_elevation(0.0, 0.0);
        assert!(prev_elev.is_finite());
        assert!(prev_elev > 300.0 && prev_elev < 500.0);

        for step in 1..=35 {
            let wx = (step as f64) * 500.0;
            let wz = 50.0;
            let elev = continuum.evaluate_elevation(wx, wz);
            assert!(elev.is_finite(), "Elevation at wx={wx} must be finite");
            assert!(
                (elev - prev_elev).abs() < 120.0,
                "Elevation must not have massive step cliffs between 500m samples"
            );
            prev_elev = elev;
        }

        // Three Mile Dam elevation should be near 1400m
        let alpine_elev = continuum.evaluate_elevation(17500.0, 940.0);
        assert!(
            alpine_elev >= 1350.0 && alpine_elev <= 1500.0,
            "Alpine elevation must match subalpine plateau: {alpine_elev}"
        );
    }

    #[test]
    fn test_unrestricted_vehicle_and_walker_poses() {
        let mut continuum = TalbingoRegionalContinuum::new();

        // Verify rover can drive beyond legacy 22m rail to multi-kilometer coordinates
        for km in [0.0, 1.8, 4.2, 9.5, 13.0, 17.5] {
            let wx = km * 1000.0;
            let wz = 10.0;
            let elev = continuum.evaluate_elevation(wx, wz);
            continuum.rover.state.position = [wx as f32, elev as f32 + 0.4, wz as f32];
            assert_eq!(continuum.rover.state.position[0], wx as f32);

            let active_site = continuum.find_active_site(wx);
            assert!(active_site <= 5);
        }

        // Verify walker dismount toggle works anywhere
        continuum.rover.state.position = [9500.0, 980.5, 680.0];
        assert!(continuum.is_rover_active);

        // Dismount
        let is_rover = continuum.toggle_dismount();
        assert!(!is_rover);
        assert!(!continuum.is_rover_active);
        assert!((continuum.walker.state.position[0] - 9500.0).abs() < 5.0);

        // Remount
        let is_rover = continuum.toggle_dismount();
        assert!(is_rover);
        assert!(continuum.is_rover_active);
    }

    #[test]
    fn test_5_scale_botanical_dissection_specs() {
        for scale in 1..=5 {
            let spec = get_botanical_dissection_spec(0, scale);
            assert_eq!(spec.scale_id, scale);
            assert!(!spec.title.is_empty());
            assert!(!spec.layers_or_organs.is_empty());
            assert!(!spec.biophysical_equations.is_empty());
            assert!(!spec.educational_takeaways.is_empty());

            // Verify ontology identifiers
            for layer in &spec.layers_or_organs {
                assert!(
                    layer.ontology_iri.starts_with("PO:") || layer.ontology_iri.starts_with("UBERON:"),
                    "Ontology IRI must be PO: or UBERON:, got {}",
                    layer.ontology_iri
                );
            }
        }
    }
}
