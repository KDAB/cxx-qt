#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

import json
import socket
import random
import threading
import time
import uuid

RUNNING = True
KILLED = False


class Client():
    def __init__(self, target_host, target_port, uuid):
        self.target_host = target_host
        self.target_port = target_port
        self.uuid = str(uuid)

    def send_json(self, obj):
        client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.connect((self.target_host, self.target_port))
        client.send(json.dumps(obj).encode("utf-8"))
        return json.loads(client.recv(4096).decode("utf-8"))

    def send_power(self, value):
        self.send_json({"command": {"power": {"value": value}},
                        "uuid": self.uuid})


def client_thread():
    client = Client("127.0.0.1", 8080, uuid.uuid4())

    while RUNNING:
        client.send_power(random.randrange(1, 1000) / 10.0)

        time.sleep(0.25)

    if not KILLED:
        client.send_json({"command": "disconnect", "uuid": client.uuid})


if __name__ == "__main__":
    t = threading.Thread(target=client_thread)
    t.start()

    try:
        input("Press Enter to disconnect")
    except KeyboardInterrupt:
        KILLED = True

    RUNNING = False
    t.join()
