
# Chaum-Pedersen Sigma Protocol Implementation

This project contains implementations of cryptographic protocols in multiple languages (Python and Rust) with gRPC.

## Description

This project aims to demonstrate cryptographic protocols through implementations in both Python and Rust. The project structure includes multiple components, such as the Python client and server implementations, protocol definitions in the `proto` directory, Rust source code in the `src` directory, and tests in the `tests` directory.

## Usage

- The `python` directory contains Python client and server implementations. You can run the server using `server.py` and interact with it using `client.py`.
- The `docker compose` file can run a test Python client-server architecture.
- The `src` directory contains Rust source code implementing cryptographic protocols. You can build and run the Rust code using Cargo.
- The `tests` directory includes tests for the Rust codebase.

## Disclaimer
This implementation is provided for educational purposes and should not be used in production environments without proper security review. The code is meant to demonstrate the basic concept of the Chaum-Pedersen protocol and may not include all necessary security features.

## License
This project is licensed under the GPL3 License - see the LICENSE file for details.
