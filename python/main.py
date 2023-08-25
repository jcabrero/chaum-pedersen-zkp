import random
import hashlib
from zkrypto import SchnorrProver, SchnorrVerifier, \
                    ChaumPedersenProver, ChaumPedersenVerifier

def test_schnorr_zkp():
    prover = SchnorrProver()
    verifier = SchnorrVerifier(*prover.params())

    print("Verification result:", verifier.verify(*prover.prove()))

def test_chaum_pedersen_zkp():
    prover = ChaumPedersenProver()
    verifier = ChaumPedersenVerifier(*prover.params())

    print("Verification result:", verifier.verify(*prover.prove()))
if __name__ == "__main__":
    test_schnorr_zkp()
    test_chaum_pedersen_zkp()