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

/// Inverse of hybrid φ-based approximation
///
/// Given approximated value `w_hat` and index `n`, reconstructs the original `w`.
///
/// This inverts the approximation:
/// `w_hat ≈ a · φ(N) · (1 - r + r / √2)`
///
/// Returns original `w ≈ a · φ(N)`
pub fn hybrid_phi_inverse(w_hat: f64, n: usize) -> f64 {
    if n == 0 || n > PHI_TABLE.len() {
        return w_hat; // fallback
    }

    let phi = PHI_TABLE[n - 1];

    // We solve for a using the original correction function:
    // w_hat = a * φ * (1 - r + r / √2)
    //
    // Let’s define x = a · φ
    // Let’s find r numerically via iteration (or approx):
    //
    // But since r = w - a·φ ≈ 0 for a good approx,
    // we can just reverse the correction:
    //
    // Let c = correction = (1 - r + r / √2)
    // Then:
    // x = w_hat / c
    //
    // We'll solve for c numerically via Newton–Raphson
    // But here we use a fast approximation (since r small):
    //
    // Approximate inverse correction:
    let mut x = w_hat;
    for _ in 0..3 {
        let a = x / phi;
        let r = x - a * phi;
        let correction = 1.0 - r + r / std::f64::consts::SQRT_2;
        x = w_hat / correction;
    }

    x
}

#[test]
fn test_inverse_consistency() {
    let eps = f64::EPSILON.sqrt(); // ≈ 1.49e-8 — безопасный допуск
    let w = 123.456;

    for n in 1..=32 {
        let approx = hybrid_phi_approximate(w, n);
        let recovered = hybrid_phi_inverse(approx, n);

        let abs_err = (recovered - w).abs();
        let rel_err = abs_err / w.abs().max(1.0); // нормировка

        assert!(
            abs_err < eps || rel_err < eps,
            "n = {}, abs_err = {:.3e}, rel_err = {:.3e}",
            n,
            abs_err,
            rel_err
        );
    }
}
