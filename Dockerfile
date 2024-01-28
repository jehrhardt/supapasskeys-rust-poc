# Builder stage
FROM rust:1.75.0-bookworm as builder

WORKDIR /app

COPY . .
RUN cargo build --release

# Final stage
FROM ubuntu:jammy-20240111

ENV BIND_ADDRESS=::
CMD ["supapasskeys"]

COPY --from=builder /app/target/release/main /usr/local/bin/supapasskeys
