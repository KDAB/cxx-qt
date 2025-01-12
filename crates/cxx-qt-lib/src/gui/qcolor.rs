// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    /// How to format the output of the name() function
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    enum QColorNameFormat {
        /// A "#" character followed by three two-digit hexadecimal numbers (i.e. #RRGGBB).
        HexRgb,
        /// A "#" character followed by four two-digit hexadecimal numbers (i.e. #AARRGGBB).
        HexArgb,
    }

    /// The type of color specified, either RGB, extended RGB, HSV, CMYK or HSL.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    enum QColorSpec {
        Invalid,
        Rgb,
        Hsv,
        Cmyk,
        Hsl,
        ExtendedRgb,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qcolor.h");
        type QColor = super::QColor;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;

        /// Returns the alpha color component of this color.
        fn alpha(self: &QColor) -> i32;
        /// Returns the black color component of this color.
        fn black(self: &QColor) -> i32;
        /// Returns the blue color component of this color.
        fn blue(self: &QColor) -> i32;
        /// Creates a copy of this color in the format specified by colorSpec.
        #[rust_name = "convert_to"]
        fn convertTo(self: &QColor, spec: QColorSpec) -> QColor;
        /// Returns the cyan color component of this color.
        fn cyan(self: &QColor) -> i32;
        /// Returns a darker (or lighter) color, but does not change this object.
        fn darker(self: &QColor, factor: i32) -> QColor;
        /// Returns the green color component of this color.
        fn green(self: &QColor) -> i32;
        /// Returns the HSL hue color component of this color.
        #[rust_name = "hsl_hue"]
        fn hslHue(self: &QColor) -> i32;
        /// Returns the HSL saturation color component of this color.
        #[rust_name = "hsl_saturation"]
        fn hslSaturation(self: &QColor) -> i32;
        /// Returns the HSV hue color component of this color.
        #[rust_name = "hsv_hue"]
        fn hsvHue(self: &QColor) -> i32;
        /// Returns the HSV saturation color component of this color.
        #[rust_name = "hsv_saturation"]
        fn hsvSaturation(self: &QColor) -> i32;
        /// Returns the HSV hue color component of this color.
        ///
        /// The color is implicitly converted to HSV.
        fn hue(self: &QColor) -> i32;
        /// Returns true if the color is valid; otherwise returns false.
        #[rust_name = "is_valid"]
        fn isValid(self: &QColor) -> bool;
        /// Returns a lighter (or darker) color, but does not change this object.
        fn lighter(self: &QColor, factor: i32) -> QColor;
        /// Returns the lightness color component of this color.
        fn lightness(self: &QColor) -> i32;
        /// Returns the magenta color component of this color.
        fn magenta(self: &QColor) -> i32;
        /// Returns the name of the color in the specified format.
        fn name(self: &QColor, format: QColorNameFormat) -> QString;
        /// Returns the red color component of this color.
        fn red(self: &QColor) -> i32;
        /// Returns the HSV saturation color component of this color.
        ///
        /// The color is implicitly converted to HSV.
        fn saturation(self: &QColor) -> i32;
        /// Sets the alpha of this color to alpha. Integer alpha is specified in the range 0-255.
        #[rust_name = "set_alpha"]
        fn setAlpha(self: &mut QColor, alpha: i32);
        /// Sets the blue color component of this color to blue. Integer components are specified in the range 0-255.
        #[rust_name = "set_blue"]
        fn setBlue(self: &mut QColor, blue: i32);
        /// Sets the color to CMYK values, c (cyan), m (magenta), y (yellow), k (black), and a (alpha-channel, i.e. transparency).
        ///
        /// All the values must be in the range 0-255.
        #[rust_name = "set_cmyk"]
        fn setCmyk(self: &mut QColor, c: i32, m: i32, y: i32, k: i32, a: i32);
        /// Sets the green color component of this color to green. Integer components are specified in the range 0-255.
        #[rust_name = "set_green"]
        fn setGreen(self: &mut QColor, green: i32);
        /// Sets a HSL color value; h is the hue, s is the saturation, l is the lightness and a is the alpha component of the HSL color.
        ///
        /// The saturation, value and alpha-channel values must be in the range 0-255, and the hue value must be greater than -1.
        #[rust_name = "set_hsl"]
        fn setHsl(self: &mut QColor, h: i32, s: i32, l: i32, a: i32);
        /// Sets a HSV color value; h is the hue, s is the saturation, v is the value and a is the alpha component of the HSV color.
        ///
        /// The saturation, value and alpha-channel values must be in the range 0-255, and the hue value must be greater than -1.
        #[rust_name = "set_hsv"]
        fn setHsv(self: &mut QColor, h: i32, s: i32, v: i32, a: i32);
        /// Sets the red color component of this color to red. Integer components are specified in the range 0-255.
        #[rust_name = "set_red"]
        fn setRed(self: &mut QColor, red: i32);
        /// Sets the RGB value to r, g, b and the alpha value to a.
        ///
        /// All the values must be in the range 0-255.
        #[rust_name = "set_rgb"]
        fn setRgb(self: &mut QColor, r: i32, g: i32, b: i32, a: i32);
        /// Returns how the color was specified.
        fn spec(self: &QColor) -> QColorSpec;
        /// Creates and returns a CMYK QColor based on this color.
        #[rust_name = "to_cmyk"]
        fn toCmyk(self: &QColor) -> QColor;
        /// Create and returns an extended RGB QColor based on this color.
        #[rust_name = "to_extended_rgb"]
        fn toExtendedRgb(self: &QColor) -> QColor;
        /// Creates and returns an HSL QColor based on this color.
        #[rust_name = "to_hsl"]
        fn toHsl(self: &QColor) -> QColor;
        /// Creates and returns an HSV QColor based on this color.
        #[rust_name = "to_hsv"]
        fn toHsv(self: &QColor) -> QColor;
        /// Create and returns an RGB QColor based on this color.
        #[rust_name = "to_rgb"]
        fn toRgb(self: &QColor) -> QColor;
        /// Returns the value color component of this color.
        fn value(self: &QColor) -> i32;
        /// Returns the yellow color component of this color.
        fn yellow(self: &QColor) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        type QColorNameFormat;
        type QColorSpec;

        #[doc(hidden)]
        #[rust_name = "qcolor_color_names"]
        fn qcolorColorNames() -> QStringList;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_cmyk"]
        fn qcolorInitFromCmyk(c: i32, m: i32, y: i32, k: i32, a: i32) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_cmyk_f"]
        fn qcolorInitFromCmykF(c: f32, m: f32, y: f32, k: f32, a: f32) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_hsl"]
        fn qcolorInitFromHsl(h: i32, s: i32, l: i32, a: i32) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_hsl_f"]
        fn qcolorInitFromHslF(h: f32, s: f32, l: f32, a: f32) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_hsv"]
        fn qcolorInitFromHsv(h: i32, s: i32, v: i32, a: i32) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_hsv_f"]
        fn qcolorInitFromHsvF(h: f32, s: f32, v: f32, a: f32) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_rgb"]
        fn qcolorInitFromRgb(red: i32, green: i32, blue: i32, alpha: i32) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_rgb_f"]
        fn qcolorInitFromRgbF(red: f32, green: f32, blue: f32, alpha: f32) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_rust_string"]
        fn qcolorInitFromRustString(string: &str) -> QColor;

        #[doc(hidden)]
        #[rust_name = "qcolor_alpha_f"]
        fn qcolorAlphaF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_black_f"]
        fn qcolorBlackF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_blue_f"]
        fn qcolorBlueF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_cyan_f"]
        fn qcolorCyanF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_green_f"]
        fn qcolorGreenF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_hsl_hue_f"]
        fn qcolorHslHueF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_hsl_saturation_f"]
        fn qcolorHslSaturationF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_hsv_hue_f"]
        fn qcolorHsvHueF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_hsv_saturation_f"]
        fn qcolorHsvSaturationF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_hue_f"]
        fn qcolorHueF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_lightness_f"]
        fn qcolorLightnessF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_magenta_f"]
        fn qcolorMagentaF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_red_f"]
        fn qcolorRedF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_saturation_f"]
        fn qcolorSaturationF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_set_alpha_f"]
        fn qcolorSetAlphaF(color: &mut QColor, alpha: f32);
        #[doc(hidden)]
        #[rust_name = "qcolor_set_blue_f"]
        fn qcolorSetBlueF(color: &mut QColor, blue: f32);
        #[doc(hidden)]
        #[rust_name = "qcolor_set_cmyk_f"]
        fn qcolorSetCmykF(color: &mut QColor, c: f32, m: f32, y: f32, k: f32, a: f32);
        #[doc(hidden)]
        #[rust_name = "qcolor_set_green_f"]
        fn qcolorSetGreenF(color: &mut QColor, green: f32);
        #[doc(hidden)]
        #[rust_name = "qcolor_set_hsl_f"]
        fn qcolorSetHslF(color: &mut QColor, h: f32, s: f32, l: f32, a: f32);
        #[doc(hidden)]
        #[rust_name = "qcolor_set_hsv_f"]
        fn qcolorSetHsvF(color: &mut QColor, h: f32, s: f32, v: f32, a: f32);
        #[doc(hidden)]
        #[rust_name = "qcolor_set_red_f"]
        fn qcolorSetRedF(color: &mut QColor, red: f32);
        #[doc(hidden)]
        #[rust_name = "qcolor_set_rgb_f"]
        fn qcolorSetRgbF(color: &mut QColor, r: f32, g: f32, b: f32, a: f32);
        #[doc(hidden)]
        #[rust_name = "qcolor_value_f"]
        fn qcolorValueF(color: &QColor) -> f32;
        #[doc(hidden)]
        #[rust_name = "qcolor_yellow_f"]
        fn qcolorYellowF(color: &QColor) -> f32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qcolor_init_default"]
        fn construct() -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_init_from_qstring"]
        fn construct(name: &QString) -> QColor;
        #[doc(hidden)]
        #[rust_name = "qcolor_eq"]
        fn operatorEq(a: &QColor, b: &QColor) -> bool;
    }
}

