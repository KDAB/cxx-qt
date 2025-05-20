// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::ops::Deref;
use std::pin::Pin;

/// This trait is automatically implemented by CXX-Qt and you most likely do not need to manually implement it.
/// Allows upcasting to either [crate::QObject] or the provided base class of a type.
/// Will not be implemented if no types inherit from [crate::QObject] or have the `#[base = T]` attribute.
///
/// # Safety
///
/// By implementing Upcast for your type, you take responsibility that the type you are upcasting to is actually a parent.
pub unsafe trait Upcast<T> {
    #[doc(hidden)]
    /// # Safety
    ///
    /// Internal function, Should probably not be implemented manually unless you're absolutely sure you need it.
    /// Automatically available for types in RustQt blocks in [cxx_qt::bridge](bridge)s.
    /// Upcasts a pointer to `Self` to a pointer to the base class `T`.
    /// > Note: Internal implementation uses `static_cast`.
    unsafe fn upcast_ptr(this: *const Self) -> *const T;

    #[doc(hidden)]
    /// # Safety
    ///
    /// Internal function, Should probably not be implemented manually unless you're absolutely sure you need it.
    /// Automatically available for types in RustQt blocks in [cxx_qt::bridge](bridge)s.
    /// Downcasts a pointer to base class `T` to a pointer to `Self`.
    /// Return a null pointer if `Self` is not actually a child of base.
    /// > Note: Internal implementation uses `dynamic_cast`.
    unsafe fn from_base_ptr(base: *const T) -> *const Self;

    /// Upcast a reference to self to a reference to the base class
    fn upcast(&self) -> &T {
        let ptr = self as *const Self;
        unsafe {
            let base = Self::upcast_ptr(ptr);
            &*base
        }
    }

    /// Upcast a mutable reference to sell to a mutable reference to the base class
    fn upcast_mut(&mut self) -> &mut T {
        let ptr = self as *const Self;
        unsafe {
            let base = Self::upcast_ptr(ptr) as *mut T;
            &mut *base
        }
    }

    /// Upcast a pinned mutable reference to self to a pinned mutable reference to the base class
    fn upcast_pin(self: Pin<&mut Self>) -> Pin<&mut T> {
        let this = self.deref() as *const Self;
        unsafe {
            let base = Self::upcast_ptr(this) as *mut T;
            Pin::new_unchecked(&mut *base)
        }
    }
}

/// This trait is automatically implemented by CXX-Qt and you most likely do not need to manually implement it.
/// Trait for downcasting to a subclass, provided the subclass implements [Upcast] to this type.
/// Returns `None` in cases where `Sub` isn't a child class of `Self`.
pub trait Downcast: Sized {
    /// Try to downcast to a subclass of this type, given that the subclass upcasts to this type
    fn downcast<Sub: Upcast<Self>>(&self) -> Option<&Sub> {
        unsafe {
            let ptr = Sub::from_base_ptr(self as *const Self);
            if ptr.is_null() {
                None
            } else {
                Some(&*ptr)
            }
        }
    }

    /// Try to downcast mutably to a subclass of this, given that the subclass upcasts to this type
    fn downcast_mut<Sub: Upcast<Self>>(&mut self) -> Option<&mut Sub> {
        unsafe {
            let ptr = Sub::from_base_ptr(self as *const Self) as *mut Sub;
            if ptr.is_null() {
                None
            } else {
                Some(&mut *ptr)
            }
        }
    }

    /// Try to downcast a pin to a pinned subclass of this, given that the subclass upcasts to this type
    fn downcast_pin<Sub: Upcast<Self>>(self: Pin<&mut Self>) -> Option<Pin<&mut Sub>> {
        let this = self.deref() as *const Self;
        unsafe {
            let ptr = Sub::from_base_ptr(this) as *mut Sub;
            if ptr.is_null() {
                None
            } else {
                Some(Pin::new_unchecked(&mut *ptr))
            }
        }
    }
}

