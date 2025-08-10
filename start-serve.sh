#!/bin/bash

check_backend() {
    curl --silent --fail http://localhost:8997/health > /dev/null
}

# ยิงเช็ค backend 2 รอบ รอบละ 2 วิ
attempt=0
max_attempts=2
while [ $attempt -lt $max_attempts ]; do
    if check_backend; then
        echo "✅ Backend is already running."
        break
    else
        echo "⏳ Backend not ready, attempt $((attempt+1))/$max_attempts..."
        sleep 2
    fi
    attempt=$((attempt + 1))
done

# ถ้ายังไม่พร้อมหลัง 2 รอบ ให้เปิด backend ใหม่
if ! check_backend; then
    echo "🚀 Starting backend..."
    gnome-terminal -- bash -c "
        cd backend || { echo 'backend folder not found'; exit 1; }
        watchexec -w src -r -- cargo run --bin backend;
        exec bash
    "
fi

# รอ backend พร้อม (รอจนกว่าจะเช็คผ่าน)
until check_backend; do
    echo '⏳ Waiting for backend to be ready...'
    sleep 1
done

echo '✅ Backend ready! Starting frontend...'

# รัน frontend ใน terminal เดิม
cd frontend || { echo "frontend folder not found"; exit 1; }
dx serve --package frontend
