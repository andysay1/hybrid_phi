//! CLI demo for φ-fs: store/load named φ-memory
//! Run with: cargo run --example fs_demo

use hybrid_phi::core::hybrid_phi_inverse;
use hybrid_phi::quantized_memory::phi_quantized_encode;
use hybrid_phi::phi_fs::PhiMemoryStore;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let n = 10;
    let step = 0.01;
    let values = [1.0, 2.0, 3.14, 42.0];
    let name = "phi_shape";
    let store = PhiMemoryStore::new(".phi_store");

    // Encode and store
    let encoded: Vec<f64> = values.iter().map(|&w| phi_quantized_encode(w, n, step)).collect();
    store.save(name, &encoded).expect("failed to save");

    // Save metadata
    let metadata_path = Path::new(".phi_store").join(format!("{}.meta.txt", name));
    let mut meta_file = File::create(&metadata_path).expect("failed to create metadata");
    writeln!(meta_file, "n={}", n).unwrap();
    writeln!(meta_file, "step={:.5}", step).unwrap();
    writeln!(meta_file, "length={}", encoded.len()).unwrap();
    writeln!(meta_file, "saved_at={:?}", chrono::Utc::now()).unwrap();

    // List contents
    let entries = store.list().expect("failed to list");
    println!("Stored φ-entries:");
    for entry in entries {
        println!("- {}", entry);
    }

    // Load and decode
    let loaded = store.load(name).expect("failed to load");
    println!("\nDecoded values from '{}':", name);
    for (i, &q) in loaded.iter().enumerate() {
        let r = hybrid_phi_inverse(q, n);
        println!("  index {}: quant = {:.6}, recon = {:.6}", i, q, r);
    }

    // Optional cleanup
    let _ = fs::remove_dir_all(".phi_store");
}
