// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod update_requester;
pub use update_requester::UpdateRequestHandler;

mod qstring;
pub use qstring::QString;

mod map_qt_value;
pub use map_qt_value::*;

pub trait PropertyChangeHandler<C, P> {
    fn handle_property_change(&mut self, cpp: std::pin::Pin<&mut C>, property: P);
}
/// This mod contains private things that need to technically be pub so that
/// they can be used inside macros. You should not use anything inside here
/// from another crate even though that would compile.
pub mod private {
    pub use crate::qstring::StackQString;
}

mod actually_private {
    /// This is a dummy struct. If you add it as a template parameter to new()
    /// inside a struct that you have been forced to make public then outside
    /// users won't be able to construct such a struct themselves.
    pub trait Private {}
}
