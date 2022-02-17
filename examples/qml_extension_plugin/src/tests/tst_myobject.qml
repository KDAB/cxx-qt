// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "MyObjectTests"

    Component {
        id: componentMyObject

        MyObject {

        }
    }

    function test_default() {
        const myObject = createTemporaryObject(componentMyObject, null, {});
        compare(myObject.number, 1);
        compare(myObject.string, "Hello World!");
    }

    function test_increment() {
        const myObject = createTemporaryObject(componentMyObject, null, {
            number: 5,
        });
        myObject.increment();
        compare(myObject.number, 6);
    }

    function test_reset() {
        const myObject = createTemporaryObject(componentMyObject, null, {
            number: 5,
            string: "KDAB",
        });
        compare(myObject.number, 1);
        compare(myObject.string, "Hello World!");
    }

    function test_serialize() {
        const myObject = createTemporaryObject(componentMyObject, null, {
            number: 5,
            string: "KDAB",
        });
        compare(myObject.serialize(), `{"number":5,"string":"KDAB"}`);
    }
}
