FROM rust:latest
WORKDIR /app
COPY . /app
RUN cargo build --release --bins
EXPOSE 8000
CMD ["/app/target/release/server"]