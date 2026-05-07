use std::mem::MaybeUninit;

/// # Safety
///
/// `constructor` must initialize the passed pointer in-place (using `new (uninit) T(...)` in C++).
///
/// # Examples
///
/// ```ignore
/// using crate::util::new_in_place;
///
/// #[cxx::bridge]
/// mod ffi {
///     extern "C++" {
///         include!("cxx-qt-lib/qfont.h");
///         type QFont = super::QFont;
///     }
///
///     #[namespace = "rust::cxxqtlib1"]
///     unsafe extern "C++" {
///         include!("cxx-qt-lib/common.h");
///
///         #[rust_name = "qfont_clone"]
///         unsafe fn constructInPlace(uninit: *mut QFont, clone_from: &QFont);
///     }
/// }
///
/// impl Clone for ffi::QFont {
///     fn clone(&self) -> Self {
///         unsafe { new_in_place(|uninit| ffi::qfont_clone(uninit, self)) }
///     }
/// }
/// ```
#[allow(unused)]
pub(crate) unsafe fn new_in_place<T, F>(constructor: F) -> T
where
    F: FnOnce(*mut T),
{
    let mut uninit = MaybeUninit::uninit();
    constructor(uninit.as_mut_ptr());
    // SAFETY: `constructor` initializes the passed pointer in-place.
    unsafe { uninit.assume_init() }
}
