FROM rust:bullseye as builder
WORKDIR /usr/src/speech-bubble-bot
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/speech-bubble-bot /usr/local/bin/speech-bubble-bot
VOLUME /data
ENV DATA_DIR=/data
CMD ["speech-bubble-bot"]