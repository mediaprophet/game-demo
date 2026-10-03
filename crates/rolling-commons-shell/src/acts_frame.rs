//! Act-gated frame light and custody.
//!
//! Halo and villain light, and a cell marker, come only from named acts.
//! Poverty, wealth, exposure, lack of housing, injury, and no medication are
//! conditions. They never set light, never open a cell, and never become a
//! drug act or a junkie look. Custody is only for a human subject
//! (`human`, `NaturalAgent`, or `who`). A company, agent, tool, or instrument
//! cannot be in a cell.

use serde::Serialize;
use serde_json::{Map, Value};
use wasm_bindgen::prelude::*;

/// Light is always attributed to acts, never to a condition or a who-kind.
pub const LIGHT_SOURCE: &str = "acts";

/// Kindness acts. Halo on the frame. Not custody.
const HALO_ACTS: &[&str] = &["act.kindness", "act.care"];

/// Harm acts. Villain light on the frame. Only the custody allowlist opens a cell.
const VILLAIN_ACTS: &[&str] = &[
    "act.violence_against_person",
    "act.theft_with_force",
    "act.harm",
    "act.illegal_drug_supply",
];

/// Short allowlist. A cell follows one of these, and only for a human.
/// Drug supply is an act on the supplier. It is never inferred from no housing.
const CUSTODY_ACTS: &[&str] = &[
    "act.violence_against_person",
    "act.theft_with_force",
    "act.illegal_drug_supply",
];

/// The only claim that counts as a drug act. Conditions never map onto it.
const DRUG_SUPPLY_ACT: &str = "act.illegal_drug_supply";

const CONDITION_KINDS: &[&str] = &[
    "condition.poverty",
    "condition.wealth",
    "condition.exposure",
    "condition.no_housing",
    "condition.injury",
    "condition.no_medication",
    "condition.cannot_afford_child",
    "condition.spent_caring_for_relative",
    "condition.lost_business",
    "condition.returned_from_war",
    "condition.unhappy_relationship_mortgage_stuck",
];

/// Paths into no housing. Kept as themselves. Never folded into one homeless kind.
const PATHWAYS: &[&str] = &[
    "condition.cannot_afford_child",
    "condition.spent_caring_for_relative",
    "condition.lost_business",
    "condition.returned_from_war",
    "condition.unhappy_relationship_mortgage_stuck",
];

/// Conditions that must not be rewritten as a drug act or a junkie costume.
const NO_DRUG_INFERENCE: &[&str] = &[
    "condition.no_housing",
    "condition.injury",
    "condition.no_medication",
];

/// Object keys that are conditions even when a caller smuggles them beside acts.
const CONDITION_FIELDS: &[&str] = &[
    "poverty",
    "wealth",
    "exposure",
    "no_housing",
    "homelessness",
    "injury",
    "no_medication",
];

const NON_HUMAN: &[&str] = &["company", "agent", "tool", "instrument"];

#[derive(Serialize)]
struct FrameLight {
    light: String,
    source: String,
}

#[derive(Serialize)]
struct CustodyDecision {
    custody: bool,
    reason: String,
}

#[derive(Serialize)]
struct ActsFrameEval {
    ok: bool,
    light: String,
    custody: bool,
    reason: String,
    /// True only when `act.illegal_drug_supply` is present. Never from a condition.
    drug_inference: bool,
    /// Costume look. Always `none`. Junkie and a collapsed homeless silhouette are refused.
    look: String,
    /// Named paths into no housing, unchanged. Never rewritten to a generic homeless id.
    pathways: Vec<String>,
    refused: Vec<String>,
}

struct Parsed {
    acts: Vec<String>,
    conditions: Vec<String>,
    pathways: Vec<String>,
    junkie_look_claimed: bool,
    silhouette_hides_path: bool,
}

fn is_human(kind: &str) -> bool {
    matches!(kind, "human" | "NaturalAgent" | "who")
}

