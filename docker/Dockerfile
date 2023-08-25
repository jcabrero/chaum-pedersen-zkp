FROM python:3.11-slim as python_base

ARG SERVER_NAME="python_server"

WORKDIR /root/python/

RUN pip3 install grpcio-tools

COPY python/ .

FROM python_base as python_client

CMD ["python3", "client.py", "python_server"]

FROM python_base as python_server

CMD ["python3", "server.py"]