/// Automatic implementation of Downcast for any applicable types
impl<T: Sized> Downcast for T {}

unsafe impl<T> Upcast<T> for T {
    unsafe fn upcast_ptr(this: *const Self) -> *const Self {
        this
    }

    unsafe fn from_base_ptr(base: *const T) -> *const Self {
        base
    }

    fn upcast(&self) -> &Self {
        self
    }

    fn upcast_mut(&mut self) -> &mut Self {
        self
    }

    fn upcast_pin(self: Pin<&mut Self>) -> Pin<&mut Self> {
        self
    }
}
/// Implements transitive casting in a chain for a type and all its ancestors
///
/// Suppose you have 3 types, A, B and C where A -> B and B -> C casting relationships exist,
/// `impl_transitive_cast!(A, B, C)` will implement the relationship A -> C
///
/// `impl_transitive_cast!` will implement casting between the first type and ***all*** its ancestors.
/// For example, impl_transitive_cast!(A, B, C, D, E) will implement the following casts
/// - A -> C
/// - A -> D
/// - A -> E
///
/// # Example
///
/// ```
/// use cxx_qt::impl_transitive_cast;
///
///
/// #[derive(Debug)]
/// struct A {
///     parent: B
/// }
///
/// #[derive(Debug)]
/// struct B {
///     parent: C
/// }
///
/// #[derive(Debug)]
/// struct C {
///     parent: D
/// }
///
/// #[derive(Debug)]
/// struct D {
///     value: i32
/// }
///
/// use cxx_qt::casting::Upcast;
///
/// unsafe impl Upcast<B> for A {
///     unsafe fn upcast_ptr(this: *const Self) -> *const B {
///         unsafe { &(*this).parent }
///     }
///
///     unsafe fn from_base_ptr(base: *const B) -> *const Self {
///         std::ptr::null() // Not needed for this example
///     }
///
/// }
///
/// unsafe impl Upcast<C> for B {
///     unsafe fn upcast_ptr(this: *const Self) -> *const C {
///         unsafe { &(*this).parent }
///     }
///
///     unsafe fn from_base_ptr(base: *const C) -> *const Self {
///         std::ptr::null()
///     }
///
/// }
///
/// unsafe impl Upcast<D> for C {
///     unsafe fn upcast_ptr(this: *const Self) -> *const D {
///         unsafe { &(*this).parent }
///     }
///
///     unsafe fn from_base_ptr(base: *const D) -> *const Self {
///         std::ptr::null()
///     }
///
/// }
///
/// impl_transitive_cast!(A, B, C, D);
///
/// # // Note that we need a fake main function for doc tests to build.
/// # fn main() {
/// #    cxx_qt::init_crate!(cxx_qt);
/// #
/// #    let a = A {
/// #           parent: B {
/// #               parent: C {
/// #                   parent: D {
/// #                       value: 25
/// #                   }
/// #               }
/// #           }
/// #       };
/// #    assert_eq!(Upcast::<D>::upcast(&a).value, 25);
/// # }
/// ```
#[macro_export]
macro_rules! impl_transitive_cast {
    ($first:ty, $second:ty, $third:ty) => {
        // $crate::impl_transitive_cast!($first, $second, $third);
        unsafe impl ::cxx_qt::casting::Upcast<$third> for $first {
            unsafe fn upcast_ptr(this: *const Self) -> *const $third {
                let base = <Self as Upcast<$second>>::upcast_ptr(this);
                <$second as Upcast<$third>>::upcast_ptr(base)
            }

            unsafe fn from_base_ptr(base: *const $third) -> *const Self {
                let base = <$second as Upcast<$third>>::from_base_ptr(base);
                if base.is_null() {
                    std::ptr::null()
                } else {
                    <Self as Upcast<$second>>::from_base_ptr(base)
                }
            }
        }
    };

    ($first:ty, $second:ty, $third:ty, $($rest:ty),*) => {
        impl_transitive_cast!($first, $second, $third);
        impl_transitive_cast!($first, $third, $($rest),*);
    };
}
