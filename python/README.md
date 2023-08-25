# Chaum-Pedersen and Schnorr $\Sigma$ Protocols

This repository contains a basic implementation of the Schnorr and Chaum-Pedersen protocol for non-interactive zero-knowledge proofs. The implementation includes a Schnorr and Chaum-Pedersen prover and a verifier.

## Features

- The `SchnorrProver` class generates proofs for the Schnorr protocol.
- The `SchnorrVerifier` class verifies the generated Schnorr proofs.
- The `ChaumPedersenProver` class generates proofs for the Chaum-Pedersen protocol.
- The `ChaumPedersenVerifier` class verifies the generated Chaum-Pedersen proofs.

## Usage

1. Clone the repository to your local machine.

2. Use the protocol:

```bash
python3 main.py
```

## Testing:
```bash
pytest
```
## Distributed Testing
Generating the `grpc`:


```bash
python3 -m grpc_tools.protoc -I./proto --python_out=./proto --pyi_out=proto --grpc_python_out=./proto proto/zkp.proto
```
Running the `docker` example:
```bash
docker compose up
```

To cleanup:
```bash
docker compose down -v --rmi all
```
## Disclaimer
This implementation is provided for educational purposes and should not be used in production environments without proper security review. The code is meant to demonstrate the basic concept of the Schnorr protocol and may not include all necessary security features.

## License
This project is licensed under the GPL3 License - see the LICENSE file for details.
