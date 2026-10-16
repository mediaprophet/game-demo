//! Variant Matrix & Entity Family Manifest WASM Bindings (WP-1.2).
//!
//! Exposes QualiaDB's `TwinFamilyManifest` / `EntityFamilyManifest` and $O(1)$
//! `resolve_variant` pipeline to the game shell and runtime.

use qualia_core_db::bundle::variant_matrix::{
    BiologicalPhenotype, LodTier, ModelVariantDescriptor, StructuralState, TwinFamilyManifest,
    FAMILY_MANIFEST_ENTRY_KEY, FAMILY_MANIFEST_LEGACY_KEY,
};
use qualia_core_db::bundle::BundleReader;
use qualia_core_db::entity_view::entity_ontologies::LifecycleStage;
use wasm_bindgen::prelude::*;

/// Browser-resident Entity Family Manifest wrapping QualiaDB's `TwinFamilyManifest`.
#[wasm_bindgen]
pub struct EntityFamilyManifestWrapper {
    inner: TwinFamilyManifest,
}

#[wasm_bindgen]
impl EntityFamilyManifestWrapper {
    /// Parse a family manifest directly from raw CBOR bytes.
    #[wasm_bindgen(constructor)]
    pub fn from_cbor(bytes: &[u8]) -> Result<EntityFamilyManifestWrapper, JsValue> {
        let manifest = TwinFamilyManifest::decode_from_cbor(bytes)
            .map_err(|e| JsValue::from_str(&format!("family manifest decode error: {e}")))?;
        Ok(Self { inner: manifest })
    }

    /// Extract and parse a family manifest from a verified HMC bundle pack.
    pub fn from_hmc_pack(pack: &[u8]) -> Result<Option<EntityFamilyManifestWrapper>, JsValue> {
        let reader = BundleReader::parse(pack)
            .map_err(|e| JsValue::from_str(&format!("HMC bundle error: {e}")))?;
        match reader.get_family_manifest() {
            Ok(Some(manifest)) => Ok(Some(Self { inner: manifest })),
            Ok(None) => Ok(None),
            Err(e) => Err(JsValue::from_str(&format!("HMC family manifest read: {e}"))),
        }
    }

    /// Family identifier URN or IRI (e.g. `"urn:taxon:phascolarctos_cinereus"`).
    pub fn family_id(&self) -> String {
        self.inner.family_id.clone()
    }

    /// Common display name (e.g. `"Koala"`).
    pub fn common_name(&self) -> String {
        self.inner.common_name.clone()
    }

    /// Classification (`"fauna"`, `"flora"`, `"funga"`, `"machinery"`, `"vehicle"`).
    pub fn classification(&self) -> String {
        self.inner.classification.clone()
    }

    /// Default variant ID.
    pub fn default_variant(&self) -> String {
        self.inner.default_variant.clone()
    }

    /// Number of declared model variants.
    pub fn variant_count(&self) -> u32 {
        self.inner.variants.len() as u32
    }

    /// Resolve a model variant using QualiaDB's principled 4-tier fallback:
    /// 1. Exact match on `(phenotype, stage, lod, state)`
    /// 2. Match on `(phenotype, stage, state)` with closest LOD
    /// 3. Fallback to `Monomorphic` if sex-specific variant missing
    /// 4. Fallback to default variant
    pub fn resolve_variant(
        &self,
        phenotype_str: &str,
        stage_str: &str,
        lod_tier: u8,
        state_str: &str,
    ) -> Result<JsValue, JsValue> {
        let phenotype = parse_phenotype(phenotype_str);
        let stage = parse_lifecycle_stage(stage_str);
        let lod = parse_lod_tier(lod_tier);
        let state = parse_structural_state(state_str);

        let descriptor = self.inner.resolve_variant(phenotype, stage, lod, state);
        match descriptor {
            Some(desc) => variant_descriptor_to_js(desc),
            None => Ok(JsValue::NULL),
        }
    }

