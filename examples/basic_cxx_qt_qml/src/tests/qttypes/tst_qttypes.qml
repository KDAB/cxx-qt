// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

import com.kdab.cxx_qt.demo 1.0

TestCase {
    name: "MockQtTypesTests"

    Component {
        id: componentMockQtTypes

        MockQtTypes {

        }
    }

    Component {
        id: componentSpy

        SignalSpy {

        }
    }

    // QPoint

    // Check that we can adjust the property for the type and it has non default value
    function test_qpoint_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "pointChanged",
            target: mock,
        });
        compare(mock.point, Qt.point(1, 3));

        compare(spy.count, 0);
        mock.point = Qt.point(10, 30);
        tryCompare(spy, "count", 1);
        compare(mock.point, Qt.point(10, 30));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qpoint_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testPointInvokable(Qt.point(10, 30));
        compare(result, Qt.point(20, 90));
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qpoint_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "pointChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.point, Qt.point(1, 3));
        mock.testPointProperty();
        compare(mock.point, Qt.point(2, 9));
        tryCompare(spy, "count", 1);
    }


    // QPointF

    // Check that we can adjust the property for the type and it has non default value
    function test_qpointf_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "pointfChanged",
            target: mock,
        });
        compare(mock.pointf, Qt.point(1, 3));

        compare(spy.count, 0);
        mock.pointf = Qt.point(10, 30);
        tryCompare(spy, "count", 1);
        compare(mock.pointf, Qt.point(10, 30));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qpointf_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testPointfInvokable(Qt.point(10, 30));
        compare(result, Qt.point(20, 90));
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qpointf_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "pointfChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.pointf, Qt.point(1, 3));
        mock.testPointfProperty();
        compare(mock.pointf, Qt.point(2, 9));
        tryCompare(spy, "count", 1);
    }


    // QRect

    // Check that we can adjust the property for the type and it has non default value
    function test_qrect_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "rectChanged",
            target: mock,
        });
        compare(mock.rect, Qt.rect(1, 2, 3, 4));

        compare(spy.count, 0);
        mock.rect = Qt.rect(10, 20, 30, 40);
        tryCompare(spy, "count", 1);
        compare(mock.rect, Qt.rect(10, 20, 30, 40));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qrect_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testRectInvokable(Qt.rect(10, 20, 30, 40));
        compare(result, Qt.rect(20, 60, 120, 200));
    }

    function test_qrect_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "rectChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.rect, Qt.rect(1, 2, 3, 4));
        mock.testRectProperty();
        compare(mock.rect, Qt.rect(2, 6, 12, 20));
        tryCompare(spy, "count", 1);
    }


    // QRectF

    // Check that we can adjust the property for the type and it has non default value
    function test_qrectf_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "rectfChanged",
            target: mock,
        });
        compare(mock.rectf, Qt.rect(1, 2, 3, 4));

        compare(spy.count, 0);
        mock.rectf = Qt.rect(10, 20, 30, 40);
        tryCompare(spy, "count", 1);
        compare(mock.rectf, Qt.rect(10, 20, 30, 40));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qrectf_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testRectfInvokable(Qt.rect(10, 20, 30, 40));
        compare(result, Qt.rect(20, 60, 120, 200));
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qrectf_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "rectfChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.rectf, Qt.rect(1, 2, 3, 4));
        mock.testRectfProperty();
        compare(mock.rectf, Qt.rect(2, 6, 12, 20));
        tryCompare(spy, "count", 1);
    }


    // QSize

    // Check that we can adjust the property for the type and it has non default value
    function test_qsize_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "sizeChanged",
            target: mock,
        });
        compare(mock.size, Qt.size(1, 3));

        compare(spy.count, 0);
        mock.size = Qt.size(10, 30);
        tryCompare(spy, "count", 1);
        compare(mock.size, Qt.size(10, 30));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qsize_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testSizeInvokable(Qt.size(10, 30));
        compare(result, Qt.size(20, 90));
    }

    function test_qsize_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "sizeChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.size, Qt.size(1, 3));
        mock.testSizeProperty();
        compare(mock.size, Qt.size(2, 9));
        tryCompare(spy, "count", 1);
    }


    // QSizeF

    // Check that we can adjust the property for the type and it has non default value
    function test_qsizef_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "sizefChanged",
            target: mock,
        });
        compare(mock.sizef, Qt.size(1, 3));

        compare(spy.count, 0);
        mock.sizef = Qt.size(10, 30);
        tryCompare(spy, "count", 1);
        compare(mock.sizef, Qt.size(10, 30));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qsizef_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testSizefInvokable(Qt.size(10, 30));
        compare(result, Qt.size(20, 90));
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qsizef_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "sizefChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.sizef, Qt.size(1, 3));
        mock.testSizefProperty();
        compare(mock.sizef, Qt.size(2, 9));
        tryCompare(spy, "count", 1);
    }


    // QVariant

    // Check that we can adjust the property for the type and it has non default value
    function test_qvariant_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "variantChanged",
            target: mock,
        });
        compare(mock.variant, 1);

        compare(spy.count, 0);
        mock.variant = "string";
        tryCompare(spy, "count", 1);
        compare(mock.variant, "string");
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qvariant_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        let result = mock.testVariantInvokable(10);
        compare(result, 20);

        result = mock.testVariantInvokable(true);
        compare(result, false);
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qvariant_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "variantChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.variant, 1);
        mock.testVariantProperty();
        compare(mock.variant, 2);
        tryCompare(spy, "count", 1);

        mock.variant = true;
        // wait for signal to occur
        tryCompare(spy, "count", 2);

        spy.clear();

        compare(spy.count, 0);
        compare(mock.variant, true);
        mock.testVariantProperty();
        compare(mock.variant, false);
        tryCompare(spy, "count", 1);
    }
}
