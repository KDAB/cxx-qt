// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "MyDataTests"

    Component {
        id: componentMyData

        MyData {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_deserialise() {
        // TODO: fix this test once we have the "Data" struct ready
        return;

        const myData = createTemporaryObject(componentMyData, null, {});
        compare(myData.number, 4);
        compare(myData.string, "Hello World!");
    }

    function test_serialize() {
        // TODO: fix this test once we have the "Data" struct ready
        return;

        const myData = createTemporaryObject(componentMyData, null, {});
        const spyNumber = createTemporaryObject(componentSpy, null, {
            signalName: "numberChanged",
            target: myData,
        });
        const spyString = createTemporaryObject(componentSpy, null, {
            signalName: "stringChanged",
            target: myData,
        });
        compare(spyNumber.count, 0);
        compare(spyString.count, 0);

        // Test initial values from serialization
        const data = myData.asJsonStr();
        compare(data, `{"number":4,"string":"Hello World!"}`);

        // Change some values
        myData.number = 2;
        myData.string = "Test!";

        compare(myData.number, 2);
        compare(myData.string, "Test!");
        compare(spyNumber.count, 1);
        compare(spyString.count, 1);

        // Test these new values appear in the serialization
        const newData = myData.asJsonStr();
        compare(newData, `{"number":2,"string":"Test!"}`);
    }
}
