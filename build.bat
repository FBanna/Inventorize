@echo off


rd /s /q "target/release/dist"
call cargo build --release
cd frontend
call yarn build
move dist ../target/release
cd ..