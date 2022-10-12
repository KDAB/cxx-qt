// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "PropertiesTests"

    readonly property url kdabUrl: "https://kdab.com"
    readonly property url kdabAboutUrl: "https://kdab.com/about"

    Component {
        id: componentProperties

        RustProperties {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_connect_disconnect() {
        const obj = createTemporaryObject(componentProperties, null, {});
        const connectedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "connectedChanged",
            target: obj,
        });
        const connectedUrlSpy = createTemporaryObject(componentSpy, null, {
            signalName: "connectedUrlChanged",
            target: obj,
        });
        const previousConnectedUrlSpy = createTemporaryObject(componentSpy, null, {
            signalName: "previousConnectedUrlChanged",
            target: obj,
        });
        const statusSpy = createTemporaryObject(componentSpy, null, {
            signalName: "statusMessageChanged",
            target: obj,
        });

        compare(obj.connected, false);
        compare(obj.connectedUrl, "");
        compare(obj.previousConnectedUrl, "");
        compare(obj.statusMessage, "Disconnected");

        obj.connect("https://kdab.com");
        compare(connectedSpy.count, 1);
        compare(connectedUrlSpy.count, 1);
        compare(previousConnectedUrlSpy.count, 1);
        compare(statusSpy.count, 1);
        compare(obj.connected, true);
        compare(obj.connectedUrl, kdabUrl);
        compare(obj.previousConnectedUrl, "");
        compare(obj.statusMessage, "Connected");

        obj.connect("https://kdab.com/about");
        compare(connectedSpy.count, 2);
        compare(connectedUrlSpy.count, 2);
        compare(previousConnectedUrlSpy.count, 2);
        compare(statusSpy.count, 2);
        compare(obj.connected, true);
        compare(obj.connectedUrl, kdabAboutUrl);
        compare(obj.previousConnectedUrl, kdabUrl);
        compare(obj.statusMessage, "Connected");

        obj.disconnect();
        compare(connectedSpy.count, 3);
        compare(connectedUrlSpy.count, 3);
        compare(previousConnectedUrlSpy.count, 3);
        compare(statusSpy.count, 3);
        compare(obj.connected, false);
        compare(obj.connectedUrl, "");
        compare(obj.previousConnectedUrl, kdabAboutUrl);
        compare(obj.statusMessage, "Disconnected");

        obj.connect("https://github.com/kdab/cxx-qt");
        compare(connectedSpy.count, 4);
        compare(connectedUrlSpy.count, 3);
        compare(previousConnectedUrlSpy.count, 3);
        compare(statusSpy.count, 4);
        compare(obj.connected, false);
        compare(obj.connectedUrl, "");
        compare(obj.previousConnectedUrl, kdabAboutUrl);
        compare(obj.statusMessage, "URL does not start with https://kdab.com");
    }
}
