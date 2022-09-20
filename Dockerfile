FROM rust:latest
COPY . /app
RUN cd /app && cargo build --release
CMD ["/app/target/release/beerated-backend"]
