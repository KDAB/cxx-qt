// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

TestCase {
    id: root
    name: "UncreatableTests"

    function test_uncreatable() {
        try {
            const obj = Qt.createQmlObject(`
                import com.kdab.cxx_qt.demo 1.0

                RustUncreatable {

                }
                `,
                root,
                "uncreatableObject"
            );
            fail("Uncreatable type should not be creatable");
        } catch (error) {
            verify(error.qmlErrors !== undefined);
            compare(error.qmlErrors.length, 1);
            // Check for Qt 5 or Qt 6 error message
            verify(error.qmlErrors[0].message === "Element is not creatable." || error.qmlErrors[0].message === "Type cannot be created in QML.");
        }
    }
}
