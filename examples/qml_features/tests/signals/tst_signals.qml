// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "SignalsTests"

    readonly property url kdabUrl: "https://kdab.com"
    readonly property url kdabAboutUrl: "https://kdab.com/about"

    Component {
        id: componentSignals

        RustSignals {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_connect() {
        const obj = createTemporaryObject(componentSignals, null, {});
        const connectedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "connected",
            target: obj,
        });
        const errorSpy = createTemporaryObject(componentSpy, null, {
            signalName: "error",
            target: obj,
        });
        compare(connectedSpy.count, 0);
        compare(errorSpy.count, 0);

        // Connect to a valid url
        obj.connect("https://kdab.com");
        compare(connectedSpy.count, 1);
        compare(errorSpy.count, 0);
        compare(connectedSpy.signalArguments[0].length, 1);
        compare(connectedSpy.signalArguments[0][0], "https://kdab.com");

        // Connect to an invalid url
        obj.connect("https://github.com/kdab/cxx-qt");
        compare(connectedSpy.count, 1);
        compare(errorSpy.count, 1);
        compare(errorSpy.signalArguments[0].length, 1);
        compare(errorSpy.signalArguments[0][0], "URL does not start with https://kdab.com");
    }

    function test_disconnect() {
        const obj = createTemporaryObject(componentSignals, null, {});
        const disconnectedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "disconnected",
            target: obj,
        });
        compare(disconnectedSpy.count, 0);

        obj.disconnect();
        compare(disconnectedSpy.count, 1);
    }
}
