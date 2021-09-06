// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::let_qstring;
use crate::qstring::QString;

pub trait MapQtValue<C, F, R> {
    fn map_qt_value(&self, map_func: F, context: &mut C) -> R;
}

impl<C, R> MapQtValue<C, fn(&mut C, &QString) -> R, R> for &str {
    fn map_qt_value(&self, map_func: fn(&mut C, &QString) -> R, context: &mut C) -> R {
        let_qstring!(s = self);
        map_func(context, &s)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, &QString) -> R, R> for String {
    fn map_qt_value(&self, map_func: fn(&mut C, &QString) -> R, context: &mut C) -> R {
        let_qstring!(s = self);
        map_func(context, &s)
    }
}

impl<C, R> MapQtValue<C, fn(&mut C, i32) -> R, R> for i32 {
    fn map_qt_value(&self, map_func: fn(&mut C, i32) -> R, context: &mut C) -> R {
        map_func(context, *self)
    }
}
