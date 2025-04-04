@echo off

cd frontend
call yarn build
cd ..
call cargo build --release