setup-db:
  docker compose up database

shutdown-db:
  docker compose down

migrate:
  cargo run --bin cli database migrate

refresh_db:
  cargo run --bin cli database refresh

generate:
  sea-orm-cli generate entity \
    --with-serde both \
    --output-dir ./src/entity/src --lib

serve:
  cargo run --bin server

clippy:
  cargo clippy --fix --workspace

prepare:
  cp .env.example .env
  cargo install sea-orm-cli
