// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "SerialisationTests"

    Component {
        id: componentSerialisation

        Serialisation {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_deserialise() {
        const serialisation = createTemporaryObject(componentSerialisation, null, {});
        compare(serialisation.number, 4);
        compare(serialisation.string, "Hello World!");
    }

    function test_serialize() {
        const serialisation = createTemporaryObject(componentSerialisation, null, {});
        const spyNumber = createTemporaryObject(componentSpy, null, {
            signalName: "numberChanged",
            target: serialisation,
        });
        const spyString = createTemporaryObject(componentSpy, null, {
            signalName: "stringChanged",
            target: serialisation,
        });
        compare(spyNumber.count, 0);
        compare(spyString.count, 0);

        // Test initial values from serialization
        const data = serialisation.asJsonStr();
        compare(data, `{"number":4,"string":"Hello World!"}`);

        // Change some values
        serialisation.number = 2;
        serialisation.string = "Test!";

        compare(serialisation.number, 2);
        compare(serialisation.string, "Test!");
        compare(spyNumber.count, 1);
        compare(spyString.count, 1);

        // Test these new values appear in the serialization
        const newData = serialisation.asJsonStr();
        compare(newData, `{"number":2,"string":"Test!"}`);
    }

    function test_grab_valuess() {
        const serialisation = createTemporaryObject(componentSerialisation, null, {});
        compare(serialisation.number, 4);
        compare(serialisation.string, "Hello World!");

        serialisation.grabValues();
        compare(serialisation.number, 2);
        compare(serialisation.string, "Goodbye!");
    }
}
