//! Build a source-linked ecology graph into the same Q42 volume format as game-demo packs.

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    imp::run()
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use qualia_core_db::lexicon::generate_60bit_token;
    use qualia_core_db::q42_volume::{StreamingQ42VolumeWriter, Q42_MAGIC};
    use qualia_core_db::{NQuin, QUINS_PER_BLOCK};
    use serde_json::Value;
    use sha2::{Digest, Sha256};
    use std::collections::{BTreeSet, HashMap};
    use std::error::Error;
    use std::path::Path;

    fn required<'a>(value: &'a Value, key: &str) -> Result<&'a str, Box<dyn Error>> {
        value.get(key).and_then(Value::as_str).filter(|s| !s.is_empty())
            .ok_or_else(|| format!("missing {key}").into())
    }

    pub fn run() -> Result<(), Box<dyn Error>> {
        let mut args = std::env::args().skip(1);
        let input = args.next().ok_or("usage: ecopunk-q42 INPUT.json OUTPUT.q42")?;
        let output = args.next().ok_or("usage: ecopunk-q42 INPUT.json OUTPUT.q42")?;
        if args.next().is_some() { return Err("too many arguments".into()); }
        let graph: Value = serde_json::from_slice(&std::fs::read(&input)?)?;
        if required(&graph, "schema")? != "ecopunk-evidence-graph-v1" {
            return Err("unsupported graph schema".into());
        }
        let taxon = required(&graph, "subject")?;
        let model_set = required(&graph, "modelSet")?;
        let sources = graph["sources"].as_object().ok_or("sources must be an object")?;
        let claims = graph["claims"].as_array().ok_or("claims must be an array")?;
        let gaps = graph["gaps"].as_array().ok_or("gaps must be an array")?;
        if claims.len() > 1024 || gaps.len() > 1024 { return Err("graph exceeds bound".into()); }

        let mut triples = BTreeSet::<(String, String, String)>::new();
        let mut add = |s: &str, p: &str, o: &str| {
            triples.insert((s.to_owned(), p.to_owned(), o.to_owned()));
        };
        add(taxon, "ecopunk:hasModelSet", model_set);
        for claim in claims {
            let predicate = required(claim, "predicate")?;
            let object = required(claim, "object")?;
            let source = required(claim, "source")?;
            if !sources.contains_key(source) {
                return Err(format!("unknown source reference {source}").into());
            }
            add(taxon, predicate, object);
            add(object, "ecopunk:sourceCredential", source);
        }
        for gap in gaps {
            let key = required(gap, "key")?;
            let required_proof = required(gap, "requiredProof")?;
            add(taxon, "ecopunk:gap", key);
            add(key, "ecopunk:requiredProof", required_proof);
        }

        let mut lex = HashMap::<u64, String>::new();
        let mut quins = Vec::<NQuin>::new();
        for (s, p, o) in &triples {
            let fields = [s, p, o].map(|term| generate_60bit_token(term.as_bytes()));
            for (hash, term) in fields.into_iter().zip([s, p, o]) {
                if let Some(previous) = lex.insert(hash, term.clone()) {
                    if previous != *term { return Err("lexicon hash collision".into()); }
                }
            }
            quins.push(NQuin {
                subject: fields[0], predicate: fields[1], object: fields[2],
                context: 0, metadata: 0,
                parity: NQuin::calculate_parity(fields[0], fields[1], fields[2], 0, 0),
            });
        }
        quins.sort_unstable_by_key(|q| q.object);
        let mut writer = StreamingQ42VolumeWriter::new(&lex)?;
        writer.declare_permissive_commons();
        for (sequence, chunk) in quins.chunks(QUINS_PER_BLOCK).enumerate() {
            writer.push_block(sequence as u64, chunk)?;
        }
        let temp = tempfile::NamedTempFile::new()?;
        writer.finish(temp.path())?;
        let bytes = std::fs::read(temp.path())?;
        if !bytes.starts_with(&Q42_MAGIC) { return Err("bad Q42 magic".into()); }
        if let Some(parent) = Path::new(&output).parent() { std::fs::create_dir_all(parent)?; }
        std::fs::write(&output, &bytes)?;
        let digest = Sha256::digest(&bytes);
        let digest_hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        println!("{} claims, {} gaps, {} quins, {} bytes, sha256:{}",
            claims.len(), gaps.len(), quins.len(), bytes.len(), digest_hex);
        Ok(())
    }
}
