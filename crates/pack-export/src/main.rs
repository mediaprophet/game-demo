//! Native pack exporter — repacks generated .10d assets, textures, and the
//! vibe-authored semantic section into verified HMC bundles.
//!
//! Replaces `scripts/export-game-assets.mjs` for the pack/manifest path so
//! regeneration does not need Node. The `.vibe` fixture remains the single
//! authored source of taxon identity and dwc/accrual statements — this tool
//! evaluates it with the real interpreter and `PoetSnapshot` host (whose
//! `hash_iri` is `generate_60bit_token`, the same seal the WASM build uses).
//! `.10d` scene regeneration itself is a WASM step (scene_build) — this
//! tool repacks the checked-in generated artifacts.
//!
//! Usage: `cargo run -p pack-export` (from the workspace root). Writes
//! `web/assets/*-manifest.json` and `web/assets/maslows-challenge-*.hmc`.

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
use qualia_core_db::bundle::{BundleReader, BundleWriter};
use qualia_core_db::lexicon::generate_60bit_token;
use qualia_core_db::q42_volume::{StreamingQ42VolumeWriter, Q42_MAGIC};
use qualia_core_db::query::shacl_compiler::{
    validate_shacl_property, ShaclConstraint, ShaclDatatype,
};
use qualia_core_db::resolver::INLINE_TAG_INTEGER;
use qualia_core_db::{NQuin, QUINS_PER_BLOCK};
use serde_json::{json, Map, Value as Json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use vibe::Value;

const MAX_DWC_QUINS: usize = 9;
const MAX_STATEMENTS: usize = 65_536;

type DynErr = Box<dyn std::error::Error>;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex(&hasher.finalize())
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn eval_fn(program: &vibe::Program, name: &str, args: Vec<Value>) -> Result<Value, DynErr> {
    let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
    let mut env = vibe::Env::default();
    vibe::eval_function(program, name, args, &mut host, &mut env)
        .map_err(|e| format!("vibe eval {name}: {e}").into())
}

fn text(record: &BTreeMap<String, Value>, field: &str) -> Result<String, DynErr> {
    match record.get(field) {
        Some(Value::String(s)) if s.len() <= 512 => Ok(s.clone()),
        _ => Err(format!("taxon record field {field} missing").into()),
    }
}

struct Binding {
    gbif_key: u64,
    iri: String,
    rank: String,
    asset_family: String,
    record: Json,
}

fn binding_for(program: &vibe::Program, key: &str) -> Result<Option<Binding>, DynErr> {
    let value = eval_fn(program, "taxon_for", vec![Value::String(key.to_owned())])?;
    let Value::Record(record) = value else {
        return Err("taxon_for did not return a record".into());
    };
    let Some(Value::F64(bound)) = record.get("bound") else {
        return Err("taxon bound flag missing".into());
    };
    if *bound == 0.0 {
        return Ok(None);
    }
    let Some(Value::F64(gbif_key)) = record.get("gbifKey") else {
        return Err("taxon gbifKey missing".into());
    };
    if !gbif_key.is_finite() || *gbif_key <= 0.0 || *gbif_key > 9_007_199_254_740_991.0
        || gbif_key.fract() != 0.0
    {
        return Err("taxon gbifKey exceeds bounds".into());
    }
    let iri = text(&record, "iri")?;
    if !iri.starts_with("https://www.gbif.org/species/") {
        return Err("taxon iri outside bounds".into());
    }
    let record_json = json!({
        "gbifKey": *gbif_key as u64,
        "iri": iri,
        "rank": text(&record, "rank")?,
        "scientificName": text(&record, "scientificName")?,
        "commonName": text(&record, "commonName")?,
        "assetFamily": text(&record, "assetFamily")?,
        "rankNote": text(&record, "rankNote")?,
        "accrualNote": text(&record, "accrualNote")?,
        "maturityIri": text(&record, "maturityIri")?,
        "irreversibilityIri": text(&record, "irreversibilityIri")?,
    });
    Ok(Some(Binding {
        gbif_key: *gbif_key as u64,
        iri,
        rank: text(&record, "rank")?,
        asset_family: text(&record, "assetFamily")?,
        record: record_json,
    }))
}

struct Statement {
    s: String,
    p: String,
    o: String,
    integer: bool,
    quin: NQuin,
}

fn statements_for(
    program: &vibe::Program,
    key: &str,
    asset_iri: &str,
    gbif_key: u64,
    taxon_iri: &str,
) -> Result<Vec<Statement>, DynErr> {
    let value = eval_fn(
        program,
        "dwc_statements_for",
        vec![
            Value::String(key.to_owned()),
            Value::String(asset_iri.to_owned()),
        ],
    )?;
    let Value::List(items) = value else {
        return Err("dwc_statements_for did not return a list".into());
    };
    if items.len() + 1 > MAX_DWC_QUINS {
        return Err("dwc statement count exceeds bound".into());
    }
    let mut out = Vec::with_capacity(items.len() + 1);
    for item in &items {
        let Value::Record(record) = item else {
            return Err("dwc statement is not a record".into());
        };
        let Some(Value::QuinRef(q)) = record.get("quin") else {
            return Err("dwc statement quin is not sealed".into());
        };
        let f = q.raw_fields();
        out.push(Statement {
            s: text(record, "s")?,
            p: text(record, "p")?,
            o: text(record, "o")?,
            integer: false,
            quin: NQuin {
                subject: f[0],
                predicate: f[1],
                object: f[2],
                context: f[3],
                metadata: f[4],
                parity: f[5],
            },
        });
    }
    // Host-packed integer quin — INLINE_TAG_INTEGER|key cannot be formed in
    // script (f64 loses the 0b001<<60 tag above 2^53).
    let subject = generate_60bit_token(taxon_iri.as_bytes());
    let predicate = generate_60bit_token(b"urn:qualia:gbif:taxonKey");
    let object = INLINE_TAG_INTEGER | gbif_key;
    out.push(Statement {
        s: taxon_iri.to_owned(),
        p: "urn:qualia:gbif:taxonKey".to_owned(),
        o: gbif_key.to_string(),
        integer: true,
        quin: NQuin {
            subject,
            predicate,
            object,
            context: 0,
            metadata: 0,
            parity: NQuin::calculate_parity(subject, predicate, object, 0, 0),
        },
    });
    Ok(out)
}

/// Same asset→renderKey rule as the exporter: flora assets prefix-match the
/// binding's assetFamily; fauna instance ids embed the renderKey token.
fn taxon_for_asset<'a>(
    asset_id: &str,
    bindings: &'a BTreeMap<String, Binding>,
) -> Option<(&'a str, &'a Binding)> {
    if asset_id.starts_with("rc:asset/flora/") {
        return bindings
            .iter()
            .find(|(_, b)| asset_id.starts_with(&format!("{}/", b.asset_family)))
            .map(|(k, b)| (k.as_str(), b));
    }
    if asset_id.starts_with("rc:asset/fauna/") {
        return bindings
            .iter()
            .find(|(k, _)| asset_id.contains(k.as_str()))
            .map(|(k, b)| (k.as_str(), b));
    }
    None
}

