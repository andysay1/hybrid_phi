//! φ-memory module for reversible encoding/decoding of value sequences
//! Uses hybrid_phi core approximation functions

use crate::core::{hybrid_phi_approximate, hybrid_phi_inverse};

/// Encode a sequence of values using φ-approximation
/// Returns encoded values (ŵ)
pub fn phi_encode_sequence(data: &[f64], n: usize) -> Vec<f64> {
    data.iter().map(|&w| hybrid_phi_approximate(w, n)).collect()
}

/// Decode a sequence of encoded values using φ-inverse
/// Returns recovered values (w̃)
pub fn phi_decode_sequence(encoded: &[f64], n: usize) -> Vec<f64> {
    encoded.iter().map(|&w_hat| hybrid_phi_inverse(w_hat, n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_memory_cycle() {
        let input = [-1000.0, -1.0, 0.0, 1.0, 42.0, 123.456, 999.99];
        let n = 10;
        let eps = f64::EPSILON.sqrt(); // ≈ 1.49e-8
    
        let encoded = phi_encode_sequence(&input, n);
        let decoded = phi_decode_sequence(&encoded, n);
    
        for (orig, recon) in input.iter().zip(decoded.iter()) {
            let abs_err = (orig - recon).abs();
            let rel_err = abs_err / orig.abs().max(1.0); // нормализация
    
            assert!(
                abs_err < eps || rel_err < eps,
                "orig = {}, recon = {}, abs_err = {:.3e}, rel_err = {:.3e}",
                orig,
                recon,
                abs_err,
                rel_err
            );
        }
    }
}
