##################################################
# Variables
#
rust_env := "rustup show"
rust_edition := "2024"
open := if os() == "linux" {
  "xdg-open"
} else if os() == "macos" {
  "open"
} else {
  "start \"\" /max"
}
args := ""
project_directory := justfile_directory()
release := `git describe --tags --always`
url := "https://github.com/tschinz/dither-cli"

##################################################
# COMMANDS
#

# List all commands
@default:
  just --list

# Information about the environment
@info:
  echo "Environment Informations\n------------------------\n"
  echo "OS   : {{os()}}({{arch()}})"
  echo "Open : {{open}}"
  echo "Rust :"
  echo "`{{rust_env}}`"

# Install dependencies
install:
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  cargo install --locked trunk

# install the release version (default is the latest)
install-release release=release:
  cargo install --git {{url}} --tag {{release}}

# install the nightly release
install-nightly:
  cargo install --git {{url}}

# Run the program in debug mode
run args=args:
  cargo run -- {{args}}

# Test the program in debug mode in folder test
test:
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-floyd_steinberg-8c.jpg -d floyd-steinberg -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-floyd_steinberg-16c.jpg -d floyd-steinberg -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-floyd_steinberg-2c.jpg -d floyd-steinberg -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-simple2d-8c.jpg -d simple2-d -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-simple2d-16c.jpg -d simple2-d -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-simple2d-2c.jpg -d simple2-d -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-jarvis-8c.jpg -d jarvis -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-jarvis-16c.jpg -d jarvis -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-jarvis-2c.jpg -d jarvis -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-atkinson-8c.jpg -d atkinson -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-atkinson-16c.jpg -d atkinson -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-atkinson-2c.jpg -d atkinson -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-stucki-8c.jpg -d stucki -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-stucki-16c.jpg -d stucki -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-stucki-2c.jpg -d stucki -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-burkes-8c.jpg -d burkes -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-burkes-16c.jpg -d burkes -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-burkes-2c.jpg -d burkes -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-sierra-8c.jpg -d sierra -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-sierra-16c.jpg -d sierra -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-sierra-2c.jpg -d sierra -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-tworowsierra-8c.jpg -d two-row-sierra -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-tworowsierra-16c.jpg -d two-row-sierra -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-tworowsierra-2c.jpg -d two-row-sierra -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-sierralite-8c.jpg -d sierra-lite -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-sierralite-16c.jpg -d sierra-lite -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-sierralite-2c.jpg -d sierra-lite -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer2x2-8c.jpg -d bayer2x2 -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer2x2-16c.jpg -d bayer2x2 -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer2x2-2c.jpg -d bayer2x2 -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer4x4-8c.jpg -d bayer4x4 -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer4x4-16c.jpg -d bayer4x4 -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer4x4-2c.jpg -d bayer8x8 -c monochrome

  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer8x8-8c.jpg -d bayer8x8 -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer8x8-16c.jpg -d bayer8x8 -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-bayer8x8-2c.jpg -d bayer8x8 -c monochrome

# output the help information
help:
  cargo run -- -h


# Build and copy the release version of the program
build:
  cargo build --release
  mkdir -p bin && cp -r {{project_directory}}/target/release bin/

# Run rustfmt with custom configuration
rustfmt:
  find {{invocation_directory()}} -name \*.rs -exec rustfmt {} \;
