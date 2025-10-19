# Rust Webserver

The aim of this project is to test a number of things using Rust:

### Complete ✅

- Compile-time checked SQL statements for a PostgreSQL database, using [`sqlx`](https://github.com/launchbadge/sqlx)

### Todo 🔮

- Rust webserver using Axum
- OpenAPI spec generation using [`utoipa`](https://github.com/juhaku/utoipa)
- Scalar API client

## Setup

1. Install the `sqxl` cli:

```sh
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

2. Run Postgres in the background using docker compose:

```sh
docker-compose up -d
```

3. Create the database:

```sh
sqlx db create
```

4. Run sql migrations

```sh
(cd sqlx_lib && sqlx migrate run)
```

## Build/Run

If the database structure changes or we add new queries in code, the query metadata will need to be updated by running `cargo sqlx prepare --workspace`. The build uses the cached queries in the `.sqlx`folder, this can be switched off using the `SQLX_OFFLINE`variable in the `.env`file.

`cargo build` to build the lib and exe, `cargo run` to run the exe (this requires a running database will clear the database table and add a single row back in).
