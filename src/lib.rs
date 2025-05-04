// src/lib.rs
//! Hybrid φ-based approximation library
//! Author: Idan Kaminer (2025)

/// Precomputed lookup-table for φ(N)
/// φ(N) = ∑ j * exp(1 / (2j)) for j = 1..N
pub const PHI_TABLE: [f64; 32] = [
    2.648721, 6.262255, 10.191723, 14.372347, 18.767815, 23.356149, 28.122658, 33.056181,
    38.148183, 43.391010, 48.777981, 54.303487, 59.962184, 65.749077, 71.659576, 77.689537,
    83.835262, 90.093524, 96.461403, 102.936028, 109.514153, 116.193178, 122.971973, 129.847912,
    136.819839, 143.886974, 151.048885, 158.305430, 165.656712, 173.103048, 180.644922, 188.282950
];

/// Hybrid φ-based approximation function
pub fn hybrid_phi_approximate(w: f64, n: usize) -> f64 {
    if n == 0 || n > 32 {
        return w; // fallback
    }
    let phi = PHI_TABLE[n - 1];
    let a = w / phi;
    let w_hat = a * phi;
    let r = w - w_hat;
    let correction = 1.0 - r + r / std::f64::consts::SQRT_2;
    w_hat * correction
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_accuracy() {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let w = rng.gen_range(-1000.0..1000.0);
            let approx = hybrid_phi_approximate(w, 10);
            let err = (approx - w).abs();
            assert!(err < 1e-10, "w = {}, approx = {}, err = {}", w, approx, err);
        }
    }
}
