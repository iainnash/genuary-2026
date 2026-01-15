# Genurary 2026

This repository contains my submissions for [Genurary 2026](https://genuary.art/), a month-long creative coding challenge held every January. The name is a mashup of "generative art" and "January."

## What is Genurary?

Genurary is an annual event where participants create generative art based on daily prompts throughout January. It encourages experimentation with algorithms, creative coding, and procedural generation techniques. The challenge is open to all skill levels and can be completed using any programming language or creative coding environment.

## Project Structure

Each day's challenge is in its own directory:

- **Day 1**: Stars animation using Rust and simple physics (rust)
- **Day 2**: DVD logo bounce simulation in Rust (rust)
- **Day 3**: Video capture and shader effects in Rust (rust, requires OSX and gstreamer)
- **Day 4**: Advanced shader techniques with video input (rust)
- **Day 5**: Typography experiments with multiple implementations:
  - Ruby implementation with DSL for letter definitions
  - Rust implementation with macro-based glyph system
  - Outputs: Printer (postscript), Plotter (g-code), Screen (svg)

## Running the Projects

Each day's project can be run independently:

### Ruby Projects
```bash
cd day-5/ruby
ruby generate_postscript.rb
```

### Rust Projects
```bash
cd day-N/rust
cargo run
```

## License

This project is open source and available under the MIT License.

## About Genurary

[Genurary](https://genuary.art/) is a community-driven initiative that encourages artists and coders to explore generative art. It's not a competition—there's no judging or prizes—but rather a celebration of creativity and code.