pub use ffi::{QColorNameFormat, QColorSpec};

/// The QColor class provides colors based on RGB, HSL, HSV or CMYK values.
#[derive(Clone)]
#[repr(C)]
pub struct QColor {
    _cspec: MaybeUninit<i32>,
    _ct: MaybeUninit<[u16; 5]>,
}

impl QColor {
    /// Returns the alpha color component of this color.
    pub fn alpha_f(&self) -> f32 {
        ffi::qcolor_alpha_f(self)
    }

    /// Returns the black color component of this color.
    pub fn black_f(&self) -> f32 {
        ffi::qcolor_black_f(self)
    }

    /// Returns the blue color component of this color.
    pub fn blue_f(&self) -> f32 {
        ffi::qcolor_blue_f(self)
    }

    /// Returns a QStringList containing the color names Qt knows about.
    pub fn color_names() -> ffi::QStringList {
        ffi::qcolor_color_names()
    }

    /// Returns the cyan color component of this color.
    pub fn cyan_f(&self) -> f32 {
        ffi::qcolor_cyan_f(self)
    }

    /// Constructs a QColor from the CMYK value c, m, y, and k.
    pub fn from_cmyk(c: i32, m: i32, y: i32, k: i32) -> Self {
        ffi::qcolor_init_from_cmyk(c, m, y, k, 255)
    }

