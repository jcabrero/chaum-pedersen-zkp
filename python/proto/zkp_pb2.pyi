from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class RegisterRequest(_message.Message):
    __slots__ = ["user", "y1", "y2"]
    USER_FIELD_NUMBER: _ClassVar[int]
    Y1_FIELD_NUMBER: _ClassVar[int]
    Y2_FIELD_NUMBER: _ClassVar[int]
    user: str
    y1: int
    y2: int
    def __init__(self, user: _Optional[str] = ..., y1: _Optional[int] = ..., y2: _Optional[int] = ...) -> None: ...

class RegisterResponse(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class AuthenticationChallengeRequest(_message.Message):
    __slots__ = ["user", "r1", "r2"]
    USER_FIELD_NUMBER: _ClassVar[int]
    R1_FIELD_NUMBER: _ClassVar[int]
    R2_FIELD_NUMBER: _ClassVar[int]
    user: str
    r1: int
    r2: int
    def __init__(self, user: _Optional[str] = ..., r1: _Optional[int] = ..., r2: _Optional[int] = ...) -> None: ...

class AuthenticationChallengeResponse(_message.Message):
    __slots__ = ["auth_id", "c"]
    AUTH_ID_FIELD_NUMBER: _ClassVar[int]
    C_FIELD_NUMBER: _ClassVar[int]
    auth_id: str
    c: int
    def __init__(self, auth_id: _Optional[str] = ..., c: _Optional[int] = ...) -> None: ...

class AuthenticationAnswerRequest(_message.Message):
    __slots__ = ["auth_id", "s"]
    AUTH_ID_FIELD_NUMBER: _ClassVar[int]
    S_FIELD_NUMBER: _ClassVar[int]
    auth_id: str
    s: int
    def __init__(self, auth_id: _Optional[str] = ..., s: _Optional[int] = ...) -> None: ...

class AuthenticationAnswerResponse(_message.Message):
    __slots__ = ["session_id"]
    SESSION_ID_FIELD_NUMBER: _ClassVar[int]
    session_id: str
    def __init__(self, session_id: _Optional[str] = ...) -> None: ...

class AsyncAuthenticationRequest(_message.Message):
    __slots__ = ["user", "r1", "r2", "c", "s"]
    USER_FIELD_NUMBER: _ClassVar[int]
    R1_FIELD_NUMBER: _ClassVar[int]
    R2_FIELD_NUMBER: _ClassVar[int]
    C_FIELD_NUMBER: _ClassVar[int]
    S_FIELD_NUMBER: _ClassVar[int]
    user: str
    r1: int
    r2: int
    c: int
    s: int
    def __init__(self, user: _Optional[str] = ..., r1: _Optional[int] = ..., r2: _Optional[int] = ..., c: _Optional[int] = ..., s: _Optional[int] = ...) -> None: ...
