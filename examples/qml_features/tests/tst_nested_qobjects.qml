// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "NestedQObjectsTests"

    Component {
        id: componentInnerObject
        InnerObject {

        }
    }

    Component {
        id: componentOuterObject
        OuterObject { }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_nested() {
        const inner = createTemporaryObject(componentInnerObject, null, {});
        const outer = createTemporaryObject(componentOuterObject, null, {
            inner: inner,
        });
        const calledSpy = createTemporaryObject(componentSpy, null, {
            signalName: "called",
            target: outer,
        });
        const calledInnerSpy = createTemporaryObject(componentSpy, null, {
            signalName: "called",
            target: inner,
        });

        compare(inner.counter, 0);
        compare(calledSpy.count, 0);
        compare(calledInnerSpy.count, 0);
        outer.reset();
        compare(inner.counter, 10);
        compare(calledSpy.count, 1);
        compare(calledInnerSpy.count, 1);
        compare(calledSpy.signalArguments[0].length, 1);
        compare(calledSpy.signalArguments[0][0], inner);

        const inner2 = createTemporaryObject(componentInnerObject, null, {
            counter: 20,
        });
        compare(inner2.counter, 20);
        outer.printCount(inner2);
        compare(calledSpy.count, 2);
        compare(calledInnerSpy.count, 2);
        compare(calledSpy.signalArguments[1].length, 1);
        compare(calledSpy.signalArguments[1][0], inner2);
        compare(calledSpy.signalArguments[1][0].counter, 20);
    }
}