fn is_condition_kind(kind: &str) -> bool {
    CONDITION_KINDS.iter().any(|c| *c == kind)
        || matches!(
            kind,
            "poverty"
                | "wealth"
                | "exposure"
                | "no_housing"
                | "homelessness"
                | "injury"
                | "no_medication"
        )
}

fn is_pathway(kind: &str) -> bool {
    PATHWAYS.iter().any(|p| *p == kind)
}

/// A costume that hides which path led here. Not a condition id.
fn hides_path(kind: &str) -> bool {
    matches!(
        kind.trim().to_ascii_lowercase().as_str(),
        "homeless"
            | "homelessness"
            | "look.homeless"
            | "look.homeless_silhouette"
            | "silhouette.homeless"
            | "homeless_silhouette"
            | "human.garbage"
            | "garbage"
            | "look.garbage"
    )
}

fn normalize_condition(kind: &str) -> String {
    // Pathways stay spelled as named. They are not rewritten to no_housing.
    if is_pathway(kind) || kind.starts_with("condition.") {
        return kind.to_string();
    }
    format!("condition.{kind}")
}

fn push_unique(list: &mut Vec<String>, item: String) {
    if !list.iter().any(|e| e == &item) {
        list.push(item);
    }
}

fn push_kind(kind: &str, parsed: &mut Parsed) {
    if kind.is_empty() {
        return;
    }
    if hides_path(kind) {
        parsed.silhouette_hides_path = true;
        return;
    }
    if is_pathway(kind) {
        push_unique(&mut parsed.pathways, kind.to_string());
        push_unique(&mut parsed.conditions, kind.to_string());
        return;
    }
    if is_condition_kind(kind) {
        push_unique(&mut parsed.conditions, normalize_condition(kind));
        return;
    }
    push_unique(&mut parsed.acts, kind.to_string());
}

fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::String(s) => {
            let t = s.trim();
            !t.is_empty() && t != "false" && t != "0"
        }
        Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
        Value::Array(a) => !a.is_empty(),
        Value::Object(_) => true,
    }
}

fn note_condition_fields(map: &Map<String, Value>, parsed: &mut Parsed) {
    for field in CONDITION_FIELDS {
        let Some(value) = map.get(*field) else {
            continue;
        };
        if !truthy(value) {
            continue;
        }
        if *field == "homelessness" {
            parsed.silhouette_hides_path = true;
            continue;
        }
        push_kind(&format!("condition.{field}"), parsed);
    }
    for (field, id) in [
        ("cannot_afford_child", "condition.cannot_afford_child"),
        ("spent_caring_for_relative", "condition.spent_caring_for_relative"),
        ("lost_business", "condition.lost_business"),
        ("returned_from_war", "condition.returned_from_war"),
        (
            "unhappy_relationship_mortgage_stuck",
            "condition.unhappy_relationship_mortgage_stuck",
        ),
    ] {
        if map.get(field).is_some_and(truthy) {
            push_kind(id, parsed);
        }
    }
    // `housing: false` / "none" / "lacking" is no housing. A present home is not.
    if let Some(value) = map.get("housing") {
        let lacking = match value {
            Value::Bool(b) => !*b,
            Value::String(s) => {
                let t = s.trim().to_ascii_lowercase();
                t.is_empty() || t == "false" || t == "none" || t == "lacking" || t == "no"
            }
            Value::Null => true,
            _ => false,
        };
        if lacking {
            push_kind("condition.no_housing", parsed);
        }
    }
}

fn junkie_token(value: &str) -> bool {
    value.trim().eq_ignore_ascii_case("junkie") || value.trim().eq_ignore_ascii_case("look.junkie")
}

fn note_junkie_look(map: &Map<String, Value>) -> bool {
    for key in ["look", "costume", "palette", "silhouette"] {
        if let Some(Value::String(s)) = map.get(key) {
            if junkie_token(s) {
                return true;
            }
        }
    }
    false
}

