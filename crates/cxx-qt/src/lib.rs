// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![deny(missing_docs)]

//! This crate and its associated crates provide a framework for generating QObjects from Rust.
//!
//! See the [book](https://kdab.github.io/cxx-qt/book/) for more information.

mod cxxqtthread;

pub use cxx_qt_macro::bridge;
pub use cxx_qt_macro::qobject;

pub use cxxqtthread::CxxQtThread;

/// This trait is automatically implemented for all types which are marked as `#[cxx_qt::qobject]`.
/// It provides information about the type that is wrapped by the QObject, as well as the methods
/// that Cxx-Qt will generate for the QObject.
pub trait CxxQtType {
    /// The Rust type that this QObject is wrapping.
    type Rust;

    /// Retrieve an immutable reference to the Rust struct backing this C++ object
    fn rust(&self) -> &Self::Rust;

    /// Retrieve a mutable reference to the Rust struct backing this C++ object
    fn rust_mut(self: core::pin::Pin<&mut Self>) -> core::pin::Pin<&mut Self::Rust>;
}

/// Types which implement the `Locking` trait are guarded from concurrent access in C++ (the default in CXX-Qt).
///
/// # Safety
///
/// This is a marker trait used to disable locking.
///
/// By default, CXX-Qt will guard all access to the generated QObject with a recursive mutex.
/// For performance reasons it may be desirable to disable this behavior for certain QObjects.
/// You can do so by negative implementing this trait `unsafe impl !cxx_qt::Locking for qobject::T {}`.
///
/// However, this is unsafe, as it may lead to concurrent mutable access to the QObject from C++.
/// You are responsible for ensuring this does not happen!
//
// This could be implemented using an auto trait in the future once stable
// https://doc.rust-lang.org/beta/unstable-book/language-features/auto-traits.html
pub trait Locking {
    // empty
}

/// Indicates that the object implements threading and has a method which returns a [CxxQtThread].
///
/// This trait is implemented by CxxQt automatically.
/// To enable this for a `qobject::T`, add `impl cxx_qt::Threading for qobject::T {}` to your [`#[cxx_qt::bridge]`](bridge).
pub trait Threading: Locking + Sized {
    #[doc(hidden)]
    type BoxedQueuedFn;
    #[doc(hidden)]
    type ThreadingTypeId;

    /// Create an instance of a [CxxQtThread]
    ///
    /// This allows for queueing closures onto the Qt event loop from a background thread.
    fn qt_thread(&self) -> CxxQtThread<Self>;

    #[doc(hidden)]
    fn queue<F>(cxx_qt_thread: &CxxQtThread<Self>, f: F) -> Result<(), cxx::Exception>
    where
        F: FnOnce(core::pin::Pin<&mut Self>),
        F: Send + 'static;

    #[doc(hidden)]
    fn threading_clone(cxx_qt_thread: &CxxQtThread<Self>) -> CxxQtThread<Self>;

    #[doc(hidden)]
    fn threading_drop(cxx_qt_thread: &mut CxxQtThread<Self>);
}

/// This trait can be implemented on any [CxxQtType] to define a
/// custom constructor in C++ for the QObject.
///
/// The `Arguments` must be a tuple of CXX types that will be the arguments to the constructor in C++.
///
/// If this trait is implemented for a given [CxxQtType], it must also be declared inside the
/// [cxx_qt::bridge](bridge) macro.
/// See the example below.
///
/// Note that declaring an implementation of this trait will stop Cxx-Qt from generating a default constructor.
/// Therefore an implementation of `Default` is no longer required for the Rust type.
///
/// # Minimal Example
/// ```
/// #[cxx_qt::bridge]
/// mod qobject {
///     extern "RustQt" {
///         #[cxx_qt::qobject]
///         type MyStruct = super::MyStructRust;
///     }
///
///     // Declare that we want to use a custom constructor
///     // Note that the arguments must be a tuple of CXX types.
///     // Any associated types that aren't included here are assumed to be `()`.
///     impl cxx_qt::Constructor<(i32, String), NewArguments=(i32, String)> for qobject::MyStruct {}
/// }
///
/// // Struct without `Default` implementation
/// pub struct MyStructRust {
///     pub integer: i32,
///     pub string: String
/// }
///
/// impl cxx_qt::Constructor<(i32, String)> for qobject::MyStruct {
///     type BaseArguments = (); // Will be passed to the base class constructor
///     type InitializeArguments = (); // Will be passed to the "initialize" function
///     type NewArguments = (i32, String); // Will be passed to the "new" function
///
///     fn route_arguments(args: (i32, String)) -> (
///         Self::NewArguments,
///         Self::BaseArguments,
///         Self::InitializeArguments
///     ) {
///         (args, (), ())
///     }
///
///     fn new((integer, string): (i32, String)) -> MyStructRust {
///         MyStructRust {
///             integer,
///             string
///         }
///     }
/// }
///
/// # // Note that we need a fake main function for doc tests to build.
/// # fn main() {}
/// ```
///
/// # Pseudo Code for generated C++ Constructor
/// You can imagine this trait as creating a constructor roughly like this:
/// ```cpp
/// class MyCxxQtType : public QObject {
///     public:
///         MyCxxQtType(Arguments... args)
///             : QObject(Constructor::route_arguments(args).BaseArguments)
///             , m_rust(Constructor::new(Constructor::route_arguments(args).NewArguments))
///         {
///             Constructor::initialize(*this, Constructor::route_arguments(args).InitializeArguments);
///         }
/// }
/// ```
/// Note that in reality, `route_arguments` will only be called once and all arguments
/// will be moved, never copied.
pub trait Constructor<Arguments>: CxxQtType {
    /// The arguments that are passed to the [`new()`](Self::new) function to construct the inner Rust struct.
    /// This must be a tuple of CXX compatible types.
    ///
    /// This way QObjects can be constructed that need additional arguments for constructing the
    /// inner Rust type.
    type NewArguments;
    /// The arguments that should be passed to the constructor of the base class.
    /// This must be a tuple of CXX compatible types.
    type BaseArguments;
    /// The arguments that should be used to initialize the QObject in the [`initialize()`](Self::initialize) function.
    /// This must be a tuple of CXX compatible types.
    type InitializeArguments;

    /// This function is called by CXX-Qt to route the arguments to the correct places.
    ///
    /// Using this function, you can split up the arguments required by the QObject constructor
    /// without additional copies.
    ///
    #[allow(unused_variables)]
    fn route_arguments(
        arguments: Arguments,
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    );

    /// This function is called to construct the inner Rust struct of the CXX-Qt QObject.
    /// You can use this to construct Rust structs that do not provide a `Default` implementation.
    fn new(arguments: Self::NewArguments) -> <Self as CxxQtType>::Rust;

    /// This function is called to initialize the QObject.
    /// After the members of the QObject is initialized, this function is called.
    /// This is equivalent to the body of the constructor in C++.
    ///
    /// # Default
    /// By default, this function does nothing
    #[allow(unused_variables)]
    fn initialize(self: core::pin::Pin<&mut Self>, arguments: Self::InitializeArguments) {
        // By default, do nothing
    }
}
