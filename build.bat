@echo off

cd frontend
call npm run build
cd ..
call cargo build --release
PAUSE