fn note_silhouette(map: &Map<String, Value>) -> bool {
    for key in ["look", "costume", "palette", "silhouette"] {
        if let Some(Value::String(s)) = map.get(key) {
            if hides_path(s) {
                return true;
            }
        }
    }
    false
}

fn push_item(item: &Value, parsed: &mut Parsed) {
    match item {
        Value::String(s) => {
            if junkie_token(s) {
                parsed.junkie_look_claimed = true;
                return;
            }
            push_kind(s, parsed);
        }
        Value::Object(map) => {
            if let Some(kind) = map
                .get("kind")
                .or_else(|| map.get("id"))
                .or_else(|| map.get("act"))
                .and_then(|v| v.as_str())
            {
                if junkie_token(kind) {
                    parsed.junkie_look_claimed = true;
                } else {
                    push_kind(kind, parsed);
                }
            }
            if note_junkie_look(map) {
                parsed.junkie_look_claimed = true;
            }
            if note_silhouette(map) {
                parsed.silhouette_hides_path = true;
            }
            note_condition_fields(map, parsed);
        }
        _ => {}
    }
}

fn parse_claims(claims_json: &str) -> Result<Parsed, String> {
    let value: Value =
        serde_json::from_str(claims_json).map_err(|_| "claims_json_unreadable".to_string())?;
    let mut parsed = Parsed {
        acts: Vec::new(),
        conditions: Vec::new(),
        pathways: Vec::new(),
        junkie_look_claimed: false,
        silhouette_hides_path: false,
    };
    match value {
        Value::Array(items) => {
            for item in &items {
                push_item(item, &mut parsed);
            }
        }
        Value::Object(map) => {
            if let Some(items) = map.get("acts").and_then(|v| v.as_array()) {
                for item in items {
                    push_item(item, &mut parsed);
                }
            }
            if let Some(items) = map.get("claims").and_then(|v| v.as_array()) {
                for item in items {
                    push_item(item, &mut parsed);
                }
            }
            if note_junkie_look(&map) {
                parsed.junkie_look_claimed = true;
            }
            if note_silhouette(&map) {
                parsed.silhouette_hides_path = true;
            }
            note_condition_fields(&map, &mut parsed);
        }
        _ => return Err("claims_json_unreadable".to_string()),
    }
    Ok(parsed)
}

fn light_of(parsed: &Parsed) -> FrameLight {
    let villain = parsed
        .acts
        .iter()
        .any(|a| VILLAIN_ACTS.iter().any(|v| v == a));
    let halo = parsed.acts.iter().any(|a| HALO_ACTS.iter().any(|h| h == a));
    let light = if villain {
        "villain"
    } else if halo {
        "halo"
    } else {
        "none"
    };
    FrameLight {
        light: light.to_string(),
        source: LIGHT_SOURCE.to_string(),
    }
}

fn explicit_drug_act(parsed: &Parsed) -> bool {
    parsed.acts.iter().any(|a| a == DRUG_SUPPLY_ACT)
}

fn condition_refusal(kind: &str) -> String {
    format!("{kind} is not an act and never authorizes custody")
}

fn drug_mapping_refusal(kind: &str) -> String {
    format!("{kind} does not map to a drug act or a junkie look")
}

