from concurrent import futures
import random
import hashlib
import logging
import grpc
import datetime

from proto.zkp_pb2 import (
    RegisterResponse,
    AuthenticationChallengeResponse,
    AuthenticationAnswerResponse
)

import proto.zkp_pb2_grpc as pt
from zkrypto import ChaumPedersenVerifier

def generate_non_colluding_random_string(colluding_set):

    id = hashlib.sha256((str(random.randint(0, 1 << 64))).encode()).hexdigest() # Generate a random 64 bit int auth id
    while id in colluding_set: # Check for potential collisions in self.auth_ids O(1)
        id = hashlib.sha256((str(random.randint(0, 1 << 64))).encode()).hexdigest() # Generate a random 64 bit int till non-colluding
    return id
        
class User:
    def __init__(self, name, y1, y2):
        self.name = name
        self.verifier = ChaumPedersenVerifier(y1, y2, default=True)
        self.auth_id = None

    def set_auth_id(self, auth_id):
        self.auth_id = auth_id
        return self.auth_id

    def __str__(self):
        return "User: %s - AuthID: [%s]" % (self.name, str(self.auth_id))

    def __repr__(self) -> str:
        return str(self)

class ChaumPedersenProtocolServer(pt.AuthServicer):

    def __init__(self) -> None:
        super().__init__()
        self.users = {}
        self.auth_ids = {}
        self.sessions = {}
        self.verifier = None


    def Register(self, request, context):
        if request.user in self.users:
            logging.warning("User %s already exists" % (request.user))
            context.abort(grpc.StatusCode.ALREADY_EXISTS, "User already exists")
        self.users[request.user] = User(request.user, request.y1, request.y2)
        logging.info("Succesfully registered user: %s" % (str(self.users[request.user])))
        return RegisterResponse()
    
    def CreateAuthenticationChallenge(self, request, context):
        # Check if user is registered
        if not request.user is None and not request.user in self.users:
            logging.warning("User %s not found" % (request.user))
            context.abort(grpc.StatusCode.NOT_FOUND, "User not found")
        
        user = self.users[request.user]

        auth_id = generate_non_colluding_random_string(self.auth_ids)
        user.set_auth_id(auth_id)
        self.auth_ids[auth_id] = user # Mapping user object too

        c = user.verifier.verify_sync_a(request.r1, request.r2) # Challenge
        logging.info("Created challenge for user: %s" % (str(self.users[request.user])))
        return AuthenticationChallengeResponse(auth_id=auth_id, c=c)
    
    def VerifyAuthentication(self, request, context):

        if not request.auth_id is None and not request.auth_id in self.auth_ids:
            context.abort(grpc.StatusCode.NOT_FOUND, "Auth ID not found")

        user = self.auth_ids.pop(request.auth_id)

        if user.auth_id != request.auth_id:
            logging.warning("Simultaneous authentication of user %s != %s" % (user.auth_id , request.auth_id))
            context.abort(grpc.StatusCode.ABORTED, "Simultaneous authentication of user %s != %s" % (user.auth_id , request.auth_id))
        
        verification_result = user.verifier.verify_sync_b(request.s)
        logging.info("Verification Successful for user: %s" % (str(user)))
        if not verification_result:
            logging.warning("Verification Unsuccesful %s != %s" % (user.auth_id , request.auth_id))
            context.abort(grpc.StatusCode.PERMISSION_DENIED, "Verification Unsuccesful")

        session_id = generate_non_colluding_random_string(self.sessions)
        expiration = datetime.datetime.now() + datetime.timedelta(days=1) # Expiration of key in 1 day from now

        self.sessions[session_id] = (user, expiration)
        
        # Cleanup of user auth_id
        user.set_auth_id(None)

        logging.info("Session ID: %s for User:\n%s\nExpiration: %s" % (session_id, str(user), str(expiration)))
        
        return AuthenticationAnswerResponse(session_id=session_id)

    def VerifyAuthenticationAsync(self, request, context):
        # Check if user is registered
        if not request.user is None and not request.user in self.users:
            logging.warning("User %s not found" % (request.user))
            context.abort(grpc.StatusCode.NOT_FOUND, "User not found")

        user = self.users[request.user]


        verification_result = user.verifier.verify_async(request.r1, request.r2, request.c, request.s)

        
        if not verification_result:
            logging.warning("Verification Unsuccesful %s != %s" % (user.auth_id , request.auth_id))
            context.abort(grpc.StatusCode.PERMISSION_DENIED, "Verification Unsuccesful")
        logging.info("Async Verification Successful for user: %s" % (str(user)))
        session_id = generate_non_colluding_random_string(self.sessions)
        expiration = datetime.datetime.now() + datetime.timedelta(days=1) # Expiration of key in 1 day from now

        self.sessions[session_id] = (user, expiration)
        

        logging.info("Async Session ID: %s for User:\n%s\nExpiration: %s" % (session_id, str(user), str(expiration)))
        return AuthenticationAnswerResponse(session_id=session_id)

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    pt.add_AuthServicer_to_server(
        ChaumPedersenProtocolServer(), server
    )
    server.add_insecure_port("0.0.0.0:50051")
    server.start()
    server.wait_for_termination()


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    serve()