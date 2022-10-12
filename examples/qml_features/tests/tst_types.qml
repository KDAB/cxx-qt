// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "TypesTests"

    readonly property url kdabUrl: "https://kdab.com"
    readonly property url kdabAboutUrl: "https://kdab.com/about"

    Component {
        id: componentTypes

        Types {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    function test_default() {
        const obj = createTemporaryObject(componentTypes, null, {});
        compare(obj.boolean, false);
        compare(obj.point, Qt.point(1.0, 2.0));
        compare(obj.url, kdabUrl);
    }

    function test_load_from_variant() {
        const obj = createTemporaryObject(componentTypes, null, {});
        const booleanSpy = createTemporaryObject(componentSpy, null, {
            signalName: "booleanChanged",
            target: obj,
        });
        const pointSpy = createTemporaryObject(componentSpy, null, {
            signalName: "pointChanged",
            target: obj,
        });
        const urlSpy = createTemporaryObject(componentSpy, null, {
            signalName: "urlChanged",
            target: obj,
        });

        compare(obj.boolean, false);
        obj.loadFromVariant(true);
        compare(obj.boolean, true);
        compare(booleanSpy.count, 1);
        compare(pointSpy.count, 0);
        compare(urlSpy.count, 0);

        compare(obj.point, Qt.point(1.0, 2.0));
        obj.loadFromVariant(Qt.point(2.0, 4.0));
        compare(obj.point, Qt.point(2.0, 4.0));
        compare(booleanSpy.count, 1);
        compare(pointSpy.count, 1);
        compare(urlSpy.count, 0);

        compare(obj.url, kdabUrl);
        obj.loadFromVariant(kdabAboutUrl);
        compare(obj.url, kdabAboutUrl);
        compare(booleanSpy.count, 1);
        compare(pointSpy.count, 1);
        compare(urlSpy.count, 1);
    }

    function test_toggle_boolean() {
        const obj = createTemporaryObject(componentTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "booleanChanged",
            target: obj,
        });
        compare(obj.boolean, false);
        compare(spy.count, 0);
        obj.toggleBoolean();
        compare(obj.boolean, true);
        compare(spy.count, 1);
    }
}
