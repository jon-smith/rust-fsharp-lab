# rust-fsharp-lab

[![build](https://github.com/jon-smith/rust-fsharp-lab/actions/workflows/build.yaml/badge.svg)](https://github.com/jon-smith/rust-fsharp-lab/actions/workflows/build.yaml)

Playing around with rust and F# web-servers. Specifically testing utopia in rust and Oxpecker for F#.

## Contents

### [`rust/`](rust/)

- Rust webserver built with axum
- Postgres database, using sqlx for compile-time checked queries
- OpenAPI spec generation using utoipa, with a Scalar page

### [`fsharp/`](fsharp/)

- Hello world F# webserver, build using [Oxpecker](https://github.com/Lanayx/Oxpecker/tree/main)

## Todo 🔮

- F# webserver
