## This Dockerfile is a multi-stage build that sets up separate images for
## Python clients and servers, as well as Rust clients and servers.

# Define a base image for Python
FROM python:3.11-slim as python_base

# Define an argument for the server name
ARG SERVER_NAME="python_server"
# Set the working directory
WORKDIR /root/python/
# Install grpcio-tools using pip
RUN pip3 install grpcio-tools
# Copy the contents of the "python" directory to the working directory
COPY python/ .
# Define an image for the Python client
FROM python_base as python_client
# Set the command to run the Python client script with the specified server name
CMD ["python3", "client.py", "python_server"]
# Define an image for the Python server
FROM python_base as python_server
# Set the command to run the Python server script
CMD ["python3", "server.py"]

# Define a base image for Rust
FROM rust:1.67 as rust_base
RUN apt-get update -y &&\
 apt-get upgrade -y &&\
 apt install -y protobuf-compiler libprotobuf-dev &&\
 rustup target add x86_64-unknown-linux-musl
# Create a directory for the target Rust files
RUN mkdir /root/target
# Set the working directory
WORKDIR /root/
# Copy the build.rs, Cargo.toml, and Cargo.lock files to the working directory
COPY build.rs Cargo.toml Cargo.lock  /root/
# Copy the contents of the "src" directory to the working directory
COPY ./src /root/src
# Build the Rust project targeting the x86_64-unknown-linux-musl platform
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch as rust_base_prime

# Set environment variables for Rust logging and backtrace (errors)
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1

# Create an image for the Rust server
FROM rust_base_prime as rust_server

# Copy the compiled Rust server binary to the root directory
COPY --from=rust_base /root/target/x86_64-unknown-linux-musl/release/server /root/server
# Set the command to run the Rust server binary
CMD ["/root/server"]
# Create an image for the Rust client
FROM rust_base_prime as rust_client
# Copy the compiled Rust client binary to the root directory
COPY --from=rust_base /root/target/x86_64-unknown-linux-musl/release/client /root/client
# Set the command to run the Rust client binary with the specified server name
CMD ["/root/client", "rust_server"]
