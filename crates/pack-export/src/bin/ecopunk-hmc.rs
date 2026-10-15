//! Bundle one ecopunk model set, its evidence graph, and animation manifest.
use qualia_core_db::bundle::{BundleReader, BundleWriter};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};

fn sha(bytes: &[u8]) -> String { Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect() }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env::args().nth(1).ok_or("usage: ecopunk-hmc MODEL_FOLDER [OUTPUT.hmc]")?);
    let out = env::args().nth(2).map(PathBuf::from).unwrap_or_else(|| root.join("corymbia-aparrerinja.hmc"));
    let manifest_bytes = fs::read(root.join("model-set.json"))?;
    let manifest: Value = serde_json::from_slice(&manifest_bytes)?;
    let mut writer = BundleWriter::new();
    writer.add_file("manifest.json", "manifest", manifest_bytes, None)?;
    for model in manifest["models"].as_array().ok_or("missing models")? {
        for component in model["components"].as_array().ok_or("missing components")? {
            let key = component["file"].as_str().ok_or("missing component file")?;
            if !key.starts_with("10d/") || key.contains("..") { return Err("invalid component path".into()); }
            let bytes = fs::read(root.join(key))?;
            if sha(&bytes) != component["sha256"].as_str().ok_or("missing component hash")? {
                return Err(format!("digest mismatch: {key}").into());
            }
            writer.add_file(key, "10d", bytes, None)?;
        }
    }
    for (field, digest, kind) in [
        ("source", "sourceSha256", "application/json"),
        ("volume", "volumeSha256", "q42"),
    ] {
        let key = manifest["graph"][field].as_str().ok_or("missing graph file")?;
        if !key.starts_with("graph/") || key.contains("..") { return Err("invalid graph path".into()); }
        let bytes = fs::read(root.join(key))?;
        if sha(&bytes) != manifest["graph"][digest].as_str().ok_or("missing graph hash")? {
            return Err(format!("digest mismatch: {key}").into());
        }
        if kind == "q42" && !bytes.starts_with(b"Q42\0") { return Err("invalid Q42 magic".into()); }
        writer.add_file(key, kind, bytes, None)?;
    }
    let pack = writer.build()?;
    let reader = BundleReader::parse(&pack)?;
    if reader.entries().len() != writer.len() || reader.entries().iter().any(|e| !reader.verify_entry(&e.key)) {
        return Err("HMC entry verification failed".into());
    }
    fs::write(&out, &pack)?;
    println!("{} entries, {} bytes, sha256:{} => {}", reader.entries().len(), pack.len(), sha(&pack), out.display());
    Ok(())
}
