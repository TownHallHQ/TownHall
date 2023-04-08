docker-setup:
  docker compose up database

docker-cleanup:
  docker compose down

migrate:
  cargo run database migrate

entities:
  sea-orm-cli generate entity \
    --with-serde both \
    --output-dir ./src/entity/src --lib
