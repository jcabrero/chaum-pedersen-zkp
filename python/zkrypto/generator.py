import random
from zkrypto.prime import Prime

class Generator(object):
    @staticmethod
    def get_generator_prime(bits, num=1):
        G = []
        # Generate a prime number q with (bits - 1) bits
        q = Prime.n_bit_prime(bits - 1)

        # Initialize p to 0
        p = 0

        # Find a prime p using q and Miller-Rabin primality test
        while not Prime.miller_rabin_primality_test(p):
            # Choose a random integer k in the range [1, 2^(bits-1)/2]
            k = random.randint(1, 1 << (bits >> 1))
            # Compute p as k*q + 1
            p = k * q + 1

        while True:
            # Choose a random integer h in the range [2, p - 1]
            h = random.randint(2, p - 1)
            # Compute g as h raised to the power (p - 1) // q modulo p
            g = pow(h, (p - 1) // q, p)
            
            # Check if g is not equal to 1, making it a potential generator
            if g != 1:
                G.append(g)
                if len(G) == num:
                    return p, q, G  # Return p, q, and g as generator parameters
    
    default = (33599304334943, 1820705773, [25395732195142, 12433296605365]) # Default params