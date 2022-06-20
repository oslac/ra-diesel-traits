# README

Example of Rust-Analyzer failing to resolve some methods in `diesel::ExpressionMethods` and `diesel::RunQueryDsl`.

## Instructions

Requires **Sqlite3** & `diesel_cli`.

1. `diesel migration run`
2. `diesel migration redo`
3. `cargo r`

## Versions

- Rustc: `rustc 1.63.0-nightly (bb8c2f411 2022-06-19)`
- RA: `rust-analyzer version: 0.0.0 (427061da1 2022-06-19)`
- Code-Insider: `1.69.0-insider
fbf1cf3832d43088e27837dbb68d24ab65a098c1
x64`
- Diesel: `diesel = { version = "1.4.8", features = ["sqlite"] }`
- Diesel CLI: `diesel 1.4.1`
- sqlite3: `3.37.2`
- Ubuntu `22.04 LTS`
