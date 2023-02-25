# RustSumCalculator

Setup (direct build):
- Run `cargo build --bins` to create the binaries.
- Run `./targets/debug/server` to start the server.
- In another console, run `./targets/debug/client` to run the client that connects to the server.

Setup (Docker):
- Run `docker build -t addserver:latest .` to build the Docker image.
- Run `docker run --name addserver -p 8000:8000 addserver` to start the server binary.
- Run `docker exec addserver /app/target/release/client` to run the client binary inside the container.

Assisted by AI