    /// Constructs a QColor from the CMYK value c, m, y, k, and the alpha-channel (transparency) value of a.
    pub fn from_cmyka(c: i32, m: i32, y: i32, k: i32, a: i32) -> Self {
        ffi::qcolor_init_from_cmyk(c, m, y, k, a)
    }

    /// Constructs a QColor from the CMYK value c, m, y, and k.
    pub fn from_cmyk_f(c: f32, m: f32, y: f32, k: f32) -> Self {
        ffi::qcolor_init_from_cmyk_f(c, m, y, k, 1.0)
    }

    /// Constructs a QColor from the CMYK value c, m, y, k, and the alpha-channel (transparency) value of a.
    pub fn from_cmyka_f(c: f32, m: f32, y: f32, k: f32, a: f32) -> Self {
        ffi::qcolor_init_from_cmyk_f(c, m, y, k, a)
    }

    /// Constructs a QColor from the HSL value h, s, and l.
    pub fn from_hsl(h: i32, s: i32, l: i32) -> Self {
        ffi::qcolor_init_from_hsl(h, s, l, 255)
    }

    /// Constructs a QColor from the HSL value h, s, l, and the alpha-channel (transparency) value of a.
    pub fn from_hsla(h: i32, s: i32, l: i32, a: i32) -> Self {
        ffi::qcolor_init_from_hsl(h, s, l, a)
    }

