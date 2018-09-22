# Overview

`rs-pugbot` is a Discord bot built to administer custom matches in
gaming, and provide a ranked ladder for the Discord's members to compete on.

# Features

These are covered in detail in `MILESTONES.md`

# Installation

Make sure you're running Rust nightly builds. [These
instructions](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)
will help.

You will need a database up and running. I recommend PostgreSQL.

I have found that, on Linux Mint (and Ubuntu etc. probably) you'll need to
install the following:

libssl-dev
libpq5
libpq-dev
gcc-multilib

`sudo apt install -y postgresql libssl-dev libpq5 libpq-dev gcc-multilib`

Additionally, you'll need to install Diesel:

`cargo install diesel_cli --no-default-features --features postgres`

Finally, once you're on Rust nightly, with postgres running, and all
system-level dependencies installed, you can run `cargo install` in the root directory.

# Contributing

Contributions from all skill levels welcome! I'm learning Rust as I go here so
I welcome contributions from fellow newbs and salty experts alike. Please see
the GitHub issues for open tasks.
