#!/bin/bash

# Start backend
cargo run --bin backend  &

# Wait for backend /health to respond via GET
until curl --silent --fail http://localhost:8997/health > /dev/null; do
    echo "⏳ Waiting for backend to be ready..."
    sleep 1
done

echo "✅ Backend ready! Starting frontend..."
dx serve --package frontend
