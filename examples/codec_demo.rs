//! CLI demo for hybrid_phi::phi_codec
//! Run with: cargo run --example codec_demo -- --n=10

use hybrid_phi::phi_codec::phi_codec;
use std::env;

fn parse_n_arg() -> usize {
    for arg in env::args().skip(1) {
        if let Some(n_str) = arg.strip_prefix("--n=") {
            if let Ok(n) = n_str.parse::<usize>() {
                if (1..=32).contains(&n) {
                    return n;
                }
            }
        }
    }
    10
}

fn main() {
    let n = parse_n_arg();
    let values = [-1000.0, -100.0, -1.0, 0.0, 1.0, 42.0, 123.456, 999.99];

    println!("φ-codec demo with N = {}\n", n);
    println!(
        "{:<10} {:>12} {:>12} {:>12} {:>12}",
        "w", "approx", "recovered", "approx_err", "recon_err"
    );

    for &w in &values {
        let (approx, recovered) = phi_codec(w, n);
        let approx_err = (approx - w).abs();
        let recon_err = (recovered - w).abs();

        println!(
            "{:<10.3} {:>12.6} {:>12.6} {:>12.3e} {:>12.3e}",
            w, approx, recovered, approx_err, recon_err
        );
    }
}
