dev:
  docker compose up

undev:
  docker compose down

clippy:
  cargo clippy --fix --workspace

prepare:
  cp .env.example .env
  cargo run database migrate
  cargo run database entities

serve:
  cargo run serve