    /// Constructs a QColor from the HSL value h, s, and l.
    pub fn from_hsl_f(h: f32, s: f32, l: f32) -> Self {
        ffi::qcolor_init_from_hsl_f(h, s, l, 1.0)
    }

    /// Constructs a QColor from the HSL value h, s, l, and the alpha-channel (transparency) value of a.
    pub fn from_hsla_f(h: f32, s: f32, l: f32, a: f32) -> Self {
        ffi::qcolor_init_from_hsl_f(h, s, l, a)
    }

    /// Constructs a QColor from the HSV value h, s, and v.
    pub fn from_hsv(h: i32, s: i32, v: i32) -> Self {
        ffi::qcolor_init_from_hsv(h, s, v, 255)
    }

    /// Constructs a QColor from the HSV value h, s, v, and the alpha-channel (transparency) value of a.
    pub fn from_hsva(h: i32, s: i32, v: i32, a: i32) -> Self {
        ffi::qcolor_init_from_hsv(h, s, v, a)
    }

    /// Constructs a QColor from the HSV value h, s, and v.
    pub fn from_hsv_f(h: f32, s: f32, v: f32) -> Self {
        ffi::qcolor_init_from_hsv_f(h, s, v, 1.0)
    }

    /// Constructs a QColor from the HSV value h, s, v, and the alpha-channel (transparency) value of a.
    pub fn from_hsva_f(h: f32, s: f32, v: f32, a: f32) -> Self {
        ffi::qcolor_init_from_hsv_f(h, s, v, a)
    }

    /// Constructs a QColor with the RGB value r, g, and b.
    ///
    /// The color is left invalid if any of the arguments are invalid.
    pub fn from_rgb(red: i32, green: i32, blue: i32) -> Self {
        ffi::qcolor_init_from_rgb(red, green, blue, 255)
    }

    /// Constructs a QColor with the RGB value r, g, b, and the alpha-channel (transparency) value of a.
    ///
    /// The color is left invalid if any of the arguments are invalid.
    pub fn from_rgba(red: i32, green: i32, blue: i32, alpha: i32) -> Self {
        ffi::qcolor_init_from_rgb(red, green, blue, alpha)
    }

    /// Constructs a QColor with the RGB value r, g, and b.
    pub fn from_rgb_f(red: f32, green: f32, blue: f32) -> Self {
        ffi::qcolor_init_from_rgb_f(red, green, blue, 1.0)
    }

    /// Constructs a QColor with the RGB value r, g, b, and the alpha-channel (transparency) value of a.
    pub fn from_rgba_f(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        ffi::qcolor_init_from_rgb_f(red, green, blue, alpha)
    }

    /// Returns the green color component of this color.
    pub fn green_f(&self) -> f32 {
        ffi::qcolor_green_f(self)
    }

    /// Returns the HSL hue color component of this color.
    pub fn hsl_hue_f(&self) -> f32 {
        ffi::qcolor_hsl_hue_f(self)
    }

    /// Returns the HSL saturation color component of this color.
    pub fn hsl_saturation_f(&self) -> f32 {
        ffi::qcolor_hsl_saturation_f(self)
    }

    /// Returns the hue color component of this color.
    pub fn hsv_hue_f(&self) -> f32 {
        ffi::qcolor_hsv_hue_f(self)
    }

    /// Returns the HSV saturation color component of this color.
    pub fn hsv_saturation_f(&self) -> f32 {
        ffi::qcolor_hsv_saturation_f(self)
    }

    /// Returns the HSV hue color component of this color.
    ///
    /// The color is implicitly converted to HSV.
    pub fn hue_f(self: &QColor) -> f32 {
        ffi::qcolor_hue_f(self)
    }

    /// Returns the lightness color component of this color.
    pub fn lightness_f(&self) -> f32 {
        ffi::qcolor_lightness_f(self)
    }

