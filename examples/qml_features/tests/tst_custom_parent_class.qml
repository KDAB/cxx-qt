// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "CustomParentClassTests"

    Component {
        id: componentCustomParentClass

        CustomParentClass {

        }
    }

    function test_create() {
        const item = createTemporaryObject(componentCustomParentClass, null, {});
        verify(item !== null);
    }
}
