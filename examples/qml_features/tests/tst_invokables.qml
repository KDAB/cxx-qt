// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "InvokablesTests"

    readonly property color kdabColor: "#0077C8"
    readonly property color orangeColor: "#ff8000"

    Component {
        id: componentInvokables

        RustInvokables {

        }
    }

    function test_store_load() {
        const obj = createTemporaryObject(componentInvokables, null, {});
        compare(obj.loadColor(), kdabColor);

        obj.storeColor(1.0, 0.5, 0.0);
        compare(obj.loadColor(), orangeColor);

        obj.reset();
        compare(obj.loadColor(), kdabColor);
    }
}