    /// Returns the magenta color component of this color.
    pub fn magenta_f(&self) -> f32 {
        ffi::qcolor_magenta_f(self)
    }

    /// Returns the red color component of this color.
    pub fn red_f(&self) -> f32 {
        ffi::qcolor_red_f(self)
    }

    /// Returns the HSV saturation color component of this color.
    ///
    /// The color is implicitly converted to HSV.
    pub fn saturation_f(&self) -> f32 {
        ffi::qcolor_saturation_f(self)
    }

    /// Sets the alpha of this color to alpha. float alpha is specified in the range 0.0-1.0.
    pub fn set_alpha_f(&mut self, alpha: f32) {
        ffi::qcolor_set_alpha_f(self, alpha);
    }

    /// Sets the blue color component of this color to blue.
    ///
    /// If blue lies outside the 0.0-1.0 range, the color model will be changed to ExtendedRgb.
    pub fn set_blue_f(&mut self, blue: f32) {
        ffi::qcolor_set_blue_f(self, blue);
    }

    /// Sets the color to CMYK values, c (cyan), m (magenta), y (yellow), k (black), and a (alpha-channel, i.e. transparency).
    ///
    /// All the values must be in the range 0.0-1.0.
    pub fn set_cmyk_f(&mut self, c: f32, m: f32, y: f32, k: f32, a: f32) {
        ffi::qcolor_set_cmyk_f(self, c, m, y, k, a);
    }

    /// Sets the green color component of this color to green.
    ///
    /// If green lies outside the 0.0-1.0 range, the color model will be changed to ExtendedRgb.
    pub fn set_green_f(&mut self, green: f32) {
        ffi::qcolor_set_green_f(self, green);
    }

    /// Sets a HSL color lightness; h is the hue, s is the saturation, l is the lightness and a is the alpha component of the HSL color.
    ///
    /// All the values must be in the range 0.0-1.0.
    pub fn set_hsl_f(&mut self, h: f32, s: f32, l: f32, a: f32) {
        ffi::qcolor_set_hsl_f(self, h, s, l, a);
    }

    /// Sets a HSV color value; h is the hue, s is the saturation, v is the value and a is the alpha component of the HSV color.
    ///
    /// All the values must be in the range 0.0-1.0.
    pub fn set_hsv_f(&mut self, h: f32, s: f32, v: f32, a: f32) {
        ffi::qcolor_set_hsv_f(self, h, s, v, a);
    }

    /// Sets the red color component of this color to red.
    ///
    /// If red lies outside the 0.0-1.0 range, the color model will be changed to ExtendedRgb.
    pub fn set_red_f(&mut self, red: f32) {
        ffi::qcolor_set_red_f(self, red);
    }

    /// Sets the color channels of this color to r (red), g (green), b (blue) and a (alpha, transparency).
    ///
    /// The alpha value must be in the range 0.0-1.0. If any of the other values are outside the range of 0.0-1.0 the color model will be set as ExtendedRgb.
    pub fn set_rgb_f(&mut self, r: f32, g: f32, b: f32, a: f32) {
        ffi::qcolor_set_rgb_f(self, r, g, b, a);
    }

    /// Returns the value color component of this color.
    pub fn value_f(self: &QColor) -> f32 {
        ffi::qcolor_value_f(self)
    }

    /// Returns the yellow color component of this color.
    pub fn yellow_f(self: &QColor) -> f32 {
        ffi::qcolor_yellow_f(self)
    }
}

impl Default for QColor {
    /// Constructs an invalid color. An invalid color is a color that is not properly set up for the underlying window system.
    ///
    /// The alpha value of an invalid color is unspecified.
    fn default() -> Self {
        ffi::qcolor_init_default()
    }
}

impl TryFrom<&str> for QColor {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let color = ffi::qcolor_init_from_rust_string(value);
        if color.is_valid() {
            Ok(color)
        } else {
            Err("Invalid color")
        }
    }
}

impl TryFrom<&String> for QColor {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let color = ffi::qcolor_init_from_rust_string(value);
        if color.is_valid() {
            Ok(color)
        } else {
            Err("Invalid color")
        }
    }
}

