<div>
  <h1 align="center">gabble</h1>
  <p align="center">
    A "Host Yourself" Chat powered by Rust and Whizzes Contributors
  </p>
</div>

<div align="center">

  [![Discord](https://img.shields.io/discord/1011702194925490186?color=blue&label=discord&logo=discord)](https://discord.gg/yde6mcgs2C)
  ![Build](https://github.com/whizzes/gabble/workflows/build/badge.svg)
  ![Clippy](https://github.com/whizzes/gabble/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/whizzes/gabble/workflows/fmt/badge.svg)

</div>

## Development

### Requirements

- [Rust](https://rustup.rs)
- [Justfile](https://github.com/casey/just) (**Recommended**)

### Getting Started

```bash
# install just command runner
cargo install just

# clone this repository
git clone https://github.com/whizzes/gabble.git

# step into repository directory
cd ./gabble

# prepare project (depends on unix shell)
just prepare

# execute the server (next time you just run this command)
just serve
```

> Note: As of today migrations runs when bootstrapping the server automatically

## Architecture

<div align="center">
  <img src="./docs/diagram.png">
</div>

## Contributions

All contributions to this project are welcome. Feel free to open a PR or issue

## License

Licensed under the MIT License
