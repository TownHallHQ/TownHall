<div align="center">
  <span style="font-size: 64px;">🏖️</span>
  <h1 align="center">playa</h1>
  <p align="center">
    Decentralized Social Platform powered by Rust and Whizzes Contributors
  </p>
</div>

<div align="center">

  [![Discord](https://img.shields.io/discord/1011702194925490186?color=blue&label=discord&logo=discord)](https://discord.gg/yde6mcgs2C)
  ![Build](https://github.com/whizzes/playa/workflows/build/badge.svg)
  ![Clippy](https://github.com/whizzes/playa/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/whizzes/playa/workflows/fmt/badge.svg)

</div>

## Development

### Requirements

- [Rust](https://rustup.rs)
- [Docker](https://www.docker.com)
- [Justfile](https://github.com/casey/just) (**Recommended**)

### Getting Started

```bash
# install just command runner
cargo install just

# clone this repository
git clone https://github.com/whizzes/playa.git


# step into repository directory
cd ./playa

# open a termital window and spin up Docker Containers
just dev

# create a new terminal window and run database migrations
just prepare

# execute the server (next time you just run this command)
cargo run serve
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