    /// Resolve variant with distance-based LOD selection:
    /// - `< 15m`: Lod0
    /// - `15m .. 45m`: Lod1
    /// - `45m .. 90m`: Lod2
    /// - `90m .. 180m`: Lod3
    /// - `>= 180m`: Impostor
    pub fn resolve_variant_for_distance(
        &self,
        phenotype_str: &str,
        stage_str: &str,
        distance_meters: f64,
        state_str: &str,
    ) -> Result<JsValue, JsValue> {
        let lod = lod_for_distance(distance_meters);
        self.resolve_variant(phenotype_str, stage_str, lod as u8, state_str)
    }

    /// List all variant descriptors in the manifest as a JS array of objects.
    pub fn list_variants(&self) -> Result<js_sys::Array, JsValue> {
        let arr = js_sys::Array::new();
        for variant in &self.inner.variants {
            let obj = variant_descriptor_to_js(variant)?;
            arr.push(&obj);
        }
        Ok(arr)
    }
}

/// Inspect whether an HMC pack carries an Entity Family Manifest.
#[wasm_bindgen]
pub fn hmc_has_family_manifest(pack: &[u8]) -> Result<bool, JsValue> {
    let reader = BundleReader::parse(pack)
        .map_err(|e| JsValue::from_str(&format!("HMC bundle error: {e}")))?;
    Ok(reader.get(FAMILY_MANIFEST_ENTRY_KEY).is_some()
        || reader.get(FAMILY_MANIFEST_LEGACY_KEY).is_some())
}

/// Read and resolve a variant directly from an HMC pack in one zero-overhead call.
#[wasm_bindgen]
pub fn hmc_resolve_variant_for_distance(
    pack: &[u8],
    phenotype: &str,
    stage: &str,
    distance_meters: f64,
    state: &str,
) -> Result<JsValue, JsValue> {
    let wrapper = EntityFamilyManifestWrapper::from_hmc_pack(pack)?;
    match wrapper {
        Some(w) => w.resolve_variant_for_distance(phenotype, stage, distance_meters, state),
        None => Ok(JsValue::NULL),
    }
}

fn parse_phenotype(s: &str) -> BiologicalPhenotype {
    match s.trim().to_ascii_lowercase().as_str() {
        "male" => BiologicalPhenotype::Male,
        "female" => BiologicalPhenotype::Female,
        "neuter" => BiologicalPhenotype::Neuter,
        "hermaphroditic" => BiologicalPhenotype::Hermaphroditic,
        "unspecified" => BiologicalPhenotype::Unspecified,
        _ => BiologicalPhenotype::Monomorphic,
    }
}

fn parse_lifecycle_stage(s: &str) -> LifecycleStage {
    match s.trim().to_ascii_lowercase().as_str() {
        "seed" | "spore" | "seed_or_spore" => LifecycleStage::SeedOrSpore,
        "juvenile" | "larva" | "sapling" => LifecycleStage::Juvenile,
        "senescent" | "aged" => LifecycleStage::Senescent,
        "deceased" | "detritus" => LifecycleStage::Deceased,
        "manufactured" => LifecycleStage::Manufactured,
        "operational" => LifecycleStage::Operational,
        "degraded" => LifecycleStage::Degraded,
        "decommissioned" => LifecycleStage::Decommissioned,
        _ => LifecycleStage::Adult,
    }
}

fn parse_lod_tier(raw: u8) -> LodTier {
    match raw {
        0 => LodTier::Lod0,
        1 => LodTier::Lod1,
        2 => LodTier::Lod2,
        3 => LodTier::Lod3,
        _ => LodTier::Impostor,
    }
}

fn parse_structural_state(s: &str) -> StructuralState {
    match s.trim().to_ascii_lowercase().as_str() {
        "weathered" => StructuralState::Weathered,
        "damaged" => StructuralState::Damaged,
        "fractured" => StructuralState::Fractured,
        _ => StructuralState::Pristine,
    }
}

