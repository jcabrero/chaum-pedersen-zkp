import grpc
import sys
import logging
import time

from proto.zkp_pb2 import (
    RegisterRequest,
    AuthenticationChallengeRequest,
    AuthenticationAnswerRequest,
    AsyncAuthenticationRequest
)

import proto.zkp_pb2_grpc as pt

from zkrypto import ChaumPedersenProver, Generator

class ChaumPedersenProtocolClient():

    def __init__(self, user: str, endpoint="localhost") -> None:
        self.user = user
        self.prover = ChaumPedersenProver(default=True)
        self.channel = grpc.insecure_channel(endpoint+":50051")
        self.client= pt.AuthStub(self.channel)


    def Register(self):

        req = RegisterRequest(user=self.user, y1=self.prover.y1, y2=self.prover.y2)
        logging.debug("[%s] y1: %d" %(self.user, self.prover.y1))
        logging.debug("[%s] y2: %d" %(self.user, self.prover.y2))
        return self.client.Register(req)
    
    def Authenticate(self):
        r1, r2 = self.prover.prove_sync_a()
        req1 = AuthenticationChallengeRequest(user=self.user, r1=r1, r2=r2)
        res1 = self.client.CreateAuthenticationChallenge(req1)
        logging.debug("[%s] r1: %d" %(self.user, self.prover.r1))
        logging.debug("[%s] r2: %d" %(self.user, self.prover.r2))
        s = self.prover.prove_sync_b(res1.c)
        
        logging.debug("[%s] c: %d" %(self.user, res1.c))
        logging.debug("[%s] s: %d" %(self.user, self.prover.s))
        req2 = AuthenticationAnswerRequest(auth_id=res1.auth_id, s=s)


        res2 = self.client.VerifyAuthentication(req2)
        return res2.session_id
    
    def AuthenticateAsync(self):
        r1, r2, c, s = self.prover.prove_async()
        logging.debug("[%s] r1: %d" %(self.user, r1))
        logging.debug("[%s] r2: %d" %(self.user, r2))
        logging.debug("[%s] c: %d" %(self.user, c))
        logging.debug("[%s] s: %d" %(self.user, s))
        req = AsyncAuthenticationRequest(user=self.user, r1=r1, r2=r2, c=c, s=s)
        res = self.client.VerifyAuthenticationAsync(req)
        return res.session_id

def main(endpoint):
    usernames = [chr(ord("A") + i) * 3 for i in range(26)]
    clients = []
    for user in usernames:

        clients.append(ChaumPedersenProtocolClient(user, endpoint=endpoint))
        logging.info("Registering User: %s" % (user))
        clients[-1].Register()

    while True:

        for client in clients:
            logging.info("Authenticating User: %s" % (client.user))
            client.Authenticate()

            time.sleep(2)
            

            logging.info("Authenticating Async User: %s" % (client.user))
            client.AuthenticateAsync()
            time.sleep(2)
            

    
if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    if len(sys.argv) <= 1:
        main("localhost")
    else:
        main(sys.argv[1])