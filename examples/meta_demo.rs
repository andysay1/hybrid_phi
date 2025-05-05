//! CLI demo for phi_meta: save/load φ-metadata
//! Run with: cargo run --example meta_demo

use hybrid_phi::phi_meta::PhiMetadata;
use chrono::Utc;
use std::path::Path;

fn main() {
    let name = "example_phi";
    let store_path = Path::new(".phi_store");

    // Ensure directory exists
    std::fs::create_dir_all(store_path).expect("failed to create store directory");


    // Create metadata
    let meta = PhiMetadata {
        n: 10,
        step: 0.01,
        length: 4,
        saved_at: Utc::now(),
    };

    // Save metadata
    meta.save(name, store_path).expect("failed to save metadata");
    println!("Saved metadata for '{}'.", name);

    // Load metadata
    let loaded = PhiMetadata::load(name, store_path).expect("failed to load metadata");
    println!("\nLoaded metadata:");
    println!("  n        = {}", loaded.n);
    println!("  step     = {:.5}", loaded.step);
    println!("  length   = {}", loaded.length);
    println!("  saved_at = {}", loaded.saved_at);

    // Optional cleanup
    let _ = std::fs::remove_file(store_path.join("example_phi.meta.txt"));
}