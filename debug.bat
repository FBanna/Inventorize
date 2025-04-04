cd frontend
start "Frontend" cmd /k call yarn dev

cd ../target/debug
start "Backend" cmd /k call cargo run 

exit