/// SHACL shape check over the emitted quins — runs on the real
/// `validate_shacl_property` engine surface, same as the browser's
/// SemanticSlice. Every bound taxon node needs the four dwc/rdf
/// properties plus the packed-integer GBIF key; every bound asset
/// needs its foaf:depicts edge.
fn validate_semantic_shape(
    stmts: &[&Statement],
    bound_asset_ids: &[String],
    bindings: &BTreeMap<String, Binding>,
) -> Result<(), DynErr> {
    const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    let quins: Vec<NQuin> = stmts.iter().map(|st| st.quin).collect();
    let one = [ShaclConstraint::MinCount(1)];
    for binding in bindings.values() {
        let taxon = generate_60bit_token(binding.iri.as_bytes());
        for prop in [
            RDF_TYPE,
            "http://rs.tdwg.org/dwc/terms/taxonID",
            "http://rs.tdwg.org/dwc/terms/scientificName",
            "http://rs.tdwg.org/dwc/terms/taxonRank",
        ] {
            if !validate_shacl_property(&quins, taxon, generate_60bit_token(prop.as_bytes()), &one) {
                return Err(format!("taxon {} missing {prop}", binding.iri).into());
            }
        }
        if !validate_shacl_property(
            &quins,
            taxon,
            generate_60bit_token(b"urn:qualia:gbif:taxonKey"),
            &[
                ShaclConstraint::MinCount(1),
                ShaclConstraint::Datatype(ShaclDatatype::Integer),
            ],
        ) {
            return Err(format!("taxon {} missing integer taxonKey", binding.iri).into());
        }
    }
    let depicts = generate_60bit_token(b"http://xmlns.com/foaf/0.1/depicts");
    for id in bound_asset_ids {
        let asset = generate_60bit_token(id.as_bytes());
        if !validate_shacl_property(&quins, asset, depicts, &one) {
            return Err(format!("asset {id} missing foaf:depicts edge").into());
        }
    }
    Ok(())
}

fn valid_entry_key(key: &str) -> bool {
    if key.contains("..") {
        return false;
    }
    if key.starts_with("10d/") && key.ends_with(".10d") {
        return true;
    }
    if let Some(hex_part) = key.strip_prefix("textures/sha256/") {
        return hex_part.len() == 64
            && hex_part
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase());
    }
    if let Some(name) = key.strip_prefix("sem/") {
        return name.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'.'
        }) && name.ends_with(".q42");
    }
    false
}

fn run_style(
    root: &Path,
    program: &vibe::Program,
    slug: &str,
    bindings: &BTreeMap<String, Binding>,
) -> Result<(), DynErr> {
    let style_root = if slug.is_empty() {
        root.join("assets/generated")
    } else {
        root.join("assets/generated").join(slug)
    };
    let manifest_name = if slug.is_empty() {
        "manifest.json".to_owned()
    } else {
        format!("{slug}-manifest.json")
    };
    let pack_name = if slug.is_empty() {
        "maslows-challenge-scenes.hmc".to_owned()
    } else {
        format!("maslows-challenge-{slug}.hmc")
    };
    let manifest_path = root.join("web/assets").join(&manifest_name);
    let manifest_text = std::fs::read_to_string(&manifest_path)?;
    let mut manifest: Json = serde_json::from_str(&manifest_text)?;
    let assets = manifest
        .get("assets")
        .and_then(Json::as_array)
        .ok_or("manifest missing assets")?
        .clone();
    let textures = manifest
        .get("textures")
        .and_then(Json::as_array)
        .cloned()
        .unwrap_or_default();

    // Collect deduped semantic statements; stamp per-asset taxon links.
    let mut stmt_map: BTreeMap<[u64; 5], Statement> = BTreeMap::new();
    let mut stamped_assets = Vec::with_capacity(assets.len());
    let mut bound_asset_ids: Vec<String> = Vec::new();
    for asset in &assets {
        let mut asset = asset.clone();
        let id = asset
            .get("id")
            .and_then(Json::as_str)
            .ok_or("asset missing id")?
            .to_owned();
        if let Some((key, binding)) = taxon_for_asset(&id, bindings) {
            bound_asset_ids.push(id.clone());
            for st in statements_for(program, key, &id, binding.gbif_key, &binding.iri)? {
                stmt_map
                    .entry([
                        st.quin.subject,
                        st.quin.predicate,
                        st.quin.object,
                        st.quin.context,
                        st.quin.metadata,
                    ])
                    .or_insert(st);
            }
            asset["taxon"] = json!({
                "gbifKey": binding.gbif_key,
                "iri": binding.iri,
                "rank": binding.rank,
            });
        }
        stamped_assets.push(asset);
    }
    if stmt_map.len() > MAX_STATEMENTS {
        return Err("semantic statement count exceeds bound".into());
    }
    let stmts: Vec<&Statement> = stmt_map.values().collect();
    validate_semantic_shape(&stmts, &bound_asset_ids, bindings)?;

    // Lexicon: hash→term map; integer-typed objects carry no hash. It feeds
    // the `.q42` volume's embedded Q42LEX section.
    let mut lex_terms: HashMap<u64, String> = HashMap::new();
    for st in &stmts {
        for (hash, term) in [
            (st.quin.subject, &st.s),
            (st.quin.predicate, &st.p),
            (st.quin.object, &st.o),
        ] {
            if st.integer && hash == st.quin.object {
                continue;
            }
            match lex_terms.get(&hash) {
                Some(existing) if existing != term => {
                    return Err(format!("lexicon hash collision at {hash:#x}").into());
                }
                Some(_) => {}
                None => {
                    lex_terms.insert(hash, term.clone());
                }
            }
        }
    }
    // Canonical ontology package: a unified v4 .q42 volume (object-sorted
    // SuperBlocks, embedded lexicon, BIDX + field-range + postings indexes,
    // Merkle root) inside the HMC. StreamingQ42VolumeWriter is deterministic
    // — assertion_timestamp is fixed at 0 and there are no wall-clock DAG
    // commits, so identical input gives identical bytes. The browser decodes
    // this volume through `Q42RangeVolume` over the verified pack bytes.
    let q42_bytes = if stmts.is_empty() {
        Vec::new()
    } else {
        let mut sorted: Vec<NQuin> = stmts.iter().map(|st| st.quin).collect();
        sorted.sort_unstable_by_key(|q| q.object);
        let mut writer = StreamingQ42VolumeWriter::new(&lex_terms)
            .map_err(|e| format!("q42 writer: {e}"))?;
        writer.declare_permissive_commons();
        for (seq, chunk) in sorted.chunks(QUINS_PER_BLOCK).enumerate() {
            writer
                .push_block(seq as u64, chunk)
                .map_err(|e| format!("q42 push_block: {e}"))?;
        }
        let temp = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
        writer
            .finish(temp.path())
            .map_err(|e| format!("q42 finish: {e}"))?;
        let bytes = std::fs::read(temp.path())?;
        if !bytes.starts_with(&Q42_MAGIC) {
            return Err("q42 volume missing magic".into());
        }
        bytes
    };

    let semantic_files: Vec<(String, Vec<u8>)> = if stmts.is_empty() {
        Vec::new()
    } else {
        vec![("sem/taxonomy.q42".to_owned(), q42_bytes)]
    };
    let semantics: Vec<Json> = semantic_files
        .iter()
        .map(|(key, bytes)| {
            json!({
                "key": key,
                "quins": Json::from(stmts.len()),
                "bytes": bytes.len(),
                "sha256": sha256_hex(bytes),
            })
        })
        .collect();

    // Update manifest: stamped assets, rebuilt binding map, semantic section.
    let mut taxon_bindings = Map::new();
    for (key, binding) in bindings {
        taxon_bindings.insert(key.clone(), binding.record.clone());
    }
    let obj = manifest.as_object_mut().ok_or("manifest is not an object")?;
    obj.insert("assets".into(), Json::Array(stamped_assets));
    obj.insert("taxonBindings".into(), Json::Object(taxon_bindings));
    obj.insert("semantics".into(), Json::Array(semantics));
    obj.insert("packer".into(), Json::from("crates/pack-export"));
    let manifest_text = serde_json::to_string_pretty(&manifest)? + "\n";

    // Pack: manifest + .10d assets + texture files + semantic entries.
    let mut writer = BundleWriter::new();
    writer
        .add_file("manifest.json", "manifest", manifest_text.as_bytes().to_vec(), None)
        .map_err(|e| format!("{e:?}"))?;
    let mut entry_count = 0usize;
    for asset in manifest["assets"].as_array().ok_or("assets lost")? {
        let file = asset
            .get("file")
            .and_then(Json::as_str)
            .ok_or("asset missing file")?;
        if !valid_entry_key(file) {
            return Err(format!("invalid asset entry key {file}").into());
        }
        let bytes = std::fs::read(style_root.join(file))?;
        if let Some(expected) = asset.get("sha256").and_then(Json::as_str) {
            if sha256_hex(&bytes) != expected {
                return Err(format!("asset {file} sha256 does not match manifest").into());
            }
        }
        writer
            .add_file(file, "10d", bytes, None)
            .map_err(|e| format!("{e:?}"))?;
        entry_count += 1;
    }
    let mut texture_files: Vec<(String, Option<String>)> = Vec::new();
    for tex in &textures {
        let file = tex
            .get("file")
            .and_then(Json::as_str)
            .ok_or("texture missing file")?
            .to_owned();
        if texture_files.iter().all(|(f, _)| f != &file) {
            texture_files.push((
                file,
                tex.get("sha256")
                    .and_then(Json::as_str)
                    .map(str::to_owned),
            ));
        }
    }
    for (file, expected) in &texture_files {
        if !valid_entry_key(file) {
            return Err(format!("invalid texture entry key {file}").into());
        }
        let bytes = std::fs::read(style_root.join(file))?;
        if let Some(expected) = expected {
            if sha256_hex(&bytes) != *expected {
                return Err(format!("texture {file} sha256 does not match manifest").into());
            }
        }
        writer
            .add_file(file, "image/png", bytes, None)
            .map_err(|e| format!("{e:?}"))?;
        entry_count += 1;
    }
    for (key, bytes) in &semantic_files {
        if !valid_entry_key(key) {
            return Err(format!("invalid semantic entry key {key}").into());
        }
        if !bytes.starts_with(&Q42_MAGIC) {
            return Err("semantic q42 volume missing magic".into());
        }
        writer
            .add_file(key, "q42", bytes.clone(), None)
            .map_err(|e| format!("{e:?}"))?;
        entry_count += 1;
    }

    let pack = writer.build().map_err(|e| e.to_string())?;
    let reader = BundleReader::parse(&pack).map_err(|e| e.to_string())?;
    if reader
        .entries()
        .iter()
        .any(|entry| !reader.verify_entry(&entry.key))
    {
        return Err("pack entry digest verification failed".into());
    }
    if reader.entries().len() != entry_count + 1 {
        return Err(format!(
            "pack entry count {} != expected {}",
            reader.entries().len(),
            entry_count + 1
        )
        .into());
    }

    std::fs::write(&manifest_path, &manifest_text)?;
    std::fs::write(root.join("web/assets").join(&pack_name), &pack)?;
    println!(
        "packed {pack_name}: {} assets, {} textures, {} semantic quins, {} bytes",
        assets.len(),
        texture_files.len(),
        stmts.len(),
        pack.len()
    );
    Ok(())
}

