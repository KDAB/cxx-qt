// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "NamingTests"

    Component {
        id: componentNaming

        RenamedObject {

        }
    }

    function test_increment() {
        const obj = createTemporaryObject(componentNaming, null, {});

        compare(obj.num, 0)
        obj.increment();
        compare(obj.num, 1)
    }
}
