import random
import hashlib

from zkrypto.prime import Prime
from zkrypto.generator import Generator

class ChaumPedersenProver:

    def __init__(self, bits=20, default=False):
        # Generate parameters (p, q, g) using Generator module
        if default:
            self.p, self.q, (self.g, self.h) = Generator.default

        else:
            self.p, self.q, (self.g, self.h) = Generator.get_generator_prime(random.randint(bits >> 1, bits), 2)

        self._k_bl = set() # k blacklist to avoid repeating

        self.r1 = None
        self.r2 = None
        self.s = None
        # Generate key pair
        self.gen_keypair()

    def params(self):
        # Returns the parameters
        return (self.y1, self.y2, self.p, self.q, self.g, self.h)

    
    def gen_keypair(self):
        # Generate a keypair (private key x, public key y) given p, g and h
        # Generate private key x
        self._x = random.randint(1, self.p - 2)
        # Compute public key y = g^x mod p
        self.y1 = pow(self.g, self._x, self.p)
        self.y2 = pow(self.h, self._x, self.p)
        return self._x, self.y1, self.y2

    def prove_async(self, c=None):
        # Prover's part of the ChaumPedersen protocol

        self.prove_sync_a()


        # Compute challenge c = H(r1 || r2 || y1 || y2)
        if c is None:
            c = int(hashlib.sha256((str(self.r1) + str(self.r2) + str(self.y1) + str(self.y2)).encode()).hexdigest(), 16) % self.p
        
        s = self.prove_sync_b(c)

        return self.r1, self.r2, c, s

    def prove(self):
        return self.prove_async()

    def prove_sync_a(self):
        # Prover's part of the ChaumPedersen protocol

        # Choose a random nonce k
        ## Problem: You need to keep track of which k you use.
        ## If you repeat the k, you leak your x
        self._k = random.randint(1, self.p - 2)
        while  self._k  in self._k_bl:
            self._k  = random.randint(1, self.p - 2)
        self._k_bl.add( self._k )
        
        # Compute commitment t = g^k mod p
        self.r1 = pow(self.g, self._k, self.p)
        self.r2 = pow(self.h, self._k, self.p)
        return self.r1, self.r2
    
    def prove_sync_b(self, c):
        # Compute response s = (k + x * c) % q
        self.s = (self._k - (self._x * c % self.q)) % self.q
        return self.s
    
class ChaumPedersenVerifier():
    def __init__(self,y1, y2, p=None, q=None, g=None, h=None, default=False):

        if default:
            self.p, self.q, (self.g, self.h) = Generator.default
        else:
            self.p = p
            self.q = q
            self.g = g
            self.h = h

        self.y1 = y1
        self.y2 = y2
    
    def verify(self, r1, r2, c, s):
        return self.verify_async(r1, r2, c, s)

    def verify_async(self, r1, r2, c, s):
        self.r1 = r1
        self.r2 = r2
        self.c = c 
        # Verify the commitment is a proper challenge
        c_ =  int(hashlib.sha256((str(self.r1) + str(self.r2) + str(self.y1) + str(self.y2)).encode()).hexdigest(), 16) % self.p
        if self.c != c_:
            return False
        return self.verify_sync_b(s)

    def verify_sync_a(self, r1, r2):
        self.r1 = r1
        self.r2 = r2
        self.c = random.randint(1, self.p - 2)
        return self.c
    
    def verify_sync_b(self, s):
        # Verifier's part of the Chaum-Pedersen protocol
        # Compute r' = (g^s * y^(c)) % p
        r1_prime = (pow(self.g, s, self.p) * pow(self.y1, self.c, self.p)) % self.p
        r2_prime = (pow(self.h, s, self.p) * pow(self.y2, self.c, self.p)) % self.p
        return self.r1 == r1_prime and self.r2 == r2_prime