<span id="rs"></span> <!-- [pdf] DELETE -->

## Rust

Install from crates.io via cargo:

```bash
cargo add pragmastat@3.1.30
```

Install from crates.io via `Cargo.toml`:

```toml
[dependencies]
pragmastat = "3.1.30"
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v3.1.30/rs

Pragmastat on crates.io: https://crates.io/crates/pragmastat

Demo:

```rust
use pragmastat::*;

fn print(result: Result<f64, &str>) {
    println!("{}", result.unwrap());
}

fn add(x: &[f64], val: f64) -> Vec<f64> {
    x.iter().map(|v| v + val).collect()
}

fn multiply(x: &[f64], val: f64) -> Vec<f64> {
    x.iter().map(|v| v * val).collect()
}

fn main() {
    let x = vec![0.0, 2.0, 4.0, 6.0, 8.0];
    print(center(&x)); // 4
    print(center(&add(&x, 10.0))); // 14
    print(center(&multiply(&x, 3.0))); // 12

    print(spread(&x)); // 4
    print(spread(&add(&x, 10.0))); // 4
    print(spread(&multiply(&x, 2.0))); // 8

    print(rel_spread(&x)); // 1
    print(rel_spread(&multiply(&x, 5.0))); // 1

    let y = vec![10.0, 12.0, 14.0, 16.0, 18.0];
    print(shift(&x, &y)); // -10
    print(shift(&x, &x)); // 0
    print(shift(&add(&x, 7.0), &add(&y, 3.0))); // -6
    print(shift(&multiply(&x, 2.0), &multiply(&y, 2.0))); // -20
    print(shift(&y, &x)); // 10

    let x = vec![1.0, 2.0, 4.0, 8.0, 16.0];
    let y = vec![2.0, 4.0, 8.0, 16.0, 32.0];
    print(ratio(&x, &y)); // 0.5
    print(ratio(&x, &x)); // 1
    print(ratio(&multiply(&x, 2.0), &multiply(&y, 5.0))); // 0.2

    let x = vec![0.0, 3.0, 6.0, 9.0, 12.0];
    let y = vec![0.0, 2.0, 4.0, 6.0, 8.0];
    print(spread(&x)); // 6
    print(spread(&y)); // 4

    print(avg_spread(&x, &y)); // 5
    print(avg_spread(&x, &x)); // 6
    print(avg_spread(&multiply(&x, 2.0), &multiply(&x, 3.0))); // 15
    print(avg_spread(&y, &x)); // 5
    print(avg_spread(&multiply(&x, 2.0), &multiply(&y, 2.0))); // 10

    print(shift(&x, &y)); // 2
    print(avg_spread(&x, &y)); // 5

    print(disparity(&x, &y)); // 0.4
    print(disparity(&add(&x, 5.0), &add(&y, 5.0))); // 0.4
    print(disparity(&multiply(&x, 2.0), &multiply(&y, 2.0))); // 0.4
    print(disparity(&y, &x)); // -0.4
}
```