# Use the official Rust image as a parent image
FROM rust:1.70 as builder

# Set the working directory in the container
WORKDIR /usr/src/hyperpm

# Copy the current directory contents from the parent directory into the container
COPY ../ . 

# Build the application
RUN cargo build --release

# Start a new stage for a smaller final image
# Set a default base image
ARG BASE_IMAGE=debian:bullseye-slim
FROM ${BASE_IMAGE} AS runtime

# Install necessary dependencies
RUN if [ "${BASE_IMAGE}" = "debian:bullseye-slim" ]; then \
        apt-get update && apt-get install -y \
        nodejs \
        npm \
        && rm -rf /var/lib/apt/lists/*; \
    elif [ "${BASE_IMAGE}" = "archlinux/base" ]; then \
        pacman -Syu --noconfirm \
        nodejs \
        npm; \
    fi

# Copy the binary from the builder stage
COPY --from=builder /usr/src/hyperpm/target/release/hyperpm /usr/local/bin/hyperpm

# Set the working directory
WORKDIR /app

# Specify the entry point for the container
ENTRYPOINT ["hyperpm"]
