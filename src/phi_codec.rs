//! φ-codec module: compact interface to encode & decode single values
//! Useful for pipelines, testing and expressive visualizations

use crate::core::{hybrid_phi_approximate, hybrid_phi_inverse};

/// φ-codec: encodes and decodes a single value w → (approx, recovered)
#[inline(always)]
pub fn phi_codec(w: f64, n: usize) -> (f64, f64) {
    let approx = hybrid_phi_approximate(w, n);
    let recovered = hybrid_phi_inverse(approx, n);
    (approx, recovered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_codec_roundtrip() {
        let values = [-1000.0, -42.0, 0.0, 1.0, 123.456, 888.88];
        let n = 10;
        let eps = f64::EPSILON.sqrt();

        for &w in &values {
            let (approx, recovered) = phi_codec(w, n);
            let abs_err = (recovered - w).abs();
            let rel_err = abs_err / w.abs().max(1.0);

            assert!(
                abs_err < eps || rel_err < eps,
                "w = {}, approx = {}, recovered = {}, abs_err = {:.3e}, rel_err = {:.3e}",
                w,
                approx,
                recovered,
                abs_err,
                rel_err
            );
        }
    }
}
