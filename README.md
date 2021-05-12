[![mattdeboard](https://circleci.com/gh/mattdeboard/rs-pugbot.svg?style=svg)](https://app.circleci.com/pipelines/github/mattdeboard/rs-pugbot)

# 🦀 PugBot 🦀

## Overview

`rs-pugbot` is a Discord bot built to administer custom matches in
gaming, and provide a ranked ladder for the Discord's members to compete on.

## Features

These are covered in detail in `MILESTONES.md`.

## Installation Prerequisites

1. Make sure you're running Rust's `stable` toolchain. This can be installed via [`rustup`](https://rustup.rs/):

```
$ rustup toolchain install stable
```

2. You will need a database server up and running. I recommend PostgreSQL, and all documentation is written assuming PostgreSQL.
   * (Postgres) You will need to add a role for the `pugbot` user. What I did was this:

    ```sql
    CREATE ROLE pugbot SUPERUSER LOGIN PASSWORD 'password';
    ```

3. Install `diesel_cli`. See [here](https://diesel.rs/guides/getting-started.html) for more information, but the below should get you going:

```shell
cargo install diesel_cli --no-default-features --features postgres
```

4. You will also need a `.env` file. At the time of this writing (May '21), three key-value pairs are read from this file:
   * `TEAM_COUNT`: How many teams there are in a given game
   * `TEAM_SIZE`: How many players are on each team
   * `DATABASE_URL`: Connection string to the database, ex. `postgres://pugbot:password@localhost:5432/pugbot`
   * `DISCORD_TOKEN`: An OAuth2 token issued by Discord that the bot will use to connect to your server.
5. You'll need an OAuth2 token for your bot from Discord. While there are [official documentation](https://discord.com/developers/docs/topics/oauth2), I found [this blog post](https://www.writebots.com/discord-bot-token) more to-the-point.

### Linux
I have found that, on Linux Mint (and Ubuntu etc. probably) you'll need to
install the following:

* libssl-dev
* libpq5
* libpq-dev
* gcc-multilib

`sudo apt install -y postgresql libssl-dev libpq5 libpq-dev gcc-multilib`

## Tests
First, you'll need Rust's stable toolchain, installed via [`rustup`](https://rustup.rs/):

```
$ rustup toolchain install stable
```

Then, you will need a `.env` file with `DATABASE_URL` and `TEAM_COUNT` attributes. Rename the file `rename-this-to-.env` to `.env`. (`.env` is in `.gitignore` so it won't be added to version control.)

Finally, run `cargo test`.

## Contributing

Contributions from all skill levels welcome! I'm learning Rust as I go here so
I welcome contributions from fellow newbs and salty experts alike. Please see
the GitHub issues for open tasks.

## License
Licensed under MIT license ([LICENSE](https://github.com/mattdeboard/rs-pugbot/blob/main/LICENSE) or https://opensource.org/licenses/MIT)
