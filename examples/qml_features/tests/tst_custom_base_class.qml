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

    Component {
        id: componentSpy

        SignalSpy {

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

    function test_multiply() {
        const model = createTemporaryObject(componentCustomBaseClass, null, {});
        const instantiator = createTemporaryObject(componentInstantiator, null, {
            model: model,
        });
        const dataChangedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "dataChanged",
            target: model,
        });
        instantiator.model = model;
        model.add();
        compare(instantiator.count, 1);
        compare(dataChangedSpy.count, 0);

        model.multiply(0, 2.0);
        compare(instantiator.count, 1);
        compare(dataChangedSpy.count, 1);
    }

    function test_roles_qenum() {
        compare(CustomBaseClass.Id, 0);
        compare(CustomBaseClass.Value, 1);

        const model = createTemporaryObject(componentCustomBaseClass, null, {});

        model.add();
        compare(model.data(model.index(0, 0), CustomBaseClass.Id), 0);
        compare(model.data(model.index(0, 0), CustomBaseClass.Value), 0);

        model.add();
        compare(model.data(model.index(1, 0), CustomBaseClass.Id), 1);
        compare(model.data(model.index(1, 0), CustomBaseClass.Value), 1.0 / 3.0);

        compare(model.state, CustomBaseClass.Idle);
        model.addOnThreadDelayed(1, 0 /*ms*/);
        // This shouldn't cause a flaky test, as the `addOnThread` cannot reset the
        // state to `Idle` until the Event-Loop of this thread is available again.
        // Which should only be after this thread has finished.
        compare(model.state, CustomBaseClass.Running);
        // Return to the event loop now to give the background thread a chance to reset the "state" again.
        tryCompare(model, "state", CustomBaseClass.Idle);
    }
}
