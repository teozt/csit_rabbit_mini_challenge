FROM rust:slim as builder
WORKDIR /tmp/app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt update && apt upgrade -y
COPY --from=builder /tmp/app/target/release/csit-rest-mongo /usr/local/bin/csit-rest-mongo
EXPOSE 8080
CMD ["csit-rest-mongo"]