cd frontend

start "Frontend BUILD" cmd /k call watchexec -r npm run build
start "Frontend DEBUG" cmd /k call npm run dev


cd ../target/debug
start "Backend" cmd /k call watchexec -w ../../ -e rs -r cargo run

exit