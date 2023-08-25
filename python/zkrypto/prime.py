import random

class Prime(object):
    @staticmethod
    def n_bit_prime(n):
        """Generate an n-bit prime number using random.choice() and the Miller-Rabin primality test."""
        while True:
            candidate = random.getrandbits(n)
            candidate |= 1  # Ensure the candidate is odd using OR instead of +=
            if Prime.miller_rabin_primality_test(candidate):
                return candidate

    @staticmethod
    def euler_totient_function(p):
        """Euler Totient Function is p-1 for any prime."""
        if Prime.miller_rabin_primality_test(p):
            return p - 1

    @staticmethod
    def miller_rabin_primality_test(n, k=10):
        """Use probabilistic primality testing to verify whether a number is prime or not."""
        if n <= 1:
            return False
        if n <= 3:
            return True
        
        # Write n - 1 as (2^s) * d where d is odd
        s = 0
        d = n - 1
        while d & 1 == 0:
            s += 1
            d >>= 1

        # Witness loop
        for _ in range(k):
            # Choose a random number
            a = random.randint(2, n - 1)
            x = pow(a, d, n)  # Compute a^d % n
            if x == 1 or x == n - 1:
                continue

            # Check whether it is a squared base.
            for _ in range(s - 1):
                x = pow(x, 2, n)  # Square x and take the modulo
                if x == n - 1:
                    break
            else:
                return False  # n is composite

        return True  # n is probably prime