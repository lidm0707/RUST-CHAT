#!/bin/bash

# รัน backend ใน terminal ใหม่
gnome-terminal -- bash -c "
    cd backend || { echo 'backend folder not found'; exit 1; }
    watchexec -w src -r -- cargo run --bin backend;
    exec bash
"

# รอ backend พร้อม
until curl --silent --fail http://localhost:8997/health > /dev/null; do
    echo '⏳ Waiting for backend to be ready...'
    sleep 1
done

echo '✅ Backend ready! Starting frontend...'

# รัน frontend ใน terminal ใหม่
gnome-terminal -- bash -c "
    cd frontend || { echo 'frontend folder not found'; exit 1; }
    dx serve --package frontend;
    exec bash
"
