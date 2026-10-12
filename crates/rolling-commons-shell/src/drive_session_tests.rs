//! Authored native persistence fixtures; execution remains deferred.
use super::*;
fn simulation() -> GameDriveSimulation {
    GameDriveSimulation::new(include_str!("../../../web/fixtures/drive-simulation.vibe")).unwrap()
}
fn session() -> GameSession {
    GameSession::new("", r#"{"actions":[]}"#).unwrap()
}
fn controls() -> Value {
    json!({"dt":0.05,"x":999.0,"z":999.0,"heading":0.0,"speed":90.0,
    "rig_id":0,"throttle":true,"reverse":false,"brake":false,"steer":0.0,
    "terrain_height":0.0,"terrain_height_ahead":0.5,"terrain_height_behind":-0.5,
    "terrain_sample_distance":0.55,"crossing_open":true,"period":"day","hazards":[],
    "engine_health":100.0,"oil_temp":10.0,"coolant_temp":10.0,"overheat_seconds":0.0,
    "traffic_queue":0.0,"hazard_stops":0.0,"emergency_stops":0.0,"last_warning":""})
}
#[test]
fn native_tick_ignores_browser_motion_and_temperature_then_restores_saved_state() {
    let sim = simulation();
    let mut game = session();
    game.drive_tick(&sim, 1, &controls().to_string()).unwrap();
    let saved = load(&game, &sim, 1, true).unwrap();
    assert!(saved.state["x"].as_f64().unwrap() < 0.0);
    assert!(saved.state["oil_temp"].as_f64().unwrap() > 90.0);
    let loaded = GameSession::new(&game.world(), r#"{"actions":[]}"#).unwrap();
    assert_eq!(
        game.drive_resume(&sim, 1).unwrap(),
        loaded.drive_resume(&sim, 1).unwrap()
    );
    assert_eq!(load(&loaded, &sim, 2, true).unwrap().ticks, 0);
    assert_eq!(game.seq, 0);
    assert!(game.events.is_empty());
}
#[test]
fn overheating_damage_survives_parking_and_different_rig_selection() {
    let sim = simulation();
    let mut game = session();
    let input = controls().to_string();
    for _ in 0..5000 {
        game.drive_tick(&sim, 1, &input).unwrap();
        if number(
            &load(&game, &sim, 1, false).unwrap().state,
            "engine_health",
            0.0,
            100.0,
        )
        .unwrap()
            < 99.0
        {
            break;
        }
    }
    let damaged = load(&game, &sim, 1, true).unwrap().state;
    assert!(damaged["engine_health"].as_f64().unwrap() < 100.0);
    game.drive_park(&sim, 1).unwrap();
    let parked = load(&game, &sim, 1, true).unwrap().state;
    for key in [
        "engine_health",
        "oil_temp",
        "coolant_temp",
        "distance",
        "x",
        "z",
    ] {
        assert_eq!(parked[key], damaged[key]);
    }
    assert_eq!(parked["speed"], json!(0.0));
    assert_eq!(
        load(&game, &sim, 0, true).unwrap().state["engine_health"],
        json!(100.0)
    );
    let loaded = GameSession::new(&game.world(), r#"{"actions":[]}"#).unwrap();
    assert_eq!(
        loaded.drive_resume(&sim, 1).unwrap(),
        game.drive_resume(&sim, 1).unwrap()
    );
}
#[test]
fn corrupt_checkpoint_and_invalid_tick_are_rejected_without_mutation() {
    let sim = simulation();
    let mut game = session();
    game.drive_tick(&sim, 0, &controls().to_string()).unwrap();
    let before = game.world();
    let mut invalid = controls();
    invalid["dt"] = json!(2.0);
    assert!(game.drive_tick(&sim, 0, &invalid.to_string()).is_err());
    assert_eq!(game.world(), before);
    let mut corrupt = load(&game, &sim, 0, true).unwrap();
    corrupt.state["engine_health"] = json!(40.0);
    persist(&mut game, &corrupt).unwrap();
    assert!(game.drive_resume(&sim, 0).is_err());
}
#[test]
fn trailer_heading_persists_and_reverse_angle_limit_stops_motion() {
    let sim = simulation();
    let mut game = session();
    let mut input = controls();
    input["steer"] = json!(0.65);
    for _ in 0..30 {
        game.drive_tick(&sim, 1, &input.to_string()).unwrap();
    }
    let turning = load(&game, &sim, 1, true).unwrap();
    assert!(
        (turning.state["trailer_heading"].as_f64().unwrap()
            - turning.state["heading"].as_f64().unwrap())
        .abs()
            > 0.001
    );
    let loaded = GameSession::new(&game.world(), r#"{"actions":[]}"#).unwrap();
    assert_eq!(
        loaded.drive_resume(&sim, 1).unwrap(),
        game.drive_resume(&sim, 1).unwrap()
    );
    // Exercise the authored policy's recovery boundary directly from a
    // defined scenario input, independently of a browser state mutation.
    let mut boundary = controls();
    boundary["x"] = json!(-5.45);
    boundary["z"] = json!(0.48);
    boundary["rig_id"] = json!(1);
    boundary["speed"] = json!(-0.3);
    boundary["heading"] = json!(0.0);
    boundary["trailer_heading"] = json!(-1.2);
    boundary["wheel_angle"] = json!(0.0);
    boundary["trailer_wheel_angle"] = json!(0.0);
    boundary["throttle"] = json!(false);
    boundary["reverse"] = json!(true);
    boundary["rig_roadworthy"] = json!(true);
    boundary["cooling_efficiency"] = json!(1.0);
    let output = evaluate(&sim, "drive_step", vec![crate::json_to_vibe(&boundary)]).unwrap();
    assert_eq!(output["speed"], json!(0.0));
    assert!(output["trailer_warning"]
        .as_str()
        .unwrap()
        .contains("ANGLE LIMIT"));
    boundary["trailer_heading"] = json!(-0.5);
    let below_limit = evaluate(&sim, "drive_step", vec![crate::json_to_vibe(&boundary)]).unwrap();
    assert!(below_limit["speed"].as_f64().unwrap() < 0.0);
}

#[test]
fn flat_contact_initializes_once_and_keeps_each_tyre_on_the_surface() {
    let sim = simulation();
    let state = initial(&sim, 1).unwrap().state;
    let contacts = json!({"fl":0.0,"fr":0.0,"rl":0.0,"rr":0.0,"front_z":-0.77,"rear_z":0.78,"track":1.72,
        "tfl":0.0,"tfr":0.0,"trl":0.0,"trr":0.0,"trailer_front_z":1.90,"trailer_rear_z":2.45,
        "trailer_track":1.86,"hitch_offset":1.36});
    let result = evaluate(
        &sim,
        "drive_contact_step",
        vec![
            crate::json_to_vibe(&state),
            crate::json_to_vibe(&state),
            crate::json_to_vibe(&contacts),
            vibe::Value::F64(0.05),
        ],
    )
    .unwrap();
    assert_eq!(result["contact_initialized"], json!(true));
    assert!(result["body_pitch"].as_f64().unwrap().abs() < 1e-12);
    assert!(result["body_roll"].as_f64().unwrap().abs() < 1e-12);
    let height = result["body_height"].as_f64().unwrap();
    for field in [
        "suspension_fl",
        "suspension_fr",
        "suspension_rl",
        "suspension_rr",
    ] {
        let tyre_bottom = height + 0.29 + result[field].as_f64().unwrap() - 0.34;
        assert!(tyre_bottom.abs() < 1e-9);
    }
}
#[test]
fn first_parking_initializes_contact_without_changing_damage_or_travel() {
    let sim = simulation();
    let mut game = session();
    let initial = load(&game, &sim, 0, true).unwrap().state;
    game.drive_park(&sim, 0).unwrap();
    let parked = load(&game, &sim, 0, true).unwrap().state;
    assert_eq!(parked["contact_initialized"], json!(true));
    for key in [
        "engine_health",
        "oil_temp",
        "coolant_temp",
        "distance",
        "x",
        "z",
    ] {
        assert_eq!(parked[key], initial[key]);
    }
    assert!(game.seq == 0 && game.events.is_empty());
}
#[test]
fn native_contact_samples_replace_browser_grade_values() {
    let sim = simulation();
    let mut game = session();
    let mut input = controls();
    input["terrain_height_ahead"] = json!(999.0);
    input["terrain_height_behind"] = json!(-999.0);
    game.drive_tick(&sim, 0, &input.to_string()).unwrap();
    let saved = load(&game, &sim, 0, true).unwrap();
    let facts = saved.input.unwrap();
    assert!(facts["terrain_height_ahead"].as_f64().unwrap().abs() < 20.0);
    assert!(facts["terrain_height_behind"].as_f64().unwrap().abs() < 20.0);
    assert_eq!(saved.contact_hash, crate::drive_contacts::SOURCE_HASH);
    for field in [
        "suspension_fl",
        "suspension_fr",
        "suspension_rl",
        "suspension_rr",
    ] {
        assert!(saved.state[field].as_f64().unwrap().abs() <= 0.18);
    }
}

#[test]
fn changed_native_solver_fingerprint_cannot_reinterpret_saved_motion() {
    let sim = simulation();
    let mut game = session();
    game.drive_tick(&sim, 0, &controls().to_string()).unwrap();
    let mut saved = load(&game, &sim, 0, true).unwrap();
    saved.contact_hash ^= 1;
    persist(&mut game, &saved).unwrap();
    let before = game.world();
    assert!(game.drive_resume(&sim, 0).is_err());
    assert_eq!(game.world(), before);
}