fn custody_of(subject_kind: &str, parsed: &Parsed) -> (CustodyDecision, Vec<String>) {
    let mut refused = Vec::new();
    let warrant = parsed
        .acts
        .iter()
        .find(|a| CUSTODY_ACTS.iter().any(|c| c == a))
        .cloned();

    for c in &parsed.conditions {
        if NO_DRUG_INFERENCE.iter().any(|k| k == c) || is_pathway(c) {
            refused.push(drug_mapping_refusal(c));
        }
    }
    for pathway in &parsed.pathways {
        refused.push(format!(
            "{pathway} is not a crime and is not human garbage; help must name it"
        ));
    }
    if parsed.junkie_look_claimed {
        refused.push(
            "junkie look is refused; no housing, injury, and no medication are not a costume"
                .to_string(),
        );
    }
    if parsed.silhouette_hides_path {
        refused.push(
            "a single silhouette that hides which path it is is refused; name the condition"
                .to_string(),
        );
    }

    if !is_human(subject_kind) {
        let who = if subject_kind.is_empty() {
            "subject"
        } else {
            subject_kind
        };
        if NON_HUMAN.iter().any(|k| *k == subject_kind) {
            refused.push(format!(
                "{who} cannot be in a cell; custody is human-only"
            ));
        } else {
            refused.push(format!(
                "{who} is not a human subject; custody is human-only"
            ));
        }
        for c in &parsed.conditions {
            refused.push(condition_refusal(c));
        }
        return (
            CustodyDecision {
                custody: false,
                reason: "subject_not_human".to_string(),
            },
            refused,
        );
    }

    if let Some(act) = warrant {
        // Conditions may sit on the same human. They are not the warrant,
        // and they do not invent the drug act.
        return (
            CustodyDecision {
                custody: true,
                reason: act,
            },
            refused,
        );
    }

    if !parsed.conditions.is_empty() {
        for c in &parsed.conditions {
            refused.push(condition_refusal(c));
        }
        return (
            CustodyDecision {
                custody: false,
                reason: "condition_not_custody_ground".to_string(),
            },
            refused,
        );
    }

    (
        CustodyDecision {
            custody: false,
            reason: if parsed.silhouette_hides_path {
                "silhouette_hides_path".to_string()
            } else if parsed.junkie_look_claimed {
                "junkie_look_refused".to_string()
            } else {
                "no_warranting_act".to_string()
            },
        },
        refused,
    )
}

fn to_js<T: Serialize>(value: &T) -> JsValue {
    serde_wasm_bindgen::to_value(value).unwrap_or(JsValue::NULL)
}

/// Frame light from acts only. Poverty, wealth, and no-housing fields are ignored.
#[wasm_bindgen]
pub fn frame_light_from_acts(acts_json: &str) -> JsValue {
    let light = match parse_claims(acts_json) {
        Ok(parsed) => light_of(&parsed),
        Err(_) => FrameLight {
            light: "none".to_string(),
            source: LIGHT_SOURCE.to_string(),
        },
    };
    to_js(&light)
}

/// Custody only when the subject is human and a warranting act is present.
#[wasm_bindgen]
pub fn custody_from_acts(subject_kind: &str, acts_json: &str) -> JsValue {
    let decision = match parse_claims(acts_json) {
        Ok(parsed) => custody_of(subject_kind, &parsed).0,
        Err(_) => CustodyDecision {
            custody: false,
            reason: "claims_json_unreadable".to_string(),
        },
    };
    to_js(&decision)
}

/// Combined gate. Fails closed: conditions never open a cell, no housing never
/// infers a drug act or a junkie look, and a non-human subject is refused.
#[wasm_bindgen]
pub fn acts_frame_eval(subject_kind: &str, claims_json: &str) -> JsValue {
    let eval = match parse_claims(claims_json) {
        Ok(parsed) => {
            let light = light_of(&parsed);
            let drug = explicit_drug_act(&parsed);
            let (decision, refused) = custody_of(subject_kind, &parsed);
            let pathways = parsed.pathways.clone();
            ActsFrameEval {
                ok: refused.is_empty(),
                light: light.light,
                custody: decision.custody,
                reason: decision.reason,
                drug_inference: drug,
                look: "none".to_string(),
                pathways,
                refused,
            }
        }
        Err(reason) => ActsFrameEval {
            ok: false,
            light: "none".to_string(),
            custody: false,
            reason: reason.clone(),
            drug_inference: false,
            look: "none".to_string(),
            pathways: Vec::new(),
            refused: vec![reason],
        },
    };
    to_js(&eval)
}
