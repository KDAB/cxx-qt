#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

import json
import socket
import random
import uuid


def random_power():
    return random.randrange(1, 1000) / 10.0


def uuid_str():
    return str(uuid.uuid4())


class Client():
    def __init__(self, target_host, target_port):
        self.target_host = target_host
        self.target_port = target_port

    def flood(self):
        while True:
            self.send_json({"command": {"power": {"value": random_power()}},
                            "uuid": uuid_str()})

    def invalid_command(self):
        print(self.send_json({"command": "unknown", "uuid": uuid_str()}))

    def invalid_json(self):
        print(self.send_string("invalid json"))

    def invalid_json_long(self):
        print(self.send_string("A" * 1024 * 1024))

    def invalid_power(self):
        print(self.send_json({"command": {"power": {"value": -50.0}},
                              "uuid": uuid_str()}))

    def invalid_utf8(self):
        print(self.send_raw(b"\x00\x9f\x92\x96"))

    def invalid_uuid(self):
        print(self.send_json({"command": {"power": {"value": random_power()}},
                              "uuid": "invaliduuid"}))

    def send_json(self, obj):
        return json.loads(self.send_string(json.dumps(obj)))

    def send_raw(self, raw):
        client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        client.connect((self.target_host, self.target_port))
        client.send(raw)
        return client.recv(4096)

    def send_string(self, string):
        return self.send_raw(string.encode("utf-8")).decode("utf-8")


if __name__ == "__main__":
    client = Client("127.0.0.1", 8080)
    print("C - Invalid Command")
    print("F - Flood with multiple UUID")
    print("J - Invalid JSON")
    print("L - Long Invalid JSON")
    print("P - Invalid Power")
    print("T - Invalid UTF-8")
    print("U - Invalid UUID")
    val = input("Option:").strip().upper()
    if val == "C":
        client.invalid_command()
    elif val == "F":
        client.flood()
    elif val == "J":
        client.invalid_json()
    elif val == "L":
        client.invalid_json_long()
    elif val == "P":
        client.invalid_power()
    elif val == "T":
        client.invalid_utf8()
    elif val == "U":
        client.invalid_uuid()
    else:
        print("Unknown command.")
