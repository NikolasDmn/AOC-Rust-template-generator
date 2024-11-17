# Simple Advent of Code templating for rust

## Features

- Organized file structure
- Easy execution with rust bins

## How to use

- Pass arguments: `{year}/{day}` to generate the files for a specific day
- Works **only** in the root directory of a rust directory
- Will append to your `Cargo.toml` specific binary configurations for each day
- To run each part of a day run: `cargo run --bin {year}_{day}_{part}`

## May implement the following features

- Generate a whole year worth of days using argument `{year}` (without specifying a day)
- Fetching input/sample file from an api dynamically using something like [this](https://github.com/antoniosubasic/aoc_api)
