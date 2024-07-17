Console 1
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target\debug\Web_Sync.exe`
Server starting on http://127.0.0.1:8080
Service 2 called. Waiting for Service 1...
Service 1 was called. Proceeding.
Service 1 called. Waiting for Service 2...
Both services called. Shutting down.
PS D:\Rust Domain\RustExample\Web_Sync>

console 2
Invoke-WebRequest -Uri "http://localhost:8080/service1" -Method POST

Console 3
Invoke-WebRequest -Uri "http://localhost:8080/service2" -Method POST
