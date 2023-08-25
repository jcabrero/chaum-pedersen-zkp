import random
import hashlib

from zkrypto.prime import Prime
from zkrypto.generator import Generator

class ChaumPedersenProver:

    def __init__(self, bits=20):
        # Generate parameters (p, q, g) using Generator module
        p, q, G = Generator.get_generator_prime(random.randint(bits >> 1, bits), 2)
        self.p = p
        self.q = q
        self.g = G[0]
        self.h = G[1]
        self.k_bl = set() # k blacklist to avoid repeating

    def params(self):
        # Returns the parameters
        return (self.p, self.q, self.g, self.h)

    
    def gen_keypair(self):
        # Generate a keypair (private key x, public key y) given p, g and h
        # Generate private key x
        self._x = random.randint(1, self.p - 2)
        # Compute public key y = g^x mod p
        self.y1 = pow(self.g, self._x, self.p)
        self.y2 = pow(self.h, self._x, self.p)
        return self._x, self.y1, self.y2

    def prove(self):
        # Prover's part of the Schnorr protocol

        # Generate key pair
        self.gen_keypair()

        # Choose a random nonce k
        ## Problem: You need to keep track of which k you use.
        ## If you repeat the k, you leak your x
        k = random.randint(1, self.p - 2)
        while k in self.k_bl:
            k = random.randint(1, self.p - 2)
        self.k_bl.add(k)
        
        # Compute commitment t = g^k mod p
        r1 = pow(self.g, k, self.p)
        r2 = pow(self.h, k, self.p)

        # Compute challenge c = H(r1 || r2 || y1 || y2)
        c = int(hashlib.sha256((str(r1) + str(r2) + str(self.y1) + str(self.y2)).encode()).hexdigest(), 16)
        
        # Compute response s = (k + x * e) % q
        s = (k - (self._x * c % self.q)) % self.q
        return self.y1, self.y2, r1, r2, c, s


class ChaumPedersenVerifier():
    def __init__(self, p, q, g, h):
        self.p = p
        self.q = q
        self.g = g
        self.h = h
    
    
    def verify(self, y1, y2, r1, r2, c, s):
        # Verifier's part of the Schnorr protocol
        # Compute r' = (g^s * y^(-e)) % p
        r1_prime = (pow(self.g, s, self.p) * pow(y1, c, self.p)) % self.p
        r2_prime = (pow(self.h, s, self.p) * pow(y2, c, self.p)) % self.p
        return r1 == r1_prime and r2 == r2_prime

