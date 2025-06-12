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
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-8c.jpg -d floyd-steinberg -c color8
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-16c.jpg -d floyd-steinberg -c color16
  cd test && cargo run -- -i in/glace-1280_853.jpg -o out/glace-1280_853-monochrome.jpg -d floyd-steinberg -c monochrome

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
