#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

import json
import socket
import random
import time
import uuid


class Client():
    def __init__(self, target_host, target_port, uuid, max_power):
        self.target_host = target_host
        self.target_port = target_port
        self.uuid = str(uuid)
        self.power = random.randrange(10, max_power)
        self.max_power = max_power

    def send_disconnect(self):
        self.send_json({"command": "disconnect", "uuid": client.uuid})

    def send_json(self, obj):
        client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.connect((self.target_host, self.target_port))
        client.send(json.dumps(obj).encode("utf-8"))
        return json.loads(client.recv(4096).decode("utf-8"))

    def send_power(self):
        self.power = max(min(self.power + random.randrange(-100, 100), self.max_power), 10)
        self.send_json({"command": {"power": {"value": self.power}},
                        "uuid": self.uuid})


if __name__ == "__main__":
    addr = ("127.0.0.1", 8080)
    client01 = Client(*addr, "45b0836f-ae80-46f4-a311-0044cdf26e3d", 200)
    client11 = Client(*addr, "acd42f5b-6056-4310-a746-7a8d9ebe7127", 2000)
    client12 = Client(*addr, "1f085cca-4008-4784-87c7-f6c21ac0369f", 20)
    client13 = Client(*addr, "c48169e3-9b7f-4c51-8838-2e027c85ead3", 1500)
    client21 = Client(*addr, "452ba07a-f798-4f82-b76e-0fb11b926cf4", 100)
    client22 = Client(*addr, "5c293d20-870b-4f85-8e71-49eebb34bf3e", 150)
    client23 = Client(*addr, "ee1c3343-83ed-43b0-98d1-8d59cd7291ae", 120)
    client24 = Client(*addr, "fb7f706d-ee88-41a7-862b-4002fdeb9fc8", 350)
    client31 = Client(*addr, "3e3f1174-6aaf-4357-93ac-b3d9285d7af8", 200)

    clients = [
        client01,
        client11,
        client12,
        client13,
        client21,
        client22,
        client23,
        client24,
        client31,
    ]

    while True:
        for client in clients:
            if random.randrange(1, 20) == 1:
                client.send_disconnect()
            else:
                client.send_power()

            time.sleep(0.16)


