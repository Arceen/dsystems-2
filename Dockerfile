FROM rust:latest AS builder
WORKDIR /peer_chat
COPY . .
RUN cargo install --path .

FROM amazonlinux:2023 AS final
COPY --from=builder /usr/local/cargo/bin/peer_chat /usr/local/bin/peer_chat
#CMD ["peer_chat"]

