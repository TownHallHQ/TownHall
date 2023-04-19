setup-db:
  docker compose up database

shutdown-db:
  docker compose down

migrate:
  cargo run --bin cli database migrate

generate:
  sea-orm-cli generate entity \
    --with-serde both \
    --output-dir ./src/entity/src --lib

serve:
  cargo run --bin server
