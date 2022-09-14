# linx
Headless URL Shortener written in Rust

## Getting Started

### Requirements

- [Rust](https://rustup.rs)
- [Docker](https://docs.docker.com/engine/install/)

### Development

```bash
# clone this repository
git clone https://github.com/whizzes/linx.git

# step into repository directory
cd ./linx

# copy the `.env.example` file into a `.env` file
cp .env.example .env

# spawn a terminal with docker for postgresql
docker compose up

# spawn another terminal with `cargo run` when the database is ready for
# connections
cargo run
```

> Note: As of today migrations runs when bootstrapping the server automatically

## Contributions

All contributions to this project are welcome. Feel free to open a PR or issue

## License

Licensed under the MIT License