fn parse_styles() -> Result<Vec<String>, DynErr> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.iter().all(|a| a == "--all") {
        return Ok(["".to_owned(), "earthlight".to_owned(), "community".to_owned()].to_vec());
    }
    let mut styles = Vec::new();
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--style" => {
                let slug = it.next().ok_or("--style needs a value")?;
                if slug != "earthlight" && slug != "community" && !slug.is_empty() {
                    return Err(format!("unknown style {slug}").into());
                }
                styles.push(slug.clone());
            }
            "--all" => {}
            other => return Err(format!("unknown argument {other}").into()),
        }
    }
    if styles.is_empty() {
        return Err("no styles selected".into());
    }
    Ok(styles)
}

    pub fn run() -> Result<(), DynErr> {
        let root = repo_root();
        let styles = parse_styles()?;
        let program = vibe::load_program(include_str!("../../../web/fixtures/taxon-bindings.vibe"))
            .map_err(|e| format!("taxon-bindings.vibe: {e}"))?;

        let profile_path = root.join("assets/ecology/profiles/se-corridor-reference.json");
        let profile: Json = serde_json::from_str(&std::fs::read_to_string(&profile_path)?)?;
        let mut keys: Vec<String> = Vec::new();
        for field in ["faunaSelection", "floraSelection"] {
            for key in profile["render"][field].as_array().into_iter().flatten() {
                if let Some(k) = key.as_str() {
                    keys.push(k.to_owned());
                }
            }
        }
        let mut bindings: BTreeMap<String, Binding> = BTreeMap::new();
        for key in &keys {
            if let Some(binding) = binding_for(&program, key)? {
                bindings.insert(key.clone(), binding);
            }
        }
        println!("taxon bindings resolved: {}", bindings.len());

        for slug in &styles {
            run_style(&root, &program, slug, &bindings)?;
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

    fn program() -> vibe::Program {
        vibe::load_program(include_str!("../../../web/fixtures/taxon-bindings.vibe"))
            .expect("taxon-bindings.vibe parses")
    }

    fn valid_parity(q: &NQuin) -> bool {
        q.parity
            == NQuin::calculate_parity(q.subject, q.predicate, q.object, q.context, q.metadata)
    }

    #[test]
    fn bound_taxon_emits_bounded_sealed_quins() {
        let program = program();
        let binding = binding_for(&program, "kangaroo")
            .expect("eval")
            .expect("kangaroo is bound");
        assert_eq!(binding.gbif_key, 9372);
        let stmts = statements_for(
            &program,
            "kangaroo",
            "rc:asset/fauna/kangaroo-0",
            binding.gbif_key,
            &binding.iri,
        )
        .expect("dwc statements");
        assert!(stmts.len() <= MAX_DWC_QUINS);
        assert!(stmts.iter().all(|st| valid_parity(&st.quin)));
        // The host-appended quin carries the inline-integer GBIF key.
        let int_quin = stmts.last().expect("integer quin");
        assert!(int_quin.integer);
        assert_eq!(int_quin.quin.object, INLINE_TAG_INTEGER | 9372);
    }

    #[test]
    fn unbound_render_key_stays_data_only() {
        let program = program();
        assert!(binding_for(&program, "unbound-key").expect("eval").is_none());
    }

    #[test]
    fn tampered_quin_fails_parity() {
        let program = program();
        let binding = binding_for(&program, "eucalypt-gum")
            .expect("eval")
            .expect("eucalypt-gum is bound");
        let stmts = statements_for(
            &program,
            "eucalypt-gum",
            "rc:asset/flora/eucalypt-gum-0",
            binding.gbif_key,
            &binding.iri,
        )
        .expect("dwc statements");
        assert_eq!(stmts.len(), MAX_DWC_QUINS);
        let mut tampered = stmts[0].quin;
        tampered.object ^= 1;
        assert!(!valid_parity(&tampered));
    }
}
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    imp::run()
}
