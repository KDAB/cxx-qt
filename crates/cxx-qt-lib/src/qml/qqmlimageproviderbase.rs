#[cxx_qt::bridge]
mod ffi {

    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    enum QQmlImageProviderBaseImageType {
        Image = 1,
        Pixmap,
        Texture,
        ImageResponse,
    }

    unsafe extern "C++" {

        include!("cxx-qt-lib/qqmlimageproviderbase.h");
        type QQmlImageProviderBase;


    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++"{
        type QQmlImageProviderBaseImageType;
    }
}

pub use ffi::QQmlImageProviderBase;
pub use ffi::QQmlImageProviderBaseImageType;
