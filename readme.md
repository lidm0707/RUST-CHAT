dx serve --package frontend
cargo run -p backend --bin backend
cargo add actix-cors --manifest-path backend/Cargo.toml
cargo run --bin hash_password -- --password asd
