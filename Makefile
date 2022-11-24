env:
	echo "Creating a .env file"
	cp .env.example .env

install: .env
	echo "Building Docker Compose images" 
	docker compose build
	echo "Installing the SeaORM CLI"
	cargo install sea-orm-cli
	echo "Starting Database using Docker Compose"
	docker compose up -d
	echo "Running database migrations"
	cargo run --bin migration -- up
	echo "Stopping Containers"
	docker compose down

migrations:
	echo "Running database migrations"
	cargo run --bin migration -- up
	echo "Generating Entities"
	sea-orm-cli generate entity \
		--with-serde both \
		--output-dir ./entity/src

run:
	echo "Starting Containers"
	docker compose up -d
	echo "Running Server"
	RUST_LOG='server=debug' cargo run --bin server

stop:
	echo "Stopping Containers"
	docker compose down
