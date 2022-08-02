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
        id: componentQtObjectUrl

        QtObject {
            property url value
        }
    }

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

    // Signals

    function test_signal() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const readySpy = createTemporaryObject(componentSpy, null, {
            signalName: "ready",
            target: mock,
        });
        const dataChangedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "dataChanged",
            target: mock,
        });

        compare(readySpy.count, 0);
        compare(dataChangedSpy.count, 0);

        mock.testSignal();

        // Safe signal emission is queued
        compare(readySpy.count, 0);
        compare(dataChangedSpy.count, 0);
        tryCompare(readySpy, "count", 1);
        compare(readySpy.signalArguments[0].length, 0);
        tryCompare(dataChangedSpy, "count", 1);
        compare(dataChangedSpy.signalArguments[0].length, 1);
        const signalArguments = dataChangedSpy.signalArguments[0];
        compare(signalArguments[0], true);
    }

    function test_unsafe_signal() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const readySpy = createTemporaryObject(componentSpy, null, {
            signalName: "ready",
            target: mock,
        });
        const dataChangedSpy = createTemporaryObject(componentSpy, null, {
            signalName: "dataChanged",
            target: mock,
        });

        compare(readySpy.count, 0);
        compare(dataChangedSpy.count, 0);

        mock.testUnsafeSignal();

        compare(readySpy.count, 1);
        compare(readySpy.signalArguments[0].length, 0);
        compare(dataChangedSpy.count, 1);
        compare(dataChangedSpy.signalArguments[0].length, 1);
        const signalArguments = dataChangedSpy.signalArguments[0];
        compare(signalArguments[0], true);
    }


    // QColor

    // Check that we can adjust the property for the type and it has non default value
    function test_color_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "colorChanged",
            target: mock,
        });
        compare(mock.color, Qt.rgba(1.0, 0.0, 0.0, 1.0));

        compare(spy.count, 0);
        mock.color = Qt.rgba(1.0, 1.0, 1.0, 1.0);
        tryCompare(spy, "count", 1);
        compare(mock.color, Qt.rgba(1.0, 1.0, 1.0, 1.0));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_color_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testColorInvokable(Qt.rgba(1.0, 0.0, 0.0, 1.0));
        compare(result, Qt.rgba(0.0, 1.0, 0, 1.0));
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_color_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "colorChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.color, Qt.rgba(1.0, 0.0, 0.0, 1.0));
        mock.testColorProperty();
        compare(mock.color, Qt.rgba(0.0, 0.0, 1.0, 1.0));
        tryCompare(spy, "count", 1);
    }



    // QDate

    // Check that we can adjust the property for the type and it has non default value
    function test_qdate_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "dateChanged",
            target: mock,
        });
        compare(mock.date.getFullYear(), 2022);
        compare(mock.date.getMonth(), 0);
        compare(mock.date.getDate(), 1);

        compare(spy.count, 0);
        mock.date = new Date(2021, 11, 31);
        tryCompare(spy, "count", 1);
        compare(mock.date.getFullYear(), 2021);
        compare(mock.date.getMonth(), 11);
        compare(mock.date.getDate(), 31);
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qdate_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testDateInvokable(new Date(2022, 0, 1));
        compare(result.getFullYear(), 2021);
        compare(result.getMonth(), 11);
        compare(result.getDate(), 31);
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qdate_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "dateChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.date.getFullYear(), 2022);
        compare(mock.date.getMonth(), 0);
        compare(mock.date.getDate(), 1);
        mock.testDateProperty();
        compare(mock.date.getFullYear(), 2021);
        compare(mock.date.getMonth(), 11);
        compare(mock.date.getDate(), 31);
        tryCompare(spy, "count", 1);
    }


    // QDateTime

    // Check that we can adjust the property for the type and it has non default value
    function test_qdatetime_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "dateTimeChanged",
            target: mock,
        });
        compare(mock.dateTime, new Date(2022, 0, 1, 1, 2, 3, 4));

        compare(spy.count, 0);
        mock.dateTime = new Date(2021, 11, 31, 4, 3, 2, 1);
        tryCompare(spy, "count", 1);
        compare(mock.dateTime, new Date(2021, 11, 31, 4, 3, 2, 1));
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qdatetime_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const result = mock.testDateTimeInvokable(new Date(2022, 0, 1, 1, 2, 3, 4));
        compare(result, new Date(2021, 11, 31, 2, 6, 12, 20));
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qdatetime_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "dateTimeChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.dateTime, new Date(2022, 0, 1, 1, 2, 3, 4));
        mock.testDateTimeProperty();
        compare(mock.dateTime, new Date(2021, 11, 31, 2, 6, 12, 20));
        tryCompare(spy, "count", 1);
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


    // QTime

    // Check that we can adjust the property for the type and it has non default value
    function test_qtime_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "timeChanged",
            target: mock,
        });
        compare(mock.time.getHours(), 1);
        compare(mock.time.getMinutes(), 2);
        compare(mock.time.getSeconds(), 3);
        compare(mock.time.getMilliseconds(), 4);

        compare(spy.count, 0);
        mock.time = new Date(0, 0, 0, 4, 3, 2, 1);
        tryCompare(spy, "count", 1);
        compare(mock.time.getHours(), 4);
        compare(mock.time.getMinutes(), 3);
        compare(mock.time.getSeconds(), 2);
        compare(mock.time.getMilliseconds(), 1);
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qtime_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});

        const result = mock.testTimeInvokable(new Date(0, 0, 0, 1, 2, 3, 4));
        compare(result.getHours(), 2);
        compare(result.getMinutes(), 6);
        compare(result.getSeconds(), 12);
        compare(result.getMilliseconds(), 20);
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qtime_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "timeChanged",target: mock,
        });

        compare(spy.count, 0);
        compare(mock.time.getHours(), 1);
        compare(mock.time.getMinutes(), 2);
        compare(mock.time.getSeconds(), 3);
        compare(mock.time.getMilliseconds(), 4);
        mock.testTimeProperty();
        compare(mock.time.getHours(), 2);
        compare(mock.time.getMinutes(), 6);
        compare(mock.time.getSeconds(), 12);
        compare(mock.time.getMilliseconds(), 20);
        tryCompare(spy, "count", 1);
    }



    // QUrl

    // Check that we can adjust the property for the type and it has non default value
    function test_qurl_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "urlChanged",
            target: mock,
        });
        compare(mock.url, "https://github.com/KDAB");

        compare(spy.count, 0);
        mock.url = "https://github.com/KDAB/cxx-qt";
        tryCompare(spy, "count", 1);
        compare(mock.url, "https://github.com/KDAB/cxx-qt");
    }

    // Check that we can pass the type as a parameter and return it back
    function test_qurl_invokable() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const result = mock.testUrlInvokable("https://github.com/KDAB");
        compare(result, "https://github.com/KDAB/cxx-qt");
    }

    // Check that an invokable can adjust (read and write) a property for the type
    function test_qurl_invokable_property() {
        const mock = createTemporaryObject(componentMockQtTypes, null, {});
        const spy = createTemporaryObject(componentSpy, null, {
            signalName: "urlChanged",
            target: mock,
        });

        compare(spy.count, 0);
        compare(mock.url, "https://github.com/KDAB");
        mock.testUrlProperty();
        compare(mock.url, "https://github.com/KDAB/cxx-qt");
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

        result = mock.testVariantInvokable(Qt.rgba(1.0, 1.0, 1.0, 1.0));
        compare(result, Qt.rgba(0.0, 1.0, 0.0, 1.0));

        result = mock.testVariantInvokable(new Date(2022, 0, 1, 1, 2, 3, 4));
        compare(result, new Date(2021, 11, 31, 2, 6, 12, 20));

        result = mock.testVariantInvokable(Qt.point(1.0, 3.0));
        compare(result, Qt.point(2.0, 6.0));

        result = mock.testVariantInvokable(Qt.rect(10, 20, 30, 40));
        compare(result, Qt.rect(20, 60, 120, 200));

        result = mock.testVariantInvokable(Qt.size(1.0, 3.0));
        compare(result, Qt.size(2.0, 6.0));

        result = mock.testVariantInvokable("KDAB");
        compare(result, "KDAB/cxx-qt");

        const urlComponent = createTemporaryObject(componentQtObjectUrl, null, {
            value: "https://github.com/KDAB",
        });
        result = mock.testVariantInvokable(urlComponent.value);
        compare(result, "https://github.com/KDAB/cxx-qt");
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
