// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::let_qstring;
use crate::qstring::QString;

pub trait MapQtValue<Out> {
    fn map_qt_value<C>(&self, map_func: fn(C, &Out), context: C);
}

impl MapQtValue<QString> for &str {
    fn map_qt_value<C>(&self, map_func: fn(C, &QString), context: C) {
        let_qstring!(s = self);
        map_func(context, &s)
    }
}

impl MapQtValue<i32> for i32 {
    fn map_qt_value<C>(&self, map_func: fn(C, &i32), context: C) {
        map_func(context, self)
    }
}