fn lod_for_distance(distance_meters: f64) -> LodTier {
    if !distance_meters.is_finite() || distance_meters < 15.0 {
        LodTier::Lod0
    } else if distance_meters < 45.0 {
        LodTier::Lod1
    } else if distance_meters < 90.0 {
        LodTier::Lod2
    } else if distance_meters < 180.0 {
        LodTier::Lod3
    } else {
        LodTier::Impostor
    }
}

fn variant_descriptor_to_js(desc: &ModelVariantDescriptor) -> Result<JsValue, JsValue> {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &"variant_id".into(),
        &JsValue::from_str(&desc.variant_id),
    )?;
    js_sys::Reflect::set(
        &obj,
        &"asset_key".into(),
        &JsValue::from_str(&desc.asset_key),
    )?;
    if let Some(ref ont_key) = desc.entity_ontology_key {
        js_sys::Reflect::set(
            &obj,
            &"entity_ontology_key".into(),
            &JsValue::from_str(ont_key),
        )?;
    } else {
        js_sys::Reflect::set(&obj, &"entity_ontology_key".into(), &JsValue::NULL)?;
    }
    if let Some(seed) = desc.procedural_seed {
        js_sys::Reflect::set(
            &obj,
            &"procedural_seed".into(),
            &JsValue::from_f64(seed as f64),
        )?;
    } else {
        js_sys::Reflect::set(&obj, &"procedural_seed".into(), &JsValue::NULL)?;
    }
    js_sys::Reflect::set(
        &obj,
        &"phenotype".into(),
        &JsValue::from_str(&format!("{:?}", desc.phenotype).to_lowercase()),
    )?;
    js_sys::Reflect::set(
        &obj,
        &"stage".into(),
        &JsValue::from_str(&format!("{:?}", desc.stage).to_lowercase()),
    )?;
    js_sys::Reflect::set(
        &obj,
        &"lod".into(),
        &JsValue::from_f64(desc.lod as u32 as f64),
    )?;
    js_sys::Reflect::set(
        &obj,
        &"state".into(),
        &JsValue::from_str(&format!("{:?}", desc.state).to_lowercase()),
    )?;
    Ok(obj.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_helpers() {
        assert_eq!(parse_phenotype("male"), BiologicalPhenotype::Male);
        assert_eq!(parse_phenotype("FEMALE"), BiologicalPhenotype::Female);
        assert_eq!(
            parse_phenotype("anything_else"),
            BiologicalPhenotype::Monomorphic
        );

        assert_eq!(parse_lifecycle_stage("seed"), LifecycleStage::SeedOrSpore);
        assert_eq!(parse_lifecycle_stage("juvenile"), LifecycleStage::Juvenile);
        assert_eq!(parse_lifecycle_stage("adult"), LifecycleStage::Adult);
        assert_eq!(
            parse_lifecycle_stage("senescent"),
            LifecycleStage::Senescent
        );
        assert_eq!(parse_lifecycle_stage("deceased"), LifecycleStage::Deceased);

        assert_eq!(parse_lod_tier(0), LodTier::Lod0);
        assert_eq!(parse_lod_tier(1), LodTier::Lod1);
        assert_eq!(parse_lod_tier(2), LodTier::Lod2);
        assert_eq!(parse_lod_tier(3), LodTier::Lod3);
        assert_eq!(parse_lod_tier(4), LodTier::Impostor);

        assert_eq!(
            parse_structural_state("pristine"),
            StructuralState::Pristine
        );
        assert_eq!(
            parse_structural_state("weathered"),
            StructuralState::Weathered
        );
        assert_eq!(parse_structural_state("damaged"), StructuralState::Damaged);
    }

    #[test]
    fn test_lod_for_distance() {
        assert_eq!(lod_for_distance(5.0), LodTier::Lod0);
        assert_eq!(lod_for_distance(14.9), LodTier::Lod0);
        assert_eq!(lod_for_distance(15.0), LodTier::Lod1);
        assert_eq!(lod_for_distance(44.9), LodTier::Lod1);
        assert_eq!(lod_for_distance(45.0), LodTier::Lod2);
        assert_eq!(lod_for_distance(89.9), LodTier::Lod2);
        assert_eq!(lod_for_distance(90.0), LodTier::Lod3);
        assert_eq!(lod_for_distance(179.9), LodTier::Lod3);
        assert_eq!(lod_for_distance(180.0), LodTier::Impostor);
        assert_eq!(lod_for_distance(500.0), LodTier::Impostor);
    }

    #[test]
    fn test_family_manifest_resolution_pipeline() {
        let mut manifest = TwinFamilyManifest::new(
            "urn:taxon:phascolarctos_cinereus",
            "Koala",
            "fauna",
            "mono_adult_lod0",
        );

        manifest.add_variant(ModelVariantDescriptor {
            variant_id: "mono_adult_lod0".into(),
            phenotype: BiologicalPhenotype::Monomorphic,
            stage: LifecycleStage::Adult,
            lod: LodTier::Lod0,
            state: StructuralState::Pristine,
            asset_key: "10d/koala_adult_lod0.10d".into(),
            entity_ontology_key: Some("ontology/koala.q42".into()),
            procedural_seed: Some(42),
        });

        manifest.add_variant(ModelVariantDescriptor {
            variant_id: "mono_adult_lod2".into(),
            phenotype: BiologicalPhenotype::Monomorphic,
            stage: LifecycleStage::Adult,
            lod: LodTier::Lod2,
            state: StructuralState::Pristine,
            asset_key: "10d/koala_adult_lod2.10d".into(),
            entity_ontology_key: Some("ontology/koala.q42".into()),
            procedural_seed: Some(42),
        });

        manifest.add_variant(ModelVariantDescriptor {
            variant_id: "male_adult_lod0".into(),
            phenotype: BiologicalPhenotype::Male,
            stage: LifecycleStage::Adult,
            lod: LodTier::Lod0,
            state: StructuralState::Pristine,
            asset_key: "10d/koala_male_adult_lod0.10d".into(),
            entity_ontology_key: None,
            procedural_seed: None,
        });

        // 1. Exact match
        let exact = manifest.resolve_variant(
            BiologicalPhenotype::Male,
            LifecycleStage::Adult,
            LodTier::Lod0,
            StructuralState::Pristine,
        );
        assert!(exact.is_some());
        assert_eq!(exact.unwrap().variant_id, "male_adult_lod0");

        // 2. Female requested: no female variant, falls back to Monomorphic adult LOD0
        let female_fallback = manifest.resolve_variant(
            BiologicalPhenotype::Female,
            LifecycleStage::Adult,
            LodTier::Lod0,
            StructuralState::Pristine,
        );
        assert!(female_fallback.is_some());
        assert_eq!(female_fallback.unwrap().variant_id, "mono_adult_lod0");

        // 3. LOD1 requested: closest available is LOD0 or LOD2
        let lod1_req = manifest.resolve_variant(
            BiologicalPhenotype::Monomorphic,
            LifecycleStage::Adult,
            LodTier::Lod1,
            StructuralState::Pristine,
        );
        assert!(lod1_req.is_some());
        // Distances: Lod0 distance is 1, Lod2 distance is 1. One of them will be returned.
        assert!(
            lod1_req.unwrap().variant_id == "mono_adult_lod0"
                || lod1_req.unwrap().variant_id == "mono_adult_lod2"
        );

        // 4. CBOR roundtrip
        let bytes = manifest.encode_to_cbor().unwrap();
        let decoded = TwinFamilyManifest::decode_from_cbor(&bytes).unwrap();
        assert_eq!(decoded.family_id, manifest.family_id);
        assert_eq!(decoded.variants.len(), 3);
    }
}
