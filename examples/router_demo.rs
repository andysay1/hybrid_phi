//! CLI demo for phi_router
//! Run with: cargo run --example router_demo -- --step=0.005 --threshold=0.9 --verbose


use hybrid_phi::quantized_memory::phi_quantized_encode;
use hybrid_phi::phi_fs::PhiMemoryStore;
use hybrid_phi::phi_router::{phi_route, phi_similarity};
use std::env;

fn parse_args() -> (f64, f64, bool) {
    let mut step = 0.01;
    let mut threshold = 0.8;
    let mut verbose = false;
    for arg in env::args().skip(1) {
        if let Some(v) = arg.strip_prefix("--step=") {
            step = v.parse().unwrap_or(step);
        }
        if let Some(v) = arg.strip_prefix("--threshold=") {
            threshold = v.parse().unwrap_or(threshold);
        }
        if arg == "--verbose" {
            verbose = true;
        }
    }
    (step, threshold, verbose)
}

fn main() {
    let n = 10;
    let (step, threshold, verbose) = parse_args();
    let store = PhiMemoryStore::new(".phi_store");

    // Define and encode reference signals
    let patterns = vec![
        ("calm", vec![0.1, 0.1, 0.1, 0.1]),
        ("ramp", vec![0.1, 0.2, 0.3, 0.4]),
        ("burst", vec![1.0, 2.0, 3.0, 4.0]),
    ];

    for (name, signal) in &patterns {
        let encoded: Vec<f64> = signal
            .iter()
            .map(|&x| phi_quantized_encode(x, n, step))
            .collect();
        store.save(name, &encoded).expect("save failed");
    }

    // Simulate input (similar to "burst")
    let input_signal = vec![0.95, 2.05, 3.1, 3.95];
    let encoded_input: Vec<f64> = input_signal
        .iter()
        .map(|&x| phi_quantized_encode(x, n, step))
        .collect();

    if verbose {
        println!("\nSimilarity to each pattern:");
        let names = store.list().expect("failed to list");
        for name in names {
            if let Ok(entry) = store.load(&name) {
                let score = phi_similarity(&encoded_input, &entry);
                println!("- {}: {:.3}%", name, score * 100.0);
            }
        }
    }

    // Route input
    match phi_route(&encoded_input, &store, threshold) {
        Some((name, score)) => {
            println!("\nInput routed to '{}', similarity = {:.3}%", name, score * 100.0);
        }
        None => println!("\nNo matching route found (threshold = {:.2})", threshold),
    }

    // Optional cleanup
    let _ = std::fs::remove_dir_all(".phi_store");
}