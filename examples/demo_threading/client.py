#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

import json
import socket
import random
import time
import uuid

class Client():
    def __init__(self, target_host, target_port):
        self.target_host = target_host
        self.target_port = target_port

    def send_json(self, obj):
        client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.connect((self.target_host, self.target_port))
        client.send(json.dumps(obj).encode("utf-8"))
        return json.loads(client.recv(4096).decode("utf-8"))

if __name__ == "__main__":
    client = Client("127.0.0.1", 8080)
    uuids = []

    # Create 100 sensors
    for i in range(100):
        uuids.append(str(uuid.uuid4()))

    # Load the sensors with initial random values
    for id in uuids:
        client.send_json({"command": {"power": {"value": random.randrange(1, 1000) / 10.0}}, "uuid": id })

    # Randomly change their values with a delay
    for id in uuids:
        client.send_json({"command": {"power": {"value": random.randrange(1, 1000) / 10.0}}, "uuid": id })
        # 0.001 - 0.01, frametime is 0.016
        time.sleep(random.randrange(1, 10) / 1000)

    # Disconnect the sensors
    for id in uuids:
        client.send_json({"command": "disconnect", "uuid": id})
