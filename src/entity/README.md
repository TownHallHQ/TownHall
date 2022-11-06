# Running Entity Generation

First make sure all migrations are applied. Refer to [migration/README.md][1].

Then install `sea-orm-cli` if not yet installed:

```bash
cargo install sea-orm-cli
```

Finally run the `sea-orm-cli` providing the `-u` option with the database url.
Make sure you run this command inside the `entity/src` directory.

```bash
sea-orm-cli generate entity -u postgresql://linx:linx@localhost:5432/linx
```

[1]: ../migration/README.md
