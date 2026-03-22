# Use the official Rust image
FROM rust:1.70 as builder

# Set the working directory
WORKDIR /usr/src/chat-microservice

# Copy and build
COPY . .
RUN cargo build --release

# Use a lightweight base image for the final stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder
COPY --from=builder /usr/src/chat-microservice/target/release/chat-microservice /usr/local/bin/

# Expose the port
EXPOSE 8080

# Run the service
CMD ["chat-microservice"]