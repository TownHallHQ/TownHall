set positional-arguments

default:
  just --list

dev:
  docker compose up

undev:
  docker compose down

clippy:
  cargo clippy --fix --workspace --allow-dirty
  cargo fmt

dotenv:
  cp .env.example .env

prepare:
  cargo run database migrate
  cargo run database entities

serve:
  cargo run serve

# Runs E2E Tests and optionally runs a specific test requires `e2e_test_dev` to be executed first.
e2e_tests *args='':
  cargo test --package test -- --test-threads=1 $1

# Runs formatting tool against Leptos source
web-fmt:
	leptosfmt ./crates/web/src/*.rs

# Runs Web UI for Development
web-dev:
  cd ./crates/web && trunk serve --config ./Trunk.toml

# Builds Web UI for Production
web-build:
  cd ./crates/web && trunk build --release --locked --config ./Trunk.toml
