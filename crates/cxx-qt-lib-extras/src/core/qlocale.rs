// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Nicolas Fella <nicolas.fella@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {

    #[repr(u8)]
    #[namespace = "rust::cxxqtlib1"]
    enum QLocaleTagSeparator {
        Dash = 45,       // -
        Underscore = 95, // _
    }

    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    enum QLocaleCurrencySymbolFormat {
        CurrencyIsoCode,
        CurrencySymbol,
        CurrencyDisplayName,
    }

    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    enum QLocaleFormatType {
        LongFormat,
        ShortFormat,
        NarrowFormat,
    }

    #[repr(u16)]
    #[namespace = "rust::cxxqtlib1"]
    enum QLocaleLanguage {
        AnyLanguage = 0,
        C = 1,
        Abkhazian = 2,
        Afar = 3,
        Afrikaans = 4,
        Aghem = 5,
        Akan = 6,
        Akkadian = 7,
        Akoose = 8,
        Albanian = 9,
        AmericanSignLanguage = 10,
        Amharic = 11,
        AncientEgyptian = 12,
        AncientGreek = 13,
        Arabic = 14,
        Aragonese = 15,
        Aramaic = 16,
        Armenian = 17,
        Assamese = 18,
        Asturian = 19,
        Asu = 20,
        Atsam = 21,
        Avaric = 22,
        Avestan = 23,
        Aymara = 24,
        Azerbaijani = 25,
        Bafia = 26,
        Balinese = 27,
        Bambara = 28,
        Bamun = 29,
        Bangla = 30,
        Basaa = 31,
        Bashkir = 32,
        Basque = 33,
        BatakToba = 34,
        Belarusian = 35,
        Bemba = 36,
        Bena = 37,
        Bhojpuri = 38,
        Bislama = 39,
        Blin = 40,
        Bodo = 41,
        Bosnian = 42,
        Breton = 43,
        Buginese = 44,
        Bulgarian = 45,
        Burmese = 46,
        Cantonese = 47,
        Catalan = 48,
        Cebuano = 49,
        CentralAtlasTamazight = 50,
        CentralKurdish = 51,
        Chakma = 52,
        Chamorro = 53,
        Chechen = 54,
        Cherokee = 55,
        Chickasaw = 56,
        Chiga = 57,
        Chinese = 58,
        Church = 59,
        Chuvash = 60,
        Colognian = 61,
        Coptic = 62,
        Cornish = 63,
        Corsican = 64,
        Cree = 65,
        Croatian = 66,
        Czech = 67,
        Danish = 68,
        Divehi = 69,
        Dogri = 70,
        Duala = 71,
        Dutch = 72,
        Dzongkha = 73,
        Embu = 74,
        English = 75,
        Erzya = 76,
        Esperanto = 77,
        Estonian = 78,
        Ewe = 79,
        Ewondo = 80,
        Faroese = 81,
        Fijian = 82,
        Filipino = 83,
        Finnish = 84,
        French = 85,
        Friulian = 86,
        Fulah = 87,
        Gaelic = 88,
        Ga = 89,
        Galician = 90,
        Ganda = 91,
        Geez = 92,
        Georgian = 93,
        German = 94,
        Gothic = 95,
        Greek = 96,
        Guarani = 97,
        Gujarati = 98,
        Gusii = 99,
        Haitian = 100,
        Hausa = 101,
        Hawaiian = 102,
        Hebrew = 103,
        Herero = 104,
        Hindi = 105,
        HiriMotu = 106,
        Hungarian = 107,
        Icelandic = 108,
        Ido = 109,
        Igbo = 110,
        InariSami = 111,
        Indonesian = 112,
        Ingush = 113,
        Interlingua = 114,
        Interlingue = 115,
        Inuktitut = 116,
        Inupiaq = 117,
        Irish = 118,
        Italian = 119,
        Japanese = 120,
        Javanese = 121,
        Jju = 122,
        JolaFonyi = 123,
        Kabuverdianu = 124,
        Kabyle = 125,
        Kako = 126,
        Kalaallisut = 127,
        Kalenjin = 128,
        Kamba = 129,
        Kannada = 130,
        Kanuri = 131,
        Kashmiri = 132,
        Kazakh = 133,
        Kenyang = 134,
        Khmer = 135,
        Kiche = 136,
        Kikuyu = 137,
        Kinyarwanda = 138,
        Komi = 139,
        Kongo = 140,
        Konkani = 141,
        Korean = 142,
        Koro = 143,
        KoyraboroSenni = 144,
        KoyraChiini = 145,
        Kpelle = 146,
        Kuanyama = 147,
        Kurdish = 148,
        Kwasio = 149,
        Kyrgyz = 150,
        Lakota = 151,
        Langi = 152,
        Lao = 153,
        Latin = 154,
        Latvian = 155,
        Lezghian = 156,
        Limburgish = 157,
        Lingala = 158,
        LiteraryChinese = 159,
        Lithuanian = 160,
        Lojban = 161,
        LowerSorbian = 162,
        LowGerman = 163,
        LubaKatanga = 164,
        LuleSami = 165,
        Luo = 166,
        Luxembourgish = 167,
        Luyia = 168,
        Macedonian = 169,
        Machame = 170,
        Maithili = 171,
        MakhuwaMeetto = 172,
        Makonde = 173,
        Malagasy = 174,
        Malayalam = 175,
        Malay = 176,
        Maltese = 177,
        Mandingo = 178,
        Manipuri = 179,
        Manx = 180,
        Maori = 181,
        Mapuche = 182,
        Marathi = 183,
        Marshallese = 184,
        Masai = 185,
        Mazanderani = 186,
        Mende = 187,
        Meru = 188,
        Meta = 189,
        Mohawk = 190,
        Mongolian = 191,
        Morisyen = 192,
        Mundang = 193,
        Muscogee = 194,
        Nama = 195,
        NauruLanguage = 196,
        Navajo = 197,
        Ndonga = 198,
        Nepali = 199,
        Newari = 200,
        Ngiemboon = 201,
        Ngomba = 202,
        NigerianPidgin = 203,
        Nko = 204,
        NorthernLuri = 205,
        NorthernSami = 206,
        NorthernSotho = 207,
        NorthNdebele = 208,
        NorwegianBokmal = 209,
        NorwegianNynorsk = 210,
        Nuer = 211,
        Nyanja = 212,
        Nyankole = 213,
        Occitan = 214,
        Odia = 215,
        Ojibwa = 216,
        OldIrish = 217,
        OldNorse = 218,
        OldPersian = 219,
        Oromo = 220,
        Osage = 221,
        Ossetic = 222,
        Pahlavi = 223,
        Palauan = 224,
        Pali = 225,
        Papiamento = 226,
        Pashto = 227,
        Persian = 228,
        Phoenician = 229,
        Polish = 230,
        Portuguese = 231,
        Prussian = 232,
        Punjabi = 233,
        Quechua = 234,
        Romanian = 235,
        Romansh = 236,
        Rombo = 237,
        Rundi = 238,
        Russian = 239,
        Rwa = 240,
        Saho = 241,
        Sakha = 242,
        Samburu = 243,
        Samoan = 244,
        Sango = 245,
        Sangu = 246,
        Sanskrit = 247,
        Santali = 248,
        Sardinian = 249,
        Saurashtra = 250,
        Sena = 251,
        Serbian = 252,
        Shambala = 253,
        Shona = 254,
        SichuanYi = 255,
        Sicilian = 256,
        Sidamo = 257,
        Silesian = 258,
        Sindhi = 259,
        Sinhala = 260,
        SkoltSami = 261,
        Slovak = 262,
        Slovenian = 263,
        Soga = 264,
        Somali = 265,
        SouthernKurdish = 266,
        SouthernSami = 267,
        SouthernSotho = 268,
        SouthNdebele = 269,
        Spanish = 270,
        StandardMoroccanTamazight = 271,
        Sundanese = 272,
        Swahili = 273,
        Swati = 274,
        Swedish = 275,
        SwissGerman = 276,
        Syriac = 277,
        Tachelhit = 278,
        Tahitian = 279,
        TaiDam = 280,
        Taita = 281,
        Tajik = 282,
        Tamil = 283,
        Taroko = 284,
        Tasawaq = 285,
        Tatar = 286,
        Telugu = 287,
        Teso = 288,
        Thai = 289,
        Tibetan = 290,
        Tigre = 291,
        Tigrinya = 292,
        TokelauLanguage = 293,
        TokPisin = 294,
        Tongan = 295,
        Tsonga = 296,
        Tswana = 297,
        Turkish = 298,
        Turkmen = 299,
        TuvaluLanguage = 300,
        Tyap = 301,
        Ugaritic = 302,
        Ukrainian = 303,
        UpperSorbian = 304,
        Urdu = 305,
        Uyghur = 306,
        Uzbek = 307,
        Vai = 308,
        Venda = 309,
        Vietnamese = 310,
        Volapuk = 311,
        Vunjo = 312,
        Walloon = 313,
        Walser = 314,
        Warlpiri = 315,
        Welsh = 316,
        WesternBalochi = 317,
        WesternFrisian = 318,
        Wolaytta = 319,
        Wolof = 320,
        Xhosa = 321,
        Yangben = 322,
        Yiddish = 323,
        Yoruba = 324,
        Zarma = 325,
        Zhuang = 326,
        Zulu = 327,
        Kaingang = 328,
        Nheengatu = 329,
        Haryanvi = 330,
        NorthernFrisian = 331,
        Rajasthani = 332,
        Moksha = 333,
        TokiPona = 334,
        Pijin = 335,
        Obolo = 336,
        Baluchi = 337,
        Ligurian = 338,
        Rohingya = 339,
        Torwali = 340,
        Anii = 341,
        Kangri = 342,
        Venetian = 343,
        Kuvi = 344,

        Afan = 220,                   // Oromo,
        Bengali = 30,                 // Bangla,
        Bhutani = 73,                 // Dzongkha,
        Byelorussian = 35,            // Belarusian,
        Cambodian = 135,              // Khmer,
        CentralMoroccoTamazight = 50, // CentralAtlasTamazight,
        Chewa = 212,                  // Nyanja,
        Frisian = 318,                // WesternFrisian,
        Greenlandic = 127,            // Kalaallisut,
        Inupiak = 117,                // Inupiaq,
        Kirghiz = 150,                // Kyrgyz,
        Kurundi = 238,                // Rundi,
        Kwanyama = 147,               // Kuanyama,
        Navaho = 197,                 // Navajo,
        Oriya = 215,                  // Odia,
        RhaetoRomance = 236,          // Romansh,
        Uighur = 306,                 // Uyghur,
        Uigur = 306,                  // Uyghur,
        Walamo = 319,                 // Wolaytta,

        LastLanguage = 344, // Kuvi,
    }

    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    enum QLocaleMeasurementSystem {
        MetricSystem = 0,
        ImperialUSSystem = 1,
        ImperialUKSystem = 2,
        ImperialSystem = 1, // ImperialUSSystem
    }

    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type DayOfWeek = cxx_qt_lib::DayOfWeek;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        include!("cxx-qt-lib-extras/qlocale.h");
        type QLocale = super::QLocale;

        /// Returns the localized name of the "AM" suffix for times specified using the conventions of the 12-hour clock.
        #[rust_name = "am_text"]
        fn amText(self: &QLocale) -> QString;

        /// Returns the BCP47 field names joined with dashes.
        // QString QLocale::bcp47Name(QLocale::TagSeparator separator = TagSeparator::Dash) const
        #[rust_name = "bcp47_name"]
        fn bcp47Name(self: &QLocale, separator: QLocaleTagSeparator) -> QString;

        /// Returns the locale to use for collation.
        #[rust_name = "collation"]
        fn collation(self: &QLocale) -> QLocale;

        #[rust_name = "create_separated_list"]
        fn createSeparatedList(self: &QLocale, list: &QStringList) -> QString;

        /// Returns a currency symbol according to the format.
        // QString QLocale::currencySymbol(QLocale::CurrencySymbolFormat format = CurrencySymbol) const
        #[rust_name = "currency_symbol"]
        fn currencySymbol(self: &QLocale, format: QLocaleCurrencySymbolFormat) -> QString;

        /// Returns the date format used for the current locale.
        #[rust_name = "date_format"]
        fn dateFormat(self: &QLocale, format: QLocaleFormatType) -> QString;

        /// Returns the date time format used for the current locale.
        #[rust_name = "date_time_format"]
        fn dateTimeFormat(self: &QLocale, format: QLocaleFormatType) -> QString;

        /// Returns the localized name of the day (where 1 represents Monday, 2 represents Tuesday and so on), in the format specified by type.
        // QString QLocale::dayName(int day, QLocale::FormatType type = LongFormat) const
        #[rust_name = "day_name"]
        fn dayName(self: &QLocale, day: i32, formatType: QLocaleFormatType) -> QString;

        /// Returns the fractional part separator for this locale.
        #[rust_name = "decimal_point"]
        fn decimalPoint(self: &QLocale) -> QString;

        /// Returns the exponent separator for this locale.
        #[rust_name = "exponential"]
        fn exponential(self: &QLocale) -> QString;

        /// Returns the first day of the week according to the current locale.
        // Qt::DayOfWeek QLocale::firstDayOfWeek() const
        #[rust_name = "first_day_of_week"]
        fn firstDayOfWeek(self: &QLocale) -> DayOfWeek;

        // QString QLocale::formattedDataSize(qint64 bytes, int precision = 2, QLocale::DataSizeFormats format = DataSizeIecFormat) const

        /// Returns the digit-grouping separator for this locale.
        #[rust_name = "group_separator"]
        fn groupSeparator(self: &QLocale) -> QString;

        /// Returns the language of this locale.
        // QLocale::Language QLocale::language() const
        #[rust_name = "language"]
        fn language(self: &QLocale) -> QLocaleLanguage;

        /// Returns the two- or three-letter language code for language, as defined in the ISO 639 standards.
        // QString QLocale::languageToCode(QLocale::Language language, QLocale::LanguageCodeTypes codeTypes = AnyLanguageCode)

        /// Returns a QString containing the name of language.
        // static QString QLocale::languageToString(QLocale::Language language)
        // #[rust_name = "language_to_string"]
        // fn languageToString(language: QLocaleLanguage) -> QString;

        // QList<QLocale> QLocale::matchingLocales(QLocale::Language language, QLocale::Script script, QLocale::Territory territory)

        /// Returns the measurement system for the locale.
        // QLocale::MeasurementSystem QLocale::measurementSystem() const
        #[rust_name = "measurement_system"]
        fn measurementSystem(self: &QLocale) -> QLocaleMeasurementSystem;

        /// Returns the localized name of month, in the format specified by type.
        // QString QLocale::monthName(int month, QLocale::FormatType type = LongFormat) const
        #[rust_name = "month_name"]
        fn monthName(self: &QLocale, month: i32, formatType: QLocaleFormatType) -> QString;

        /// The short name of this locale.
        // QString QLocale::name(QLocale::TagSeparator separator = TagSeparator::Underscore) const
        #[rust_name = "name"]
        fn name(self: &QLocale, separator: QLocaleTagSeparator) -> QString;

        /// Returns a native name of the language for the locale. For example "Schweizer Hochdeutsch" for the Swiss-German locale.
        #[rust_name = "native_language_name"]
        fn nativeLanguageName(self: &QLocale) -> QString;

        /// Returns a native name of the territory for the locale. For example "España" for Spanish/Spain locale.
        #[rust_name = "native_territory_name"]
        #[cfg(cxxqt_qt_version_major = "6")]
        fn nativeTerritoryName(self: &QLocale) -> QString;

    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {

        type QLocaleTagSeparator;
        type QLocaleCurrencySymbolFormat;
        type QLocaleFormatType;
        type QLocaleLanguage;
        type QLocaleMeasurementSystem;

        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qlocale_init_default"]
        fn construct() -> QLocale;

        #[doc(hidden)]
        #[allow(dead_code)]
        #[rust_name = "qlocale_init_from_name"]
        fn construct(name: &QString) -> QLocale;

        #[doc(hidden)]
        #[rust_name = "qlocale_drop"]
        fn drop(locale: &mut QLocale);

        #[doc(hidden)]
        #[rust_name = "qlocale_init_from_qlocale"]
        fn construct(locale: &QLocale) -> QLocale;
    }
}

pub use ffi::QLocaleCurrencySymbolFormat;
pub use ffi::QLocaleFormatType;
pub use ffi::QLocaleLanguage;
pub use ffi::QLocaleMeasurementSystem;
pub use ffi::QLocaleTagSeparator;

#[repr(C)]
pub struct QLocale {
    _cspec: MaybeUninit<usize>,
}

impl Default for QLocale {
    fn default() -> Self {
        ffi::qlocale_init_default()
    }
}

impl Drop for QLocale {
    fn drop(&mut self) {
        ffi::qlocale_drop(self)
    }
}

impl Clone for QLocale {
    fn clone(&self) -> Self {
        ffi::qlocale_init_from_qlocale(self)
    }
}

// Safety:

// Static checks on the C++ side ensure that QSize is trivial.
unsafe impl ExternType for QLocale {
    type Id = type_id!("QLocale");
    type Kind = cxx::kind::Trivial;
}
