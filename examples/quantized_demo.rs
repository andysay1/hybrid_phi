//! CLI demo for quantized φ-memory
//! Run with: cargo run --example quantized_demo -- --n=10 --step=0.01

use hybrid_phi::quantized_memory::{phi_quantized_encode, phi_quantized_decode};
use std::env;

fn parse_args() -> (usize, f64) {
    let mut n = 10;
    let mut step = 0.01;
    for arg in env::args().skip(1) {
        if let Some(n_str) = arg.strip_prefix("--n=") {
            if let Ok(v) = n_str.parse::<usize>() {
                if (1..=32).contains(&v) {
                    n = v;
                }
            }
        }
        if let Some(s_str) = arg.strip_prefix("--step=") {
            if let Ok(s) = s_str.parse::<f64>() {
                if s > 0.0 {
                    step = s;
                }
            }
        }
    }
    (n, step)
}

fn main() {
    let (n, step) = parse_args();
    let values = [-1000.0, -100.0, -1.0, 0.0, 1.0, 42.0, 123.456, 999.99];

    println!("Quantized φ-memory demo (N = {}, step = {})\n", n, step);
    println!(
        "{:<10} {:>12} {:>12} {:>12}",
        "w", "quantized", "recovered", "error"
    );

    for &w in &values {
        let q = phi_quantized_encode(w, n, step);
        let recon = phi_quantized_decode(q, n);
        let err = (w - recon).abs();

        println!(
            "{:<10.3} {:>12.6} {:>12.6} {:>12.3e}",
            w, q, recon, err
        );
    }
}
