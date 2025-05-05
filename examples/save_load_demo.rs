//! CLI demo: encode values → save to disk → load → decode
//! Run with: cargo run --example save_load_demo

use hybrid_phi::core::hybrid_phi_inverse;
use hybrid_phi::quantized_memory::phi_quantized_encode;
use hybrid_phi::storage::{save_quantized, load_quantized};
use std::path::Path;

fn main() {
    let values = [-1000.0, -1.0, 0.0, 1.0, 42.0, 123.456, 999.99];
    let n = 10;
    let step = 0.01;
    let path = Path::new("phi_memory.bin");

    // Encode & quantize
    let encoded: Vec<f64> = values
        .iter()
        .map(|&w| phi_quantized_encode(w, n, step))
        .collect();

    // Save to disk
    save_quantized(&encoded, &path).expect("save failed");
    println!("Saved {} entries to {:?}", encoded.len(), path);

    // Load from disk
    let loaded = load_quantized(&path).expect("load failed");
    println!("Loaded {} entries from disk\n", loaded.len());

    println!(
        "{:<10} {:>12} {:>12} {:>12}",
        "original", "quantized", "recovered", "error"
    );

    for ((&w, &q), r) in values.iter().zip(loaded.iter()).zip(loaded.iter().map(|&q| hybrid_phi_inverse(q, n))) {
        let err = (w - r).abs();
        println!(
            "{:<10.3} {:>12.6} {:>12.6} {:>12.3e}",
            w, q, r, err
        );
    }

    // Optionally: remove file after demo
    let _ = std::fs::remove_file(path);
}
