import pytest
from Prime import Prime

class TestPrime:
    @staticmethod
    def test_n_bit_prime():
        # Test case for n_bit_prime method
        bits = 32
        prime = Prime.n_bit_prime(bits)
        assert Prime.miller_rabin_primality_test(prime)
        
    @staticmethod
    def test_euler_totient_function():
        # Test case for euler_totient_function method
        prime = 23
        phi = Prime.euler_totient_function(prime)
        assert phi == prime - 1
        
    @staticmethod
    def test_miller_rabin_primality_test():
        # Test case for miller_rabin_primality_test method
        prime = 101
        assert Prime.miller_rabin_primality_test(prime)
        
        composite = 100
        assert not Prime.miller_rabin_primality_test(composite)

# Run pytest when the script is executed
if __name__ == "__main__":
    pytest.main()