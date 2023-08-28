# Chaum-Pedersen Sigma Protocol Implementation

![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/jcabrero/chaum-pedersen-zkp/test_python.yml?style=for-the-badge&label=Build%20Python)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/jcabrero/chaum-pedersen-zkp/test_rust.yml?style=for-the-badge&label=Build%20Rust)
![GitHub](https://img.shields.io/github/license/jcabrero/chaum-pedersen-zkp?style=for-the-badge)



This project contains implementations of Chaum-Pedersen cryptographic protocols in multiple languages (Python and Rust) with gRPC.

## Features
:white_check_mark: Python Implementation:
- Technical Features: 
  - Developed using Python's standard libraries with no external dependencies.
  - Native support for big integers in Python.
  - Includes gRPC support for communication.
  - Comprehensive test suite for ensuring functionality.

- Cryptographic Features:
  - Automatic generation and verification of prime numbers and generators.
  - Implementation of the Schnorr Sigma Protocol.
  - Implementation of the Chaum-Pedersen Sigma Protocol.
  - Introduction of an *asynchronous* Chaum-Pedersen Sigma Protocol.

    
:white_check_mark: Rust Implementation:
- Technical Features: 
  - Utilizes the `BigInt` type for handling large numbers.
  - Incorporates gRPC support with server and client components.
  - Comprehensive unit test suite for thorough testing.
- Cryptographic Features:
  - Automatic generation and verification of prime numbers and generators.
  - Implementation of the Chaum-Pedersen Sigma Protocol.

:white_check_mark: Docker:
- Includes a Multi-Stage `Dockerfile` that can be used to build all the different images.
- Includes a docker-compose file that enables launching both `python` and `rust` clients and servers simultaneously.
- Includes a `devcontainer.json` for fast setup of the project both on Github and VSCode.

:white_check_mark: Continuous Integration:
- Includes a Github Actions implementations for CI testing of Python and Rust.
  

## Description

This project aims to demonstrate cryptographic protocols through implementations in both Python and Rust. The project structure includes multiple components, such as the Python client and server implementations, protocol definitions in the `proto` directory, Rust source code in the `src` directory, and tests in the `tests` directory.

## Usage

- The `python` directory contains Python client and server implementations. You can run the server using `server.py` and interact with it using `client.py`.
- The `docker compose` file can run a test Python client-server architecture.
- The `src` directory contains Rust source code implementing cryptographic protocols. You can build and run the Rust code using Cargo.
- The `tests` directory includes tests for the Rust codebase.


## Run Rust and Python Client and Server(x86_64)

To run both the client and servers you do it with docker.
```bash
docker compose up
```

To cleanup:
```bash
docker compose down -v --rmi all
```

## Rust
### Build and Run Client-Server

```bash
cargo build
export RUST_LOG=debug # To see the debug logs where params are printed
cargo run --bin server # For the server on one terminal
cargo run --bin client # For the client on other terminal 
```

### Run Tests

```bash
cargo test
```

## Python
### Build and Run Client-Server

```bash
cd python
python3 server.py # For the server on one terminal
python3 client.py # For the client on other terminal 
```
### Run Tests

```bash
pip3 install pytest
cd python
pytest
```
## Disclaimer
This implementation is provided for educational purposes and should not be used in production environments without proper security review. The code is meant to demonstrate the basic concept of the Chaum-Pedersen protocol and may not include all necessary security features.

## License
This project is licensed under the GPL3 License - see the LICENSE file for details.

## Author
José Cabrero Holgueras
[jose@cabreroholgueras.com](mailto:jose@cabreroholgueras.com)

[![image](https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white)](https://www.linkedin.com/in/jose-cabrero-holgueras/)
[![image](https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/jcabreroholg)
