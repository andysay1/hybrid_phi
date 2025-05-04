# hybrid_phi

**High-accuracy numerical approximation using a φ-based hybrid method.**

This Rust library provides a fast, compact and reversible approximation for real numbers using a precomputed exponential φ-basis and a smooth linear correction.

---

## ✨ Features

-   ⚡ **Machine-level precision** (~1e-14)
-   📐 **φ-basis from exponential series** (precomputed φ[1..32])
-   🧠 **Smooth correction** preserves reversibility
-   🔢 **1 multiplication + 1 correction**: ideal for embedded/AI/inference
-   ✅ **Zero allocation**, pure `f64`

---

## 📦 Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
hybrid_phi = "0.1.0"
```

---

## 🔧 Example (Library)

```rust
use hybrid_phi::hybrid_phi_approximate;

fn main() {
    let x = 123.456;
    let approx = hybrid_phi_approximate(x, 10);
    println!("Approximated value: {}", approx);
}
```

---

## 🚀 Example (CLI)

```bash
cargo run --example demo -- --n=32
```

```
Hybrid φ-approximation with N = 32

w                approx    recovered   approx_err    recon_err
-1000.000  -1000.000000 -1000.000000      0.000e0      0.000e0
-100.000    -100.000000  -100.000000    4.121e-13    4.121e-13
0.000          0.000000     0.000000      0.000e0      0.000e0
42.000        42.000000    42.000000      0.000e0      0.000e0
123.456      123.456000   123.456000      0.000e0      0.000e0
999.990      999.990000   999.990000      0.000e0      0.000e0
```

---

## 📚 Algorithm

We approximate:

```math
w ≈ a · φ(N) · (1 - r + r / √2)
```

Where:

-   `φ(N) = ∑_{j=1}^N j · exp(1 / (2j))`
-   `a = w / φ(N)`, `r = w - aφ(N)`

This approximation is reversible:

```rust
use hybrid_phi::{hybrid_phi_approximate, hybrid_phi_inverse};

let w = 123.456;
let approx = hybrid_phi_approximate(w, 10);
let recovered = hybrid_phi_inverse(approx, 10);
let error = (w - recovered).abs();
```

---

## 🔢 φ(N) Lookup Table (excerpt)

| N   | φ(N)      | N   | φ(N)       |
| --- | --------- | --- | ---------- |
| 1   | 2.648721  | 17  | 83.835262  |
| 2   | 6.262255  | 18  | 90.093524  |
| 3   | 10.191723 | 19  | 96.461403  |
| 4   | 14.372347 | 20  | 102.936028 |
| 5   | 18.767815 | 21  | 109.514153 |
| ... | ...       | 32  | 188.282950 |

---

## 🔐 License

🚫 **Commercial use requires a separate license.**  
Please contact **info@paxintrade.com** for licensing options.

---

**© 2025 Idan Kaminer** — author of the method and implementation.
