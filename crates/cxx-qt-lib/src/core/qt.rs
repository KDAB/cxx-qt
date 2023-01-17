// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge(namespace = "Qt")]
mod ffi {
    /// This enum type defines what happens to the aspect ratio when scaling an rectangle.
    #[repr(i32)]
    enum AspectRatioMode {
        /// The size is scaled freely. The aspect ratio is not preserved.
        IgnoreAspectRatio,
        /// The size is scaled to a rectangle as large as possible inside a given rectangle, preserving the aspect ratio.
        KeepAspectRatio,
        /// The size is scaled to a rectangle as small as possible outside a given rectangle, preserving the aspect ratio.
        KeepAspectRatioByExpanding,
    }

    #[repr(i32)]
    enum CaseSensitivity {
        CaseInsensitive,
        CaseSensitive,
    }

    #[repr(i32)]
    enum DateFormat {
        TextDate = 0,
        ISODateWithMs = 9,
        ISODate = 1,
        RFC2822Date = 8,
    }

    #[repr(i32)]
    enum SplitBehaviorFlags {
        KeepEmptyParts,
        SkipEmptyParts,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type AspectRatioMode;
        type CaseSensitivity;
        type DateFormat;
        type SplitBehaviorFlags;
    }
}

pub use ffi::{AspectRatioMode, CaseSensitivity, DateFormat, SplitBehaviorFlags};
