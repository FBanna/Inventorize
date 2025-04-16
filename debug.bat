cd frontend
start "Frontend" cmd /k call watchexec -r yarn build

cd ../target/debug
start "Backend" cmd /k call watchexec -w ../../ -e rs -r cargo run

exit