@echo off


rd /s /q "dist"
call cargo build --release
cd frontend
call yarn build
cd ..
move dist target/release