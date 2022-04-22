# 👾 VGDB

Video game database. 🦀 Rust-powered deployable Docker container with the following:

- [Rocket](https://rocket.rs/) Rust server (it’s low resource and really stinking fast)
- [sqlite](https://docs.rs/rusqlite/latest/rusqlite/index.html) database (why pay for expensive DB hosting when sqlite is free?)
- [Tera](https://tera.netlify.app/) HTML templating (the Rust equivalent of Jinja2)

Disregarding the “side project”-y nature of this repo, if you find use of any of this config, use this as an example.

## Dev

### Updating database

The sqlite database lives in `db/vgdb_create.sql`. Update the seed data there, then run:

```
make db
```

This will rebuild the database.

### Running locally

Using the [Nightly](https://doc.rust-lang.org/nightly/book/appendix-07-nightly-rust.html) release of Rust (`rustup default nightly`), run the following commands:

```
cargo install
cargo run
```

This will start the server at `localhost:8000` 🚀🚀🚀

## Build

⚠️ WIP: will add soon

This project comes with a Docker image ready to deploy. Simply run:

```
make build
```

✨ sqlite can live within the same Docker container, saving you a separate DB deployment! Of course, this setup is readonly, and modifying the DB requires a redeploy. But for many projects this isn’t a concern, and you reap the benefits of a lightning fast colocated database, and the low-memory + multi-concurrent performance of a non-virtual DB.
