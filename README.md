# Dither CLI + Library

A fast and flexible **Rust image dithering toolkit**:

## Features

- Dithering algorithms:
  - Floyd-Steinberg
  - Jarvis, Judice, Ninke
  - Stucki
  - Atkinson
  - Burkes
  - Sierra
  - Two-Row Sierra
  - Sierra Lite
  - 4x4 Bayer
  - 8x8 Bayer
- Palette support:
  - Monochrome (2 colors)
  - 8 Color
  - 16 Color
  - Custom palettes (planned)
- Usable as both **CLI** and **library**.

---

## Project structure
```
dither-cli/            # package
├── Cargo.toml         # package = dither-cli
  └── src/
    ├── main.rs        # binary = dither-cli
    ├── lib.rs         # library crate = dither_lib
    ├── dither.rs
    ├── palette.rs
    ├── args.rs
```

---

##  CLI usage

### Build

```bash
cargo build --release --bin dither-cli
```

### Run

```bash
./target/release/dither-cli -i input.png -o output.png -c color8 --dither floyd-steinberg
./target/release/dither-cli -i input.png -o output.png -c color16 --dither floyd-steinberg
./target/release/dither-cli -i input.png -o output.png -c monochrome --dither floyd-steinberg
```
