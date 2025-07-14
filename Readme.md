# fluent-plots

[![Crates.io](https://img.shields.io/crates/v/fluent-plots.svg)](https://crates.io/crates/fluent-plots)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A declarative and easy-to-use Rust data visualization library for creating static and interactive charts from Polars DataFrames. `fluent-plots` provides a simple, chainable API for quickly building common plot types.

## Features

- **Fluent API**: Chain methods together to build your chart declaratively.
- **Polars `DataFrame` Integration**: Designed to work directly with the powerful Polars `DataFrame`.
- **Dual Backends**:
  - **(Default)** Static chart rendering using `plotters` (drawing logic still in development).
  - **(Optional)** Interactive HTML charts powered by `charming` and ECharts.js.
- **Lightweight**: The interactive backend is an optional feature to keep your dependency tree small if you only need static plots.

## Installation

Add `fluent-plots` to your `Cargo.toml` file.

For interactive charts, enable the `interactive` feature:

```toml
[dependencies]
fluent-plots = { version = "0.1.0", features = ["interactive"] }
polars = { version = "0.41", features = ["csv"] } # Needed for the examples
```

If you only need the static plotting capabilities (once implemented), you can omit the feature flag.

## Usage

Here's how to create an interactive bar chart from a Polars `DataFrame`.

```rust
use fluent_plots::barchart;
use polars::prelude::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a DataFrame
    let df = df!(
        "category" => &["A", "B", "C", "D"],
        "value" => &[10, 25, 5, 42],
    )?;

    // 2. Build the chart and generate the HTML
    let html_content = barchart(df)
        .x("category")
        .y("value")
        .to_interactive_html()?;

    // 3. Save the HTML to a file
    fs::write("my_interactive_chart.html", html_content)?;

    Ok(())
}
```

### Creating Different Chart Types

You can easily create other chart types by calling `linechart()` or `scatterplot()`.

```rust
use fluent_plots::linechart;
use polars::prelude::*;
use std::fs;

// ... create your DataFrame `df` ...

let html_content = linechart(df)
    .x("category")
    .y("value")
    .to_interactive_html()?;

fs::write("my_line_chart.html", html_content)?;
```

## Running with the Example Binary

This repository includes a test application in the `plot-tester` directory (if you choose to include it) that can be run from the command line to generate charts from a CSV file.

First, create a `sample_data.csv` file:
```csv
category,value
A,10
B,25
C,5
D,42
E,18
```

Then, run the application:

```bash
# Generate a bar chart (default)
cargo run -- sample_data.csv

# Generate a line chart
cargo run -- sample_data.csv line

# Generate a scatter plot
cargo run -- sample_data.csv scatter
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.
