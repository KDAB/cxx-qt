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
    function test_pointf_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "pointfChanged",
            target: mock,
        });
        compare(mock.pointf, Qt.point(1, 2));

        compare(spy.count, 0);
        mock.pointf = Qt.point(10, 20);
        tryCompare(spy, "count", 1);
        compare(mock.pointf, Qt.point(10, 20));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_pointf_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testPointfInvokable(Qt.point(10, 20));
        compare(result, Qt.point(20, 40));
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_pointf_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "pointfChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.pointf, Qt.point(1, 2));
        mock.testPointfProperty();
        compare(mock.pointf, Qt.point(2, 4));
        tryCompare(spy, "count", 1);
    }

    // QVariant

    // Check that we can adjust the property for the type and it has non default value
    function test_qvariant_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "variantChanged",
            target: mock,
        });
        compare(mock.variant, 1);

        compare(spy.count, 0);
        mock.variant = "string";
        tryCompare(spy, "count", 1);
        compare(mock.variant, "string");
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qvariant_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        let result = mock.testVariantInvokable(10);
        compare(result, 20);

        result = mock.testVariantInvokable(true);
        compare(result, false);
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qvariant_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "variantChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.variant, 1);
        mock.testVariantProperty();
        compare(mock.variant, 2);
        tryCompare(spy, "count", 1);

        mock.variant = true;
        // wait for signal to occur
        tryCompare(spy, "count", 2);

        spy.clear();

        compare(spy.count, 0);
        compare(mock.variant, true);
        mock.testVariantProperty();
        compare(mock.variant, false);
        tryCompare(spy, "count", 1);
    }
}
