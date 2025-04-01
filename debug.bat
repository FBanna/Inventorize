cd frontend
start "Frontend" cmd /k call yarn dev

cd ..
start "Backend" cmd /k call cargo run 

exit