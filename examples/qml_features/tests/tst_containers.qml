// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "ContainerTests"

    Component {
        id: componentContainers

        RustContainers {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_container_hash() {
        const obj = createTemporaryObject(componentContainers, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "stringHashChanged",
            target: obj,
        });
        compare(spy.count, 0);
        compare(obj.stringHash, "");

        obj.insertHash("A1", 1);
        obj.insertHash("A1", 1);
        obj.insertHash("A3", 3);
        obj.insertHash("A3", 3);

        compare(spy.count, 2);
        // Order of Hash is not consistent
        verify(obj.stringHash === "A1 => 1, A3 => 3" || obj.stringHash === "A3 => 3, A1 => 1");

        obj.reset();
        compare(spy.count, 3);
        compare(obj.stringHash, "");
    }

    function test_container_list() {
        const obj = createTemporaryObject(componentContainers, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "stringListChanged",
            target: obj,
        });
        compare(spy.count, 0);
        compare(obj.stringList, "");

        obj.appendList(1);
        obj.appendList(1);
        obj.appendList(3);
        obj.appendList(3);

        compare(spy.count, 4);
        compare(obj.stringList, "1, 1, 3, 3");

        obj.reset();
        compare(spy.count, 5);
        compare(obj.stringList, "");
    }

    function test_container_map() {
        const obj = createTemporaryObject(componentContainers, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "stringMapChanged",
            target: obj,
        });
        compare(spy.count, 0);
        compare(obj.stringMap, "");

        obj.insertMap("A1", 1);
        obj.insertMap("A1", 1);
        obj.insertMap("A3", 3);
        obj.insertMap("A3", 3);

        compare(spy.count, 2);
        compare(obj.stringMap, "A1 => 1, A3 => 3");

        obj.reset();
        compare(spy.count, 3);
        compare(obj.stringMap, "");
    }

    function test_container_set() {
        const obj = createTemporaryObject(componentContainers, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "stringSetChanged",
            target: obj,
        });
        compare(spy.count, 0);
        compare(obj.stringSet, "");

        obj.insertSet(1);
        obj.insertSet(1);
        obj.insertSet(3);
        obj.insertSet(3);

        compare(spy.count, 2);
        // Order of Set is not consistent
        verify(obj.stringSet === "1, 3" || obj.stringSet === "3, 1");

        obj.reset();
        compare(spy.count, 3);
        compare(obj.stringSet, "");
    }

    function test_container_vector() {
        const obj = createTemporaryObject(componentContainers, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "stringVectorChanged",
            target: obj,
        });
        compare(spy.count, 0);
        compare(obj.stringVector, "");

        obj.appendVector(1);
        obj.appendVector(1);
        obj.appendVector(3);
        obj.appendVector(3);

        compare(spy.count, 4);
        compare(obj.stringVector, "1, 1, 3, 3");

        obj.reset();
        compare(spy.count, 5);
        compare(obj.stringVector, "");
    }
}
