# Builder Stage
FROM rust:1.91.1-alpine3.22 AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

# Production Image Stage
FROM scratch AS prod
WORKDIR /app
COPY --from=builder /app/target/release/add-me-bro ./server

EXPOSE 3000

CMD ["./server"]
