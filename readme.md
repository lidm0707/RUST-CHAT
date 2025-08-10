![Alt text](image_readme/poc1.png)





dx serve --package frontend
cargo run -p backend --bin backend
cargo add actix-cors --manifest-path backend/Cargo.toml
cargo run --bin hash_password -- --password asd


cargo search cargo-watchexec


lsof -i :8997
kill -9 <PID>
