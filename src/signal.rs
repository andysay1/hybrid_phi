//! φ-signal module for encoding and decoding simple signals
//! Demonstrates how hybrid φ can represent wave-like or temporal data

use crate::core::{hybrid_phi_approximate, hybrid_phi_inverse};

/// Generate a simple sine wave signal of length `len`
pub fn generate_sine_wave(len: usize, freq: f64, phase: f64) -> Vec<f64> {
    let step = 2.0 * std::f64::consts::PI * freq / len as f64;
    (0..len)
        .map(|i| (i as f64 * step + phase).sin())
        .collect()
}

/// Encode signal using hybrid φ approximation
pub fn phi_encode_signal(signal: &[f64], n: usize) -> Vec<f64> {
    signal.iter().map(|&x| hybrid_phi_approximate(x, n)).collect()
}

/// Decode signal using hybrid φ inverse
pub fn phi_decode_signal(encoded: &[f64], n: usize) -> Vec<f64> {
    encoded.iter().map(|&x| hybrid_phi_inverse(x, n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_signal_cycle() {
        let signal = generate_sine_wave(128, 3.0, 0.0);
        let n = 10;
        let eps = f64::EPSILON.sqrt();

        let encoded = phi_encode_signal(&signal, n);
        let decoded = phi_decode_signal(&encoded, n);

        for (orig, recon) in signal.iter().zip(decoded.iter()) {
            let abs_err = (orig - recon).abs();
            let rel_err = abs_err / orig.abs().max(1.0);

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