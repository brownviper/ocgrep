# Stage 1: Build
FROM rust:slim AS builder
WORKDIR /app
COPY . .
# Build for release to get maximum grep-like speed
RUN cargo build --release

# Stage 2: Run
FROM debian:bookworm-slim
# Copy only the compiled binary from the builder stage
COPY --from=builder /app/target/release/ocgrep /usr/local/bin/ocgrep
WORKDIR /workspace

# Set the binary as the entrypoint so the container acts like a CLI tool
ENTRYPOINT ["ocgrep"]
