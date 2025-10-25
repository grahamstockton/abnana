# abnana
Memoized AB testing framework written in Rust

Prerequisite installs
* sqlite
* sqlx-cli (cargo install sqlx-cli)

Setup steps
* if database not already on machine, run sqlx database create --database-url=sqlite://database.db
* set up database by running sqlx migrate run --database-url=sqlite://database.db

Commands
* start server with cargo run
* build with cargo build
* test with cargo test

