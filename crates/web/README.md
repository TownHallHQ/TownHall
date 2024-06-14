<div align="center">
  <h1 align="center">TownHall Web Front-End</h1>
</div>

## Development

### Requirements

- [Rust](https://rustup.rs)

### First Time Setup

1. Install `Trunk` Rust bundler from [here](https://trunkrs.dev/)
2. Install `Just` (`Justfile`) Rust command runner [here](https://github.com/casey/just)
3. Make sure the `wasm32-unkown-unknown` target is installed if not you can use

```bash
rustup target add wasm32-unknown-unknown
```

4. Run the project using `just web-dev` command

```bash
just web-dev
```

### Running Tests

Tests runs in a Headless Browser using `wasm-bindgen-test` and `wasm-bindgen-test-runner`

You must install `wasm-pack`:

```bash
cargo install wasm-pack
```

Then run the tests using `just web-test` command

```bash
just web-test
```

> The first time the Headless Browser is installed to the temporal directory
> for your system. Then it is cached for future runs, so expect a bit of delay
> the first time you run the tests.

## License

Licensed under the MIT License
