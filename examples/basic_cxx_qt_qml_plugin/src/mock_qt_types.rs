// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::make_qobject;

#[make_qobject]
mod mock_qt_types {
    use cxx_qt_lib::QPointF;

    pub struct Data {
        pointf: QPointF,
    }

    impl Default for Data {
        fn default() -> Self {
            Data {
                pointf: QPointF::new(1.0, 2.0),
            }
        }
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_pointf_property(&self, cpp: Pin<&mut CppObj>) {
            let mut wrapper = CppObjWrapper::new(cpp);
            let mut point = *wrapper.pointf();
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 2.0);
            wrapper.set_pointf(&point);
        }

        #[invokable]
        fn test_pointf_invokable(&self, point: &QPointF) -> QPointF {
            let mut point = *point;
            point.set_x(point.x() * 2.0);
            point.set_y(point.x() * 2.0);
            point
        }
    }
}
