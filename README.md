# linx
Headless URL Shortener written in Rust

## Development

### Requirements

- [Rust](https://rustup.rs)

### Optional

- [Docker](https://docs.docker.com/engine/install/)
> For use with PostgreSQL

### Setup for Local Development

**Makefile**

A `Makefile` is included to help setting up the environment and running the
application with ease. Execute `make env` to create a copy of the `.env.example`
file in the project directory. Then use `make install` to build Docker images,
install SeaORM CLI and run migrations.

Finally use `make run` to start docker containers using Docker Compose detached
mode and then execute the HTTP server by running `cargo run` against the `server`
crate.

Remember to run `make stop` when you are done to shut down running containers
and apply cleanup routines.

**Manual (Using PostgreSQL)**

Before running any of the further commands, being database management or
executing the server for local development, its important you follow the
following setup steps before.

```bash
# clone this repository
git clone https://github.com/whizzes/linx.git

# step into repository directory
cd ./linx

# copy the `.env.example` file into a `.env` file
cp .env.example .env

# run docker compose containers
docker compose up -d

# run migrations
cargo run --bin migration -- up

# run server
cargo run --bin server
```

> Note: As of today migrations runs when bootstrapping the server automatically

**Manual (Using SQLite)**

You can run this application without Docker relying on SQLite instead of PSQL.
To do this set the `USE_SQLITE` environment variable to `1` in your `.env` file.

### Crates

This project uses Rust's [Cargo Workspaces][2] structure.

- Entity: SeaORM Generated Entities
- Migration: Database Migrations
- Server: HTTP Server Application

### Database Management

The following commands are used to manage the database used by linx. It's
important to follow on the [Setup for Local Development](#setup-for-local-development) before.

Command | Usage
--- | ---
`cargo run --bin migration -- up` | Run all pending migrations
`cargo run --bin migration -- down` | Rollback last applied migrations
`cargo run --bin migration -- status` | Check migrations status
`cargo run --bin migration -- fresh` | Drops all tables and apply migrations
`cargo run --bin migration -- reset` | Rollback all applied migrations
`cargo run --bin migration -- refresh` | Rollback all migrations and reapply them

### Entities

Entities live in the `entity` crate. These are generated structs from the
database state. To generate these structs you must have `sea-orm-cli` installed.

> In future versions this no longer be required due to the availabilty of [this feature][1].

Install `sea-orm-cli` if not yet installed:

```bash
cargo install sea-orm-cli
```

Finally run the `sea-orm-cli` providing the `-u` option with the database url.
Make sure you run this command inside the `entity/src` directory.

```bash
sea-orm-cli generate entity -u postgresql://linx:linx@localhost:5432/linx
```

## Contributions

All contributions to this project are welcome. Feel free to open a PR or issue

## License

Licensed under the MIT License

[1]: https://github.com/SeaQL/sea-orm/pull/1054
[2]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
