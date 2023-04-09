<div>
  <h1 align="center">quicklink</h1>
  <p align="center">Headless URL Shortener</p>
</div>

<div align="center">

  [![Discord](https://img.shields.io/discord/1015625333887733882?color=blue&label=discord&logo=discord)](https://discord.gg/yde6mcgs2C)
  ![Build](https://github.com/whizzes/quicklink/workflows/build/badge.svg)
  ![Clippy](https://github.com/whizzes/quicklink/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/whizzes/quicklink/workflows/fmt/badge.svg)

</div>

## Development

### Requirements

- [Rust](https://rustup.rs)

### Getting Started

```bash
# clone this repository
git clone https://github.com/whizzes/quicklink.git

# step into repository directory
cd ./quicklink

# copy the `.env.example` file into a `.env` file
cp .env.example .env

# execute the server
cargo run
```

> Note: As of today migrations runs when bootstrapping the server automatically

## Architecture

<div align="center">
  <img src="./docs/components-chart.png" />
</div>

## Contributions

All contributions to this project are welcome. Feel free to open a PR or issue

## License

Licensed under the MIT License
