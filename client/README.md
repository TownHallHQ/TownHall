
<div>
  <h1 align="center">townhall Frontend</h1>
  <h4 align="center">
    A "Host Yourself" Chat powered by Rust and Whizzes Contributors
  </h4>
</div>

<div align="center">

[![Discord](https://img.shields.io/discord/1011702194925490186?color=blue&label=discord&logo=discord)](https://discord.gg/yde6mcgs2C)
![Build](https://github.com/TownHall-HQ/TownHall/workflows/build/badge.svg)
![Tests](https://github.com/TownHall-HQ/TownHall/workflows/test/badge.svg)
![Lint](https://github.com/TownHall-HQ/TownHall/workflows/lint/badge.svg)

</div>

## Development

> We use [Bun.sh][1] to run this project. We recommend you install it to
> contribute w/o any issues. `npm i -g bun`

```bash
# clone repository
git clone git@github.com:whizzes/townhall.git

# cd into the new directory
cd ./townhall/client

# create a `.env` file by copying contents from `.env.example`
cp .env.example .env

# install dependencies
bun i

# optional: make sure townhall server is running
lsof -i -P -n | grep LISTEN

# run on development mode
bun run dev:open

# optional: if you don't want the browser to open-up automatically run
# "bun run dev" instead
```

## License

Licensed under the MIT License

[1]: https://bun.sh
