// examples/demo.rs
use hybrid_phi::{hybrid_phi_approximate, hybrid_phi_inverse};
use std::env;

fn parse_n_arg() -> usize {
    for arg in env::args().skip(1) {
        if let Some(n_str) = arg.strip_prefix("--n=") {
            if let Ok(n) = n_str.parse::<usize>() {
                if (1..=32).contains(&n) {
                    return n;
                } else {
                    eprintln!("Warning: N must be between 1 and 32. Using default N=10.");
                }
            } else {
                eprintln!("Warning: couldn't parse N from '{}'. Using default N=10.", n_str);
            }
        }
    }
    10 // default
}

fn main() {
    let n = parse_n_arg();
    let values = [-1000.0, -100.0, 0.0, 42.0, 123.456, 999.99];

    println!("Hybrid φ-approximation with N = {}\n", n);
    println!("{:<10} {:>12} {:>12} {:>12} {:>12}", "w", "approx", "recovered", "approx_err", "recon_err");

    for &w in &values {
        let approx = hybrid_phi_approximate(w, n);
        let recovered = hybrid_phi_inverse(approx, n);
        let approx_err = (approx - w).abs();
        let recon_err = (recovered - w).abs();
        println!(
            "{:<10.3} {:>12.6} {:>12.6} {:>12.3e} {:>12.3e}",
            w, approx, recovered, approx_err, recon_err
        );
    }
}
