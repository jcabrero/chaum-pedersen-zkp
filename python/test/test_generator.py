import pytest
from zkrypto import Generator

class TestGenerator:
    @staticmethod
    def test_get_generator_prime():
        # Test case for get_generator_prime method
        
        bits = 1024
        p, q, g = Generator.get_generator_prime(bits)
        
        assert isinstance(p, int)
        assert isinstance(q, int)
        assert isinstance(g, int)
        
        assert p > q
        assert p > g
        assert q > 0
        assert g > 1
    
    # Add more test cases as needed

# Run pytest when the script is executed
if __name__ == "__main__":
    pytest.main()