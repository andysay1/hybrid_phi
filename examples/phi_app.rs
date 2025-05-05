//! phi_app: unified CLI tool for encoding, storing, routing φ-signals
//! Run with: cargo run --example phi_app encode <name>
//!         : cargo run --example phi_app route --input=... --threshold=0.9
//!         : cargo run --example phi_app list
//!         : cargo run --example phi_app delete <name>
//!         : cargo run --example phi_app describe <name>
//!         : cargo run --example phi_app export <name> --to=file.json
//!         : cargo run --example phi_app import <name> --from=file.json

use hybrid_phi::quantized_memory::phi_quantized_encode;
use hybrid_phi::phi_fs::PhiMemoryStore;
use hybrid_phi::phi_meta::PhiMetadata;
use hybrid_phi::phi_bundle::PhiBundle;
use hybrid_phi::phi_router::{phi_similarity, phi_route};
use std::env;
use std::fs;

fn parse_input_vec(arg: &str) -> Vec<f64> {
    arg.split(',').filter_map(|s| s.parse().ok()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage:\n  encode <name>\n  route --input=... [--threshold=0.9] [--verbose]\n  list\n  delete <name>\n  describe <name>\n  export <name> --to=file.json\n  import <name> --from=file.json");
        return;
    }

    let mode = &args[1];
    let store = PhiMemoryStore::new(".phi_store");
    let n = 10;
    let step = 0.01;

    if mode == "encode" && args.len() >= 3 {
        let name = &args[2];
        println!("Encoding input signal for '{}'. Enter comma-separated values:", name);
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let signal = parse_input_vec(&buf);
        let encoded: Vec<f64> = signal.iter().map(|&x| phi_quantized_encode(x, n, step)).collect();
        store.save(name, &encoded).expect("failed to save");
        let meta = PhiMetadata {
            n,
            step,
            length: encoded.len(),
            saved_at: chrono::Utc::now(),
        };
        meta.save(name, ".phi_store").expect("failed to save metadata");
        println!("Saved {} values to '{}'.", encoded.len(), name);
        return;
    }

    if mode == "route" {
        let mut input: Option<Vec<f64>> = None;
        let mut threshold = 0.8;
        let mut verbose = false;

        for arg in &args[2..] {
            if let Some(v) = arg.strip_prefix("--input=") {
                input = Some(parse_input_vec(v));
            }
            if let Some(v) = arg.strip_prefix("--threshold=") {
                threshold = v.parse().unwrap_or(threshold);
            }
            if arg == "--verbose" {
                verbose = true;
            }
        }

        let input = input.expect("Missing --input argument");
        let encoded_input: Vec<f64> = input.iter().map(|&x| phi_quantized_encode(x, n, step)).collect();

        if verbose {
            println!("Similarity to each stored φ-memory:");
            for name in store.list().unwrap_or_default() {
                if let Ok(entry) = store.load(&name) {
                    let score = phi_similarity(&encoded_input, &entry);
                    println!("- {}: {:.3}%", name, score * 100.0);
                }
            }
        }

        match phi_route(&encoded_input, &store, threshold) {
            Some((name, score)) => println!("\nInput routed to '{}', score = {:.3}%", name, score * 100.0),
            None => println!("\nNo route found (threshold = {:.2})", threshold),
        }
        return;
    }

    if mode == "list" {
        let entries = store.list().unwrap_or_default();
        println!("Stored φ-memories:");
        for name in entries {
            println!("- {}", name);
        }
        return;
    }

    if mode == "delete" && args.len() >= 3 {
        let name = &args[2];
        let _ = fs::remove_file(format!(".phi_store/{}.bin", name));
        let _ = fs::remove_file(format!(".phi_store/{}.meta.txt", name));
        println!("Deleted memory '{}'.", name);
        return;
    }

    if mode == "describe" && args.len() >= 3 {
        let name = &args[2];
        match PhiMetadata::load(name, ".phi_store") {
            Ok(meta) => {
                println!("φ-memory '{}':", name);
                println!("  length   = {}", meta.length);
                println!("  n        = {}", meta.n);
                println!("  step     = {:.5}", meta.step);
                println!("  saved_at = {}", meta.saved_at);
            }
            Err(err) => {
                println!("Failed to load metadata: {}", err);
            }
        }
        return;
    }

    if mode == "export" && args.len() >= 4 {
        let name = &args[2];
        let mut out_path = None;
        for arg in &args[3..] {
            if let Some(p) = arg.strip_prefix("--to=") {
                out_path = Some(p);
            }
        }
        let out_path = out_path.expect("Missing --to=... argument");
        let bundle = PhiBundle::from_store(name, &store).expect("failed to bundle");
        bundle.save_json(out_path).expect("failed to save json");
        println!("Exported '{}' to '{}'.", name, out_path);
        return;
    }

    if mode == "import" && args.len() >= 4 {
        let name = &args[2];
        let mut in_path = None;
        for arg in &args[3..] {
            if let Some(p) = arg.strip_prefix("--from=") {
                in_path = Some(p);
            }
        }
        let in_path = in_path.expect("Missing --from=... argument");
        let bundle = PhiBundle::load_json(in_path).expect("failed to load json");
        bundle.save_to_store(&store).expect("failed to restore");
        println!("Imported '{}' from '{}'.", name, in_path);
        return;
    }

    eprintln!("Unknown mode '{}'. Use 'encode', 'route', 'list', 'delete', 'describe', 'export', or 'import'", mode);
}