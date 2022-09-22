FROM rust:1.62 as build
COPY . .
RUN cargo build


FROM debian:buster-slim
COPY --from=build ./target/debug/beerated-backend ./target/debug/beerated-backend
EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0

CMD ["/target/debug/beerated-backend"]
