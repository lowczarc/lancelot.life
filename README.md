# lancelot.life

My personnal website written in Rust 🦀 from scratch

### Usage
You need launch a postgresql server first and define `DATABASE_URL` and `PORT` environnement variables.

Then lauch
```rust
cargo run --release
```
and the server will be listening on the port set in the environnement `PORT` 😄

### TODO:

- Add support for array in get_var_value in views.rs
