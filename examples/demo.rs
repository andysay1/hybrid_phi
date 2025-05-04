// examples/demo.rs
use hybrid_phi::hybrid_phi_approximate;

fn main() {
    let values = [-1000.0, -100.0, 0.0, 42.0, 123.456, 999.99];
    let n = 10; // Use φ(N=10)

    println!("Hybrid φ-approximation with N = {}\n", n);
    for &w in &values {
        let approx = hybrid_phi_approximate(w, n);
        let error = (approx - w).abs();
        println!("Original: {:>8.3}, Approximated: {:>8.3}, Error: {:.3e}", w, approx, error);
    }
} 
