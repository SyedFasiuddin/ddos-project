#!/usr/bin/env python3

import socket
import time

IP = "127.0.0.1"
PORT = 8000

request = [
        "GET /test HTTP/1.1",
        f"Host: {IP}:{PORT}",
        "User-agent: Mozilla/5.0 (Windows NT 6.3; rv:36.0) Gecko/20100101 Firefox/36.0",
        "Accept-language: en-US,en,q=0.5",
        "Connection: keep-alive",
        "Keep-Alive: timeout=100, max=1000",
    ]
request = "\r\n".join(request)

def main():
    with socket.socket() as s:
        s.connect((IP, PORT))
        s.sendall(str.encode(request))
        while True:
            s.send(b"X-a 10000\r\n")
            time.sleep(1)


if __name__ == "__main__":
    main()
