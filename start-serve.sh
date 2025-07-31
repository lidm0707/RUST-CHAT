#!/bin/bash

# รัน dx serve สำหรับ frontend (รันใน background หรือ terminal ใหม่)
dx serve --package frontend &

# รัน backend server
cargo run -p backend
