# Pragmastat

This is a Rust implementation of 'Pragmastat: Pragmatic Statistical Toolkit', which presents a toolkit of statistical procedures that provide reliable results across diverse real-world distributions, with ready-to-use implementations and detailed explanations.

- PDF manual for this version: https://pragmastat.dev/pragmastat-v3.1.24.pdf
- Online manual for the latest version: https://pragmastat.dev
- Manual DOI: [10.5281/zenodo.17236778](https://doi.org/10.5281/zenodo.17236778)
- Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v3.1.24/rust

## Installation

Install from crates.io via cargo:

```bash
cargo add pragmastat
```

Install from crates.io via `Cargo.toml`:

```toml
[dependencies]
pragmastat = "3.1.24"
```

## Demo

```rust
use pragmastat::*;

fn main() {
    let x = vec![0.0, 2.0, 4.0, 6.0, 8.0];
    println!("{}", center(&x).unwrap()); // 4
    let x_plus_10: Vec<f64> = x.iter().map(|v| v + 10.0).collect();
    println!("{}", center(&x_plus_10).unwrap()); // 14
    let x_times_3: Vec<f64> = x.iter().map(|v| v * 3.0).collect();
    println!("{}", center(&x_times_3).unwrap()); // 12

    println!("{}", spread(&x).unwrap()); // 4
    println!("{}", spread(&x_plus_10).unwrap()); // 4
    let x_times_2: Vec<f64> = x.iter().map(|v| v * 2.0).collect();
    println!("{}", spread(&x_times_2).unwrap()); // 8

    println!("{}", rel_spread(&x).unwrap()); // 1
    let x_times_5: Vec<f64> = x.iter().map(|v| v * 5.0).collect();
    println!("{}", rel_spread(&x_times_5).unwrap()); // 1

    let y = vec![10.0, 12.0, 14.0, 16.0, 18.0];
    println!("{}", shift(&x, &y).unwrap()); // -10
    println!("{}", shift(&x, &x).unwrap()); // 0
    let x_plus_7: Vec<f64> = x.iter().map(|v| v + 7.0).collect();
    let y_plus_3: Vec<f64> = y.iter().map(|v| v + 3.0).collect();
    println!("{}", shift(&x_plus_7, &y_plus_3).unwrap()); // -6
    let y_times_2: Vec<f64> = y.iter().map(|v| v * 2.0).collect();
    println!("{}", shift(&x_times_2, &y_times_2).unwrap()); // -20
    println!("{}", shift(&y, &x).unwrap()); // 10

    let x = vec![1.0, 2.0, 4.0, 8.0, 16.0];
    let y = vec![2.0, 4.0, 8.0, 16.0, 32.0];
    println!("{}", ratio(&x, &y).unwrap()); // 0.5
    println!("{}", ratio(&x, &x).unwrap()); // 1
    let x_times_2: Vec<f64> = x.iter().map(|v| v * 2.0).collect();
    let y_times_5: Vec<f64> = y.iter().map(|v| v * 5.0).collect();
    println!("{}", ratio(&x_times_2, &y_times_5).unwrap()); // 0.2

    let x = vec![0.0, 3.0, 6.0, 9.0, 12.0];
    let y = vec![0.0, 2.0, 4.0, 6.0, 8.0];
    println!("{}", spread(&x).unwrap()); // 6
    println!("{}", spread(&y).unwrap()); // 4

    println!("{}", avg_spread(&x, &y).unwrap()); // 5
    println!("{}", avg_spread(&x, &x).unwrap()); // 6
    let x_times_2: Vec<f64> = x.iter().map(|v| v * 2.0).collect();
    let x_times_3: Vec<f64> = x.iter().map(|v| v * 3.0).collect();
    println!("{}", avg_spread(&x_times_2, &x_times_3).unwrap()); // 15
    println!("{}", avg_spread(&y, &x).unwrap()); // 5
    let y_times_2: Vec<f64> = y.iter().map(|v| v * 2.0).collect();
    println!("{}", avg_spread(&x_times_2, &y_times_2).unwrap()); // 10

    println!("{}", shift(&x, &y).unwrap()); // 2
    println!("{}", avg_spread(&x, &y).unwrap()); // 5

    println!("{}", disparity(&x, &y).unwrap()); // 0.4
    let x_plus_5: Vec<f64> = x.iter().map(|v| v + 5.0).collect();
    let y_plus_5: Vec<f64> = y.iter().map(|v| v + 5.0).collect();
    println!("{}", disparity(&x_plus_5, &y_plus_5).unwrap()); // 0.4
    println!("{}", disparity(&x_times_2, &y_times_2).unwrap()); // 0.4
    println!("{}", disparity(&y, &x).unwrap()); // -0.4
}
```

## The MIT License

Copyright (c) 2025 Andrey Akinshin

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
