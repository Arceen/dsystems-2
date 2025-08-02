FROM rust:latest AS builder
WORKDIR /peer_chat
COPY . .
RUN cargo install --path .

# phase 2
FROM amazonlinux:2023
COPY --from=builder /usr/local/cargo/bin/peer_chat /usr/local/bin/peer_chat
