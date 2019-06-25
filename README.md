# lancelot.life

My personnal website written in Rust from scratch

### Usage
You need launch a mysql server first and define `MYSQL_USER`, `MYSQL_PASSWORD` and `MYSQL_DATABASE` environnement variables.

Then lauch
```rust
cargo run --release
```
and the server will be listening on https://localhost:5432

### TODO:

- Get articles and project from airtable
- Remove build.rs and include! in include_view! and interprete views file at compile time
- Write articles !
