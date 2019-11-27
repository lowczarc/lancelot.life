# lancelot.life

My personnal website written in Rust 🦀 from scratch

### Usage
You need launch a mysql server first and define `MYSQL_USER`, `MYSQL_PASSWORD` and `MYSQL_DATABASE` environnement variables.

Then lauch
```rust
cargo run --release
```
and the server will be listening on http://localhost:5432 😄

### TODO:

- Get projects from airtable
- Add support for array in get_var_value in views.rs
