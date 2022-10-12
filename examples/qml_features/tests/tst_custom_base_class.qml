// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtQml.Models 2.15
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "CustomBaseClassTests"

    Component {
        id: componentCustomBaseClass

        CustomBaseClass {

        }
    }

    Component {
        id: componentInstantiator

        Instantiator {
            delegate: QtObject {}
        }
    }

    function test_add_remove() {
        const model = createTemporaryObject(componentCustomBaseClass, null, {});
        const instantiator = createTemporaryObject(componentInstantiator, null, {
            model: model,
        });
        instantiator.model = model;
        compare(instantiator.count, 0);
        model.add();
        compare(instantiator.count, 1);
        model.add();
        compare(instantiator.count, 2);
        model.remove(1);
        compare(instantiator.count, 1);
        model.remove(0);
        compare(instantiator.count, 0);
    }

    function test_add_on_thread_clear() {
        const model = createTemporaryObject(componentCustomBaseClass, null, {});
        const instantiator = createTemporaryObject(componentInstantiator, null, {
            model: model,
        });
        compare(instantiator.count, 0);
        model.addOnThread(5);
        tryCompare(instantiator, "count", 5);
        model.clear();
        compare(instantiator.count, 0);
    }
}
