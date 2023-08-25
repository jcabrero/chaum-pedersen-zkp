import pytest
from zkrypto import SchnorrProver, SchnorrVerifier

class TestSchnorrProtocol:
    @staticmethod
    def test_schnorr_protocol():
        # Create a SchnorrProver instance
        prover = SchnorrProver(bits=20)
        p, q, g = prover.params()

        # Generate proof from prover
        y, r, e, s = prover.prove()

        # Create a SchnorrVerifier instance
        verifier = SchnorrVerifier(p, q, g)

        # Verify the proof using the verifier
        verification_result = verifier.verify(y, r, e, s)
        assert verification_result

    @staticmethod
    def test_schnorr_protocol_fail():
        # Create a SchnorrProver instance
        prover = SchnorrProver(bits=20)
        p, q, g = prover.params()

        # Generate proof from prover
        y, r, e, s = prover.prove()

        # Create a SchnorrVerifier instance
        verifier = SchnorrVerifier(p, q, g)

        # Verify the proof using the verifier
        verification_result = verifier.verify(y + 1, r, e, s)
        assert verification_result == False

# Run pytest when the script is executed
if __name__ == "__main__":
    pytest.main()