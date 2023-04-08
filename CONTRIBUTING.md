## Database Management

This application uses seaql/sea-orm for Database connections and conpsumption.
You must install `sea-orm-cli` if you want to create new migrations.

Use the following command to install SeaORM CLI

```bash
cargo install sea-orm-cli
```

To run any migration or database related command, make sure `DATABASE_URL`
environment variable is present, either inline via Terminal or via `.env` file.

```bash
cargo run --manifest-path ./migration/Cargo.toml -- up
```

To learn more about SeaORM you can read on their tests and examples, a
recommended Migrations example is the [_Bakery Chain_ schema][1].

[1]: https://github.com/SeaQL/sea-orm/tree/master/tests/common/bakery_chain
