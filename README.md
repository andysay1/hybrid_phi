# README.md

# hybrid_phi

**High-accuracy numerical approximation using a φ-based hybrid method.**

This Rust library provides a fast, compact and reversible approximation for real numbers using a precomputed exponential φ-basis and a smooth linear correction.

## ✨ Features

-   Machine-level precision (~1e-14)
-   Precomputed lookup table (φ[1..32])
-   Suitable for embedded/AI/quantized data
-   Simple implementation: 1 multiplication + 1 correction

## 📦 Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
hybrid_phi = "0.1.0"
```

## 🔧 Example

```rust
use hybrid_phi::hybrid_phi_approximate;

fn main() {
    let x = 123.456;
    let approx = hybrid_phi_approximate(x, 10);
    println!("Approximated value: {}", approx);
}
```

## 📚 Algorithm

```math
w ≈ a · φ(N) · (1 - r + r / √2)
```

Where:

-   `φ(N) = ∑ j · exp(1 / (2j))`
-   `a = w / φ`, `r = w - aφ`

## 🔐 License

MIT

---

**© 2025 Idan Kaminer** — author of the method and implementation.

---
