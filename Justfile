dev:
  docker compose up

undev:
  docker compose down

clippy:
  cargo clippy --fix --workspace

dotenv:
  cp .env.example .env

prepare:
  cargo run database migrate
  cargo run database entities

serve:
  cargo run serve

e2e_test:
  cargo test --package test -- --test-threads=1
