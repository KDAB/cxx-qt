// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "MockQtTypesTests"

    Component {
        id: componentMockQtTypes

        MockQtTypes {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    // QPointF

    // Check that we can adjust the property for the type and it has non default value
    function test_qpointf_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "pointfChanged",
            target: mock,
        });
        compare(mock.pointf, Qt.point(1, 2));

        compare(spy.count, 0);
        mock.pointf = Qt.point(10, 20);
        compare(spy.count, 1);
        compare(mock.pointf, Qt.point(10, 20));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qpointf_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testPointfInvokable(Qt.point(10, 20));
        compare(result, Qt.point(20, 40));
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qpointf_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "pointfChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.pointf, Qt.point(1, 2));
        mock.testPointfProperty();
        compare(mock.pointf, Qt.point(2, 4));
        compare(spy.count, 1);
    }
}
