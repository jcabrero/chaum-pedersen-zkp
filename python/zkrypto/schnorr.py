import random
import hashlib

from zkrypto.prime import Prime
from zkrypto.generator import Generator

class SchnorrProver():

    def __init__(self, bits=20):
        # Generate parameters (p, q, g) using Generator module
        p, q, G = Generator.get_generator_prime(random.randint(bits >> 1, bits))
        self.p = p
        self.q = q
        self.g = G[0]

    def params(self):
        # Returns the parameters
        return (self.p, self.q, self.g)

    # Generate a keypair (private key x, public key y) given p and g
    def gen_keypair(self):
        # Generate private key x
        self._x = random.randint(1, self.p - 2)
        # Compute public key y = g^x mod p
        self.y = pow(self.g, self._x, self.p)
        return self._x, self.y

    def prove(self):
        # Prover's part of the Schnorr protocol

        # Generate key pair
        self.gen_keypair()

        # Choose a random nonce k
        ## Problem: You need to keep track of which k you use.
        ## If you repeat the k, you leak your x
        k = random.randint(1, self.p - 2)

        # Compute commitment t = g^k mod p
        r = pow(self.g, k, self.p)

        # Compute challenge e = H(t || y)
        e = int(hashlib.sha256((str(r) + str(self.y)).encode()).hexdigest(), 16)
        
        # Compute response s = (k + x * e) % q
        s = (k + (self._x * e % self.q)) % self.q
        return self.y, r, e, s


class SchnorrVerifier():
    def __init__(self, p, q, g):
        self.p = p
        self.q = q
        self.g = g
    
    # Verifier's part of the Schnorr protocol
    def verify(self, y, r, e, s):
        # Compute r' = (g^s * y^(-e)) % p
        r_prime = (pow(self.g, s, self.p) * pow(y, -e, self.p)) % self.p
        return r_prime == r

