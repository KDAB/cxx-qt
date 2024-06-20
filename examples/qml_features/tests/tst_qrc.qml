// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
import QtQuick 2.12
import QtTest 1.12

TestCase {
    name: "QrcTests"

    Component {
        id: componentImage

        Image {

        }
    }

    function test_image(data) {
        const image = createTemporaryObject(componentImage, null, {
            "source": data.source,
        });

        compare(image.status, data.status);
    }

    function test_image_data() {
        return [
            {
                tag: "valid", source: "qrc:/images/red.png", status: Image.Ready,
                tag: "invalid", source: "qrc:/images/invalid.png", status: Image.Error,
            }
        ]
    }
}
