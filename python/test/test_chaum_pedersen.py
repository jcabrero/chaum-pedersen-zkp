import pytest
from zkrypto import ChaumPedersenProver, ChaumPedersenVerifier

class TestSchnorrProtocol:
    @staticmethod
    def test_chaum_pedersen_protocol_async():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        assert isinstance(p, int)
        assert isinstance(q, int)
        assert isinstance(g, int)
        assert isinstance(h, int)
        assert isinstance(y1, int)
        assert isinstance(y2, int)

        # Generate proof from prover
        r1, r2, c, s = prover.prove()

        assert isinstance(r1, int)
        assert isinstance(r2, int)
        assert isinstance(c, int)
        assert isinstance(s, int)

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # Verify the proof using the verifier
        verification_result = verifier.verify(r1, r2, c, s)
        assert verification_result


    @staticmethod
    def test_chaum_pedersen_protocol_sync():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        assert isinstance(p, int)
        assert isinstance(q, int)
        assert isinstance(g, int)
        assert isinstance(h, int)
        assert isinstance(y1, int)
        assert isinstance(y2, int)

        # Generate proof from prover
        r1, r2 = prover.prove_sync_a()

        assert isinstance(r1, int)
        assert isinstance(r2, int)

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # The verifier produces the challenge c
        c = verifier.verify_sync_a(r1, r2)
        assert isinstance(c, int)

        # The prover produces the proof s
        s = prover.prove_sync_b(c)
        assert isinstance(s, int)

        # Verify the proof s using the verifier
        verification_result = verifier.verify_sync_b(s)
        
        #verification_result = verifier.verify(r1, r2, c, s)
        assert verification_result

    @staticmethod
    def test_chaum_pedersen_protocol_async_default():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20, default=True)
        y1, y2, p, q, g, h= prover.params()

        assert isinstance(p, int)
        assert isinstance(q, int)
        assert isinstance(g, int)
        assert isinstance(h, int)
        assert isinstance(y1, int)
        assert isinstance(y2, int)

        # Generate proof from prover
        r1, r2, c, s = prover.prove()

        assert isinstance(r1, int)
        assert isinstance(r2, int)
        assert isinstance(c, int)
        assert isinstance(s, int)

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, default=True)

        # Verify the proof using the verifier
        verification_result = verifier.verify(r1, r2, c, s)
        assert verification_result


    @staticmethod
    def test_chaum_pedersen_protocol_sync_default():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20, default=True)
        y1, y2, p, q, g, h = prover.params()

        assert isinstance(p, int)
        assert isinstance(q, int)
        assert isinstance(g, int)
        assert isinstance(h, int)
        assert isinstance(y1, int)
        assert isinstance(y2, int)

        # Generate proof from prover
        r1, r2 = prover.prove_sync_a()

        assert isinstance(r1, int)
        assert isinstance(r2, int)

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, default=True)

        # The verifier produces the challenge c
        c = verifier.verify_sync_a(r1, r2)
        assert isinstance(c, int)

        # The prover produces the proof s
        s = prover.prove_sync_b(c)
        assert isinstance(s, int)

        # Verify the proof s using the verifier
        verification_result = verifier.verify_sync_b(s)
        
        #verification_result = verifier.verify(r1, r2, c, s)
        assert verification_result


    @staticmethod
    def test_chaum_pedersen_protocol_async_fail_r1():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        # Generate proof from prover
        r1, r2, c, s = prover.prove()

        r1 -= 1 # Modify r1

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # Verify the proof using the verifier
        verification_result = verifier.verify(r1, r2, c, s)
        assert verification_result == False

    @staticmethod
    def test_chaum_pedersen_protocol_sync_fail_r1():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        # Generate proof from prover
        r1, r2 = prover.prove_sync_a()

        r1 -= 1 # Modify r1

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # The verifier produces the challenge c
        c = verifier.verify_sync_a(r1, r2)

        # The prover produces the proof s
        s = prover.prove_sync_b(c)

        # Verify the proof s using the verifier
        verification_result = verifier.verify_sync_b(s)
        
        assert verification_result == False

    @staticmethod
    def test_chaum_pedersen_protocol_async_fail_r2():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        # Generate proof from prover
        r1, r2, c, s = prover.prove()

        r2 -= 1 # Modify r2

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # Verify the proof using the verifier
        verification_result = verifier.verify(r1, r2, c, s)
        assert verification_result == False

    @staticmethod
    def test_chaum_pedersen_protocol_sync_fail_r2():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        # Generate proof from prover
        r1, r2 = prover.prove_sync_a()

        r2 -= 1 # Modify r2

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # The verifier produces the challenge c
        c = verifier.verify_sync_a(r1, r2)

        # The prover produces the proof s
        s = prover.prove_sync_b(c)

        # Verify the proof s using the verifier
        verification_result = verifier.verify_sync_b(s)
        
        assert verification_result == False
    
    @staticmethod
    def test_chaum_pedersen_protocol_async_fail_c():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        # Generate proof from prover
        r1, r2, c, s = prover.prove()



        c -= 1 # Modify c
        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # Verify the proof using the verifier
        verification_result = verifier.verify(r1, r2, c, s)
        assert verification_result == False

    @staticmethod
    def test_chaum_pedersen_protocol_sync_fail_c():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        # Generate proof from prover
        r1, r2 = prover.prove_sync_a()

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # The verifier produces the challenge c
        c = verifier.verify_sync_a(r1, r2)

        c -= 1 # Modify c

        # The prover produces the proof s
        s = prover.prove_sync_b(c)

        # Verify the proof s using the verifier
        verification_result = verifier.verify_sync_b(s)
        
        assert verification_result == False

    @staticmethod
    def test_chaum_pedersen_protocol_async_fail_s():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        # Generate proof from prover
        r1, r2, c, s = prover.prove()

        s -= 1 # Modify s

        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # Verify the proof using the verifier
        verification_result = verifier.verify(r1, r2, c, s)
        assert verification_result == False

    @staticmethod
    def test_chaum_pedersen_protocol_sync_fail_s():
        # Create a ChaumPedersenProver instance
        prover = ChaumPedersenProver(bits=20)
        y1, y2, p, q, g, h = prover.params()

        # Generate proof from prover
        r1, r2 = prover.prove_sync_a()


        # Create a ChaumPedersenVerifier instance
        verifier = ChaumPedersenVerifier(y1, y2, p, q, g, h)

        # The verifier produces the challenge c
        c = verifier.verify_sync_a(r1, r2)

        # The prover produces the proof s
        s = prover.prove_sync_b(c)

        s -= 1 # Modify s

        # Verify the proof s using the verifier
        verification_result = verifier.verify_sync_b(s)
        
        assert verification_result == False
# Run pytest when the script is executed
if __name__ == "__main__":
    pytest.main()