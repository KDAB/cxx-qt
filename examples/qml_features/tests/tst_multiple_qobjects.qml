// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "MultipleQObjectsTests"

    Component {
        id: componentFirstObject

        FirstObject {

        }
    }

    Component {
        id: componentSecondObject

        SecondObject {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_increment() {
        const first = createTemporaryObject(componentFirstObject, null, {});
        const second = createTemporaryObject(componentSecondObject, null, {});
        const firstAcceptedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "accepted",
            target: first,
        });
        const firstRejectedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "rejected",
            target: first,
        });
        const secondAcceptedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "accepted",
            target: second,
        });
        const secondRejectedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "rejected",
            target: second,
        });
        compare(first.counter, 10);
        compare(first.color, Qt.rgba(0, 0, 1, 1));
        compare(second.counter, 100);
        compare(second.url, "https://github.com/kdab/cxx-qt");
        compare(firstAcceptedSpy.count, 0);
        compare(firstRejectedSpy.count, 0);
        compare(secondAcceptedSpy.count, 0);
        compare(secondRejectedSpy.count, 0);

        first.increment();
        compare(first.counter, 11);
        compare(first.color, Qt.rgba(1, 0, 0, 1));
        compare(second.counter, 100);
        compare(firstAcceptedSpy.count, 0);
        compare(firstRejectedSpy.count, 1);
        compare(secondAcceptedSpy.count, 0);
        compare(secondRejectedSpy.count, 0);

        second.increment();
        compare(first.counter, 11);
        compare(second.counter, 101);
        compare(second.url, "https://kdab.com");
        compare(firstAcceptedSpy.count, 0);
        compare(firstRejectedSpy.count, 1);
        compare(secondAcceptedSpy.count, 0);
        compare(secondRejectedSpy.count, 1);
    }
}
