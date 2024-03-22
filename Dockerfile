# Use a specific version of Rust for consistency
FROM rust:latest as builder
LABEL authors="rami.bendhia"

WORKDIR /app
COPY . .

# Build your Rust application
RUN cargo build --release

# Final production image
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/service .

# Expose the port if your application listens on a specific port
# EXPOSE 8080

# Command to run the application
CMD ["./service"]
