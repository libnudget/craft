# craft

An experimental package manager for single-file tools.

craft explores installing tools as single, signed binaries without
package-manager lock-in. This is an experiment: the interface may change,
and the philosophy is being tested.

## Status

This is an experimental release. The interface is not yet stable.

## Installation

```sh
cargo install --git https://github.com/libnudget/craft --tag v0.1.0
```

## Usage

Create a new package:

```sh
craft new my-tool
```

This creates `my-tool/` with a `Craft.toml` manifest and a starter
`src/main.rs`.

```sh
craft --help
craft --version
```

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## License

MIT