impl TryFrom<&ffi::QString> for QColor {
    type Error = &'static str;

    fn try_from(value: &ffi::QString) -> Result<Self, Self::Error> {
        let color = ffi::qcolor_init_from_qstring(value);
        if color.is_valid() {
            Ok(color)
        } else {
            Err("Invalid color")
        }
    }
}

impl std::cmp::PartialEq for QColor {
    fn eq(&self, other: &Self) -> bool {
        ffi::qcolor_eq(self, other)
    }
}

impl std::cmp::Eq for QColor {}

impl fmt::Display for QColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: consider the different color spec
        let r = self.red();
        let g = self.green();
        let b = self.blue();
        let a = self.alpha();
        write!(f, "RGBA({r}, {g}, {b}, {a})")
    }
}

impl fmt::Debug for QColor {
    // We use more fancy printing for the Debug formatter
    // If you dislike this, use the Display formatter instead
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: consider the different color spec
        let r = self.red();
        let g = self.green();
        let b = self.blue();
        let a = self.alpha();
        // very simple heuristic to use a light foreground if background is dark and vice versa
        let fg = if (r + b + g) < 384 { 255 } else { 0 };
        // Use terminal escape codes to **actually** print the color
        write!(f, "\x1b[48;2;{r};{g};{b}m\x1b[38;2;{fg};{fg};{fg}mRGBA({r}, {g}, {b}, {a})\x1b[39m\x1b[49m")
    }
}

#[cfg(feature = "rgb")]
impl From<&rgb::RGB8> for QColor {
    fn from(value: &rgb::RGB8) -> Self {
        Self::from_rgb(value.r as i32, value.g as i32, value.b as i32)
    }
}

#[cfg(feature = "rgb")]
impl From<&rgb::RGBA8> for QColor {
    fn from(value: &rgb::RGBA8) -> Self {
        Self::from_rgba(
            value.r as i32,
            value.g as i32,
            value.b as i32,
            value.a as i32,
        )
    }
}

#[cfg(feature = "rgb")]
impl From<&QColor> for rgb::RGB8 {
    fn from(value: &QColor) -> Self {
        Self {
            r: value.red() as u8,
            g: value.green() as u8,
            b: value.blue() as u8,
        }
    }
}

#[cfg(feature = "rgb")]
impl From<&QColor> for rgb::RGBA8 {
    fn from(value: &QColor) -> Self {
        Self {
            r: value.red() as u8,
            g: value.green() as u8,
            b: value.blue() as u8,
            a: value.alpha() as u8,
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for QColor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let format = if self.alpha() == 255 {
            ffi::QColorNameFormat::HexRgb
        } else {
            ffi::QColorNameFormat::HexArgb
        };
        self.name(format).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for QColor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = ffi::QString::deserialize(deserializer)?;
        Self::try_from(&string).map_err(|_| {
            use serde::de::{Error as _, Unexpected};
            D::Error::invalid_value(Unexpected::Str(&String::from(&string)), &"hex color code")
        })
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QColor {
    type Id = type_id!("QColor");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "rgb")]
    use super::*;

    #[cfg(feature = "rgb")]
    #[test]
    fn test_rgb() {
        let color = rgb::RGB8 {
            r: 0,
            g: 100,
            b: 255,
        };
        let qcolor = QColor::from(&color);
        assert_eq!(qcolor.red(), 0);
        assert_eq!(qcolor.green(), 100);
        assert_eq!(qcolor.blue(), 255);
        assert_eq!(qcolor.alpha(), 255);

        let rgb_color = rgb::RGB8::from(&qcolor);
        assert_eq!(color, rgb_color);
    }

    #[cfg(feature = "rgb")]
    #[test]
    fn test_rgba() {
        let color = rgb::RGBA8 {
            r: 0,
            g: 100,
            b: 255,
            a: 100,
        };
        let qcolor = QColor::from(&color);
        assert_eq!(qcolor.red(), 0);
        assert_eq!(qcolor.green(), 100);
        assert_eq!(qcolor.blue(), 255);
        assert_eq!(qcolor.alpha(), 100);

        let rgba_color = rgb::RGBA8::from(&qcolor);
        assert_eq!(color, rgba_color);
    }
}
