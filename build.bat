@echo off

cd frontend
call npm run build
cd ..

cargo build --release
PAUSE