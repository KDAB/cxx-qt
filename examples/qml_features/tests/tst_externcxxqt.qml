// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0
import com.kdab.cxx_qt.demo_cpp 1.0

TestCase {
    name: "ExternCxxQtTests"

    Component {
        id: componentExternalQObject

        ExternalQObject {

        }
    }

    Component {
        id: componentExternalCxxQtHelper

        ExternalCxxQtHelper {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_connect_to_external() {
        const obj = createTemporaryObject(componentExternalQObject, null, {});
        const helper = createTemporaryObject(componentExternalCxxQtHelper, null, {});
        const triggeredSpy = createTemporaryObject(componentSpy, null, {
            signalName: "triggered",
            target: obj,
        });
        const triggeredPrivateSpy = createTemporaryObject(componentSpy, null, {
            signalName: "triggeredPrivateSignal",
            target: obj,
        });

        helper.connectToExternal(obj);

        compare(triggeredSpy.count, 0);
        compare(triggeredPrivateSpy.count, 0);
        compare(helper.count, 0);
        compare(helper.privateCount, 0);

        helper.triggerOnExternal(obj, 2);

        tryCompare(triggeredSpy, "count", 2);
        tryCompare(triggeredPrivateSpy, "count", 2);
        tryCompare(helper, "count", 2);
        tryCompare(helper, "privateCount", 2);
    }
}
