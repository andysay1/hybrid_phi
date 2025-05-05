//! Quantized φ-memory: lossy φ-based encoding with configurable step
//! Useful for compressing or storing signal "shadows"

use crate::core::{hybrid_phi_approximate, hybrid_phi_inverse};

/// Encode with quantization: round(approx / step) * step
pub fn phi_quantized_encode(w: f64, n: usize, step: f64) -> f64 {
    let approx = hybrid_phi_approximate(w, n);
    (approx / step).round() * step
}

/// Decode quantized φ-code
pub fn phi_quantized_decode(quantized: f64, n: usize) -> f64 {
    hybrid_phi_inverse(quantized, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantized_phi_memory() {
        let values = [-1000.0, -42.0, -1.0, 0.0, 1.0, 42.0, 123.456, 999.99];
        let n = 10;
        let step = 0.01; // simulate compression

        for &w in &values {
            let q = phi_quantized_encode(w, n, step);
            let recovered = phi_quantized_decode(q, n);
            let err = (w - recovered).abs();
            assert!(err < step * 1.5, "w = {}, recovered = {}, err = {:.3e}", w, recovered, err);
        }
    }
}
