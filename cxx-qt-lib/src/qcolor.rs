// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// We are only using references to QColor so it is actually ffi safe as far as we are concerned
#![allow(improper_ctypes)]

use crate::actually_private::Private;
use cxx::{type_id, ExternType};
use std::{
    marker::{PhantomData, PhantomPinned},
    mem::MaybeUninit,
    pin::Pin,
};

#[repr(u8)]
#[allow(dead_code)]
enum QColorSpec {
    Unsupported = 0,
    Rgb = 1,
}

extern "C" {
    #[link_name = "cxxqt1$qcolor$init$from$argb"]
    fn qcolor_init_from_argb(
        this: &mut MaybeUninit<QColor>,
        alpha: u16,
        red: u16,
        green: u16,
        blue: u16,
    );
    #[link_name = "cxxqt1$qcolor$get$alpha"]
    fn qcolor_get_alpha(this: &QColor) -> u16;
    #[link_name = "cxxqt1$qcolor$get$red"]
    fn qcolor_get_red(this: &QColor) -> u16;
    #[link_name = "cxxqt1$qcolor$get$green"]
    fn qcolor_get_green(this: &QColor) -> u16;
    #[link_name = "cxxqt1$qcolor$get$blue"]
    fn qcolor_get_blue(this: &QColor) -> u16;
    #[link_name = "cxxqt1$qcolor$get$spec"]
    fn qcolor_get_spec(this: &QColor) -> QColorSpec;
    #[link_name = "cxxqt1$qcolor$drop"]
    fn qcolor_drop(this: &mut MaybeUninit<QColor>);
}

/// Binding to Qt `QColor`.
///
/// # Invariants
///
/// As an invariant of this API and the static analysis of the cxx::bridge
/// macro, in Rust code we can never obtain a `QColor` by value. Qt's QColor
/// requires a move constructor and may hold internal pointers, which is not
/// compatible with Rust's move behavior. Instead in Rust code we will only ever
/// look at a QColor through a reference or smart pointer, as in `&QColor`
/// or `UniquePtr<QColor>`.
#[repr(C)]
pub struct QColor {
    _pinned: PhantomData<PhantomPinned>,
}

// TODO: figure out how to make Syntax and Example compile as code
// and then change ```ignore back to ```

/// Construct a QColor on the Rust stack.
///
/// # Syntax
///
/// In statement position:
///
/// ```ignore
/// # use cxx_qt_lib::{let_qcolor, Color};
/// # let expression = Color::ARGB { alpha: 0, red: 255, green: 0, blue: 0 });
/// let_qcolor!(var = expression);
/// ```
///
///
/// The `expression` must refer to a `&cxx_qt_lib::Color`.
///
/// The macro expands to something resembling `let $var: Pin<&mut QColor> =
/// /*???*/;`. The resulting [`Pin`] can be deref'd to `&QColor` as needed.
///
/// # Example
///
/// ```ignore
/// use cxx_qt_lib::{let_qcolor, QColor};
///
///
/// fn f(s: &QColor) {/* ... */}
///
/// fn main() {
///     let_qcolor!(s = "example");
///     f(&s);
/// }
/// ```
#[macro_export]
macro_rules! let_qcolor {
    ($var:ident = $value:expr $(,)?) => {
        let mut stack_qcolor = $crate::private::StackQColor::new();
        #[allow(unused_mut, unused_unsafe)]
        let mut $var = match $value {
            let_qcolor => unsafe { stack_qcolor.init(let_qcolor) },
        };
    };
}

impl QColor {
    /// `QColor` is not constructible via `new`.
    /// Instead, use the [`let_qcolor!`] macro.
    pub fn new<T: Private>() -> Self {
        unreachable!()
    }

    /// Create a new Rust Color from this QColor.
    /// This is a copy operation so any changes will not propagate to the original QColor.
    pub fn to_rust(&self) -> Option<Color> {
        // Given that a QColor can only be constructed using [`let_qcolor!`] macro,
        // it is safe to assume that self is a valid QColor reference which makes these
        // function calls safe.
        match unsafe { qcolor_get_spec(self) } {
            QColorSpec::Unsupported => None,
            QColorSpec::Rgb => Some(Color::ARGB {
                alpha: unsafe { qcolor_get_alpha(self) },
                red: unsafe { qcolor_get_red(self) },
                green: unsafe { qcolor_get_green(self) },
                blue: unsafe { qcolor_get_blue(self) },
            }),
        }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct StackQColor {
    // Static assertions in cxx_qt.cpp validate that this
    // is large enough and aligned enough.
    space: MaybeUninit<[usize; 2]>,
}

#[allow(missing_docs)]
impl StackQColor {
    pub fn new() -> Self {
        StackQColor {
            space: MaybeUninit::uninit(),
        }
    }

    /// # Safety
    ///
    /// Calling this function twice on the same StackQColor is unsafe
    /// and leads to undefined behaviour. It is therefore recommended
    /// to not use this function directly and instead use the [`let_qcolor!`]
    /// macro which ensures that safe behaviour.
    pub unsafe fn init(&mut self, value: &Color) -> Pin<&mut QColor> {
        let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<QColor>>();
        match value {
            Color::ARGB {
                alpha,
                red,
                green,
                blue,
            } => qcolor_init_from_argb(this, *alpha, *red, *green, *blue),
        }
        Pin::new_unchecked(&mut *this.as_mut_ptr())
    }
}

impl Default for StackQColor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StackQColor {
    fn drop(&mut self) {
        // # Safety
        //
        // This simply calls ~QColor on self.space which is safe as long
        // as self.space contains a valid QColor. Using the [`let_qcolor!`]
        // macro guarantees that this will be the case.
        unsafe {
            let this = &mut *self.space.as_mut_ptr().cast::<MaybeUninit<QColor>>();
            qcolor_drop(this);
        }
    }
}

// Safety:
//
// The code in this file ensures that QColor can only ever be allocated
// on the stack in pinned form which avoids the pitfalls of trying to
// move this type that has a non-trivial move constructor.
unsafe impl ExternType for QColor {
    type Id = type_id!("QColor");
    type Kind = cxx::kind::Opaque;
}

pub enum Color {
    ARGB {
        alpha: u16,
        red: u16,
        green: u16,
        blue: u16,
    },
}

impl From<&QColor> for Option<Color> {
    fn from(qcolor: &QColor) -> Self {
        qcolor.to_rust()
    }
}
