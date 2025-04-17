cd frontend

start "Frontend BUILD" cmd /k call watchexec -r yarn build
start "Frontend DEBUG" cmd /k call yarn dev


cd ../target/debug
start "Backend" cmd /k call watchexec -w ../../ -e rs -r cargo run

exit