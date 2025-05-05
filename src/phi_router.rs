//! φ-router: signal-to-memory matching and routing
//! Maps input φ-signals to named memory slots

use crate::phi_fs::PhiMemoryStore;

/// Compute similarity between two quantized φ-sequences
pub fn phi_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut matched = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        if (x - y).abs() < 1e-6 {
            matched += 1;
        }
    }
    matched as f64 / a.len() as f64
}

/// φ-router: find the best-matching memory entry from store
pub fn phi_route(input: &[f64], store: &PhiMemoryStore, threshold: f64) -> Option<(String, f64)> {
    let mut best_score = 0.0;
    let mut best_name = None;

    if let Ok(names) = store.list() {
        for name in names {
            if let Ok(entry) = store.load(&name) {
                let score = phi_similarity(input, &entry);
                if score > best_score {
                    best_score = score;
                    best_name = Some(name);
                }
            }
        }
    }

    if best_score >= threshold {
        best_name.map(|n| (n, best_score))
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::phi_fs::PhiMemoryStore;
    use std::fs;
    use crate::quantized_memory::phi_quantized_encode;

    #[test]
    fn test_phi_routing_logic() {
        let store = PhiMemoryStore::new(".phi_routing_store");

        let n = 10;
        let step = 0.01;
        let threshold = 0.8;

        let signal1 = vec![1.0, 2.0, 3.0];
        let signal2 = vec![10.0, 20.0, 30.0];

        let encoded1: Vec<f64> = signal1.iter().map(|&w| phi_quantized_encode(w, n, step)).collect();
        let encoded2: Vec<f64> = signal2.iter().map(|&w| phi_quantized_encode(w, n, step)).collect();

        store.save("alpha", &encoded1).unwrap();
        store.save("beta", &encoded2).unwrap();

        let input = encoded1.clone();
        let route = phi_route(&input, &store, threshold).unwrap();

        assert_eq!(route.0, "alpha");
        assert!(route.1 >= threshold);

        fs::remove_dir_all(".phi_routing_store").unwrap();
    }
}
