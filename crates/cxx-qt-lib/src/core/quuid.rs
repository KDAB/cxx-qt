// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{QByteArray, QString};
use cxx::{type_id, ExternType};
use std::{fmt, mem};
#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum QUuidVariant {
        /// Variant is unknown
        VarUnknown = -1,
        /// Reserved for NCS (Network Computing System) backward compatibility
        NCS = 0,
        /// Distributed Computing Environment, the scheme used by QUuid
        DCE = 2,
        /// Reserved for Microsoft backward compatibility (GUID)
        Microsoft = 6,
        /// Reserved for future definition
        Reserved = 7,
    }

    #[repr(i32)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum QUuidVersion {
        /// Version is unknown
        VerUnknown = -1,
        /// Time-based, by using timestamp, clock sequence, and MAC network card address (if
        /// available) for the node sections
        Time = 1,
        /// DCE Security version, with embedded POSIX UUIDs
        EmbeddedPOSIX = 2,
        /// Name-based, by using values from a name for all sections
        Md5 = 3,
        /// Random-based, by using random numbers for all sections
        Random = 4,
        Sha1 = 5,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/quuid.h");
        type QUuid = super::QUuid;
        type QUuidVariant;
        type QUuidVersion;

        /// Returns the value in the variant field of the UUID. If the return value is
        /// `QUuidVariant::DCE`, call `version()` to see which layout it uses. The null UUID is
        /// considered to be of an unknown variant.
        fn variant(self: &QUuid) -> QUuidVariant;

        /// Returns the version field of the UUID, if the UUID's variant field is `QUuidVariant::DCE`.
        /// Otherwise it returns `QUuidVariant::VerUnknown`.
        fn version(self: &QUuid) -> QUuidVersion;

        /// Returns the binary representation of this UUID. The byte array is in big endian format,
        /// and formatted according to RFC 4122, section 4.1.2 - "Layout and byte order".
        #[rust_name = "to_rfc_4122"]
        fn toRfc4122(self: &QUuid) -> QByteArray;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "quuid_create_uuid_v3"]
        fn quuidCreateUuidV3(ns: &QUuid, data: &[u8]) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_create_uuid"]
        fn quuidCreateUuid() -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_create_uuid_v5"]
        fn quuidCreateUuidV5(ns: &QUuid, data: &[u8]) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_to_string"]
        fn quuidToString(uuid: &QUuid) -> QString;
        #[doc(hidden)]
        #[rust_name = "quuid_from_string"]
        fn quuidFromString(string: &QString) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_from_str"]
        fn quuidFromStr(string: &str) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_from_rfc_4122"]
        fn quuidFromRfc4122(bytes: &QByteArray) -> QUuid;
    }
}

pub use ffi::{QUuidVariant, QUuidVersion};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct QUuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Default for QUuid {
    /// Creates the null UUID. `to_string()` will output the null UUID as
    /// "{00000000-0000-0000-0000-000000000000}".
    fn default() -> Self {
        Self::null()
    }
}

impl fmt::Display for QUuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::quuid_to_string(self).fmt(f)
    }
}

impl fmt::Debug for QUuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::quuid_to_string(self).fmt(f)
    }
}

impl QUuid {
    /// Creates the null UUID. `to_string()` will output the null UUID as
    /// "{00000000-0000-0000-0000-000000000000}".
    pub const fn null() -> Self {
        Self {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        }
    }

    /// Returns `true` if this is the null UUID {00000000-0000-0000-0000-000000000000};
    /// otherwise returns `false`.
    pub const fn is_null(&self) -> bool {
        (unsafe { std::mem::transmute::<QUuid, u128>(*self) }) == 0
    }

    /// This function returns a new UUID with variant `QUuidVariant::DCE` and version
    /// `QUuidVersion::Md5`. `namespace` is the namespace and `data` is the basic data as described
    /// by RFC 4122.
    pub fn create_uuid_v3(namespace: &Self, data: &[u8]) -> Self {
        ffi::quuid_create_uuid_v3(namespace, data)
    }

    /// On any platform other than Windows, this function returns a new UUID with variant
    /// `QUuidVariant::DCE` and version `QUuidVersion::Random`. On Windows, a GUID is generated using
    /// the Windows API and will be of the type that the API decides to create.
    pub fn create_uuid() -> Self {
        ffi::quuid_create_uuid()
    }

    /// This function returns a new UUID with variant `QUuidVariant::DCE` and version
    /// `QUuidVersion::Sha1`. `namespace` is the namespace and `data` is the basic data as described
    /// by RFC 4122.
    pub fn create_uuid_v5(namespace: &Self, data: &[u8]) -> Self {
        ffi::quuid_create_uuid_v5(namespace, data)
    }

    /// Creates a `QUuid` object from the binary representation of the UUID.
    /// The byte array is in big endian format, and formatted according to RFC 4122, section 4.1.2 -
    /// "Layout and byte order".
    ///
    /// The byte array accepted is NOT a human readable format.
    ///
    /// If the conversion fails, a null UUID is created.
    pub fn from_rfc_4122(bytes: &QByteArray) -> Self {
        ffi::quuid_from_rfc_4122(bytes)
    }

    /// Creates a UUID with the value specified by the parameters.
    pub const fn from_fields(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    pub const fn as_fields(&self) -> (u32, u16, u16, &[u8; 8]) {
        (self.data1, self.data2, self.data3, &self.data4)
    }

    /// Creates a UUID from its representation as a byte array in big endian.
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        // On big endian targets, this is a no-op.
        // On little endian targets, it swaps the bytes of each integer field (data1, data2, data3).
        unsafe { mem::transmute::<[u8; 16], Self>(bytes) }.to_be()
    }

    /// Returns the memory representation of this UUID as a byte array in big-endian byte order.
    pub const fn to_bytes(self) -> [u8; 16] {
        // On big endian targets, this is a no-op.
        // On little endian targets, it swaps the bytes of each integer field (data1, data2, data3).
        unsafe { mem::transmute::<Self, [u8; 16]>(self.to_be()) }
    }

    /// Creates a UUID from its representation as a 128-bit integer.
    pub const fn from_u128(data: u128) -> Self {
        Self::from_bytes(data.to_be_bytes())
    }

    /// Returns the memory representation of this UUID as a 128-bit integer.
    pub const fn to_u128(&self) -> u128 {
        u128::from_be_bytes(self.to_bytes())
    }

    /// Converts self to big endian from the target’s endianness.
    /// This function is analogous to [`u8::to_be`](https://doc.rust-lang.org/src/core/num/uint_macros.rs.html#399-431).
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    ///
    /// This is useful for converting between QUuids and byte arrays because byte array
    /// representations of UUIDs are always in big endian mode.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    const fn to_be(self) -> Self {
        #[cfg(target_endian = "big")]
        {
            self
        }
        #[cfg(target_endian = "little")]
        {
            Self {
                data1: self.data1.swap_bytes(),
                data2: self.data2.swap_bytes(),
                data3: self.data3.swap_bytes(),
                data4: self.data4,
            }
        }
    }
}

unsafe impl ExternType for QUuid {
    type Id = type_id!("QUuid");
    type Kind = cxx::kind::Trivial;
}

impl From<QUuid> for QString {
    fn from(value: QUuid) -> Self {
        ffi::quuid_to_string(&value)
    }
}

impl From<u128> for QUuid {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<QUuid> for u128 {
    fn from(value: QUuid) -> Self {
        value.to_u128()
    }
}

impl From<&QString> for QUuid {
    /// Creates a QUuid object from the string text, which must be formatted as five hex fields
    /// separated by '-', e.g., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where each 'x' is a hex
    /// digit. The curly braces shown here are optional, but it is normal to include them.
    ///
    /// If the conversion fails, a null UUID is returned.
    fn from(value: &QString) -> Self {
        ffi::quuid_from_string(value)
    }
}

impl From<&str> for QUuid {
    /// Creates a QUuid object from the string text, which must be formatted as five hex fields
    /// separated by '-', e.g., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where each 'x' is a hex
    /// digit. The curly braces shown here are optional, but it is normal to include them.
    ///
    /// If the conversion fails, a null UUID is returned.
    fn from(value: &str) -> Self {
        ffi::quuid_from_str(value)
    }
}

impl From<&String> for QUuid {
    /// Creates a QUuid object from the string text, which must be formatted as five hex fields
    /// separated by '-', e.g., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where each 'x' is a hex
    /// digit. The curly braces shown here are optional, but it is normal to include them.
    ///
    /// If the conversion fails, a null UUID is returned.
    fn from(value: &String) -> Self {
        ffi::quuid_from_str(value)
    }
}

impl From<&QByteArray> for QUuid {
    /// See [`QUuid::from_rfc_4122`].
    fn from(value: &QByteArray) -> Self {
        ffi::quuid_from_rfc_4122(value)
    }
}

impl From<QUuid> for QByteArray {
    /// See [`QUuid::to_rfc_4122`].
    fn from(value: QUuid) -> Self {
        value.to_rfc_4122()
    }
}

#[cfg(feature = "uuid")]
impl From<Uuid> for QUuid {
    fn from(value: Uuid) -> Self {
        let (data1, data2, data3, &data4) = value.as_fields();
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

#[cfg(feature = "uuid")]
impl From<QUuid> for Uuid {
    fn from(value: QUuid) -> Self {
        Self::from_fields(value.data1, value.data2, value.data3, &value.data4)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const NAMESPACE_DNS: &QUuid = &QUuid::from_u128(0x6ba7b8109dad11d180b400c04fd430c);

    #[test]
    fn quuid_is_null() {
        assert!(QUuid::null().is_null())
    }

    #[test]
    fn quuid_is_not_null() {
        assert!(!QUuid::create_uuid().is_null())
    }

    #[test]
    fn quuid_variant() {
        assert_eq!(
            [QUuid::null().variant(), QUuid::create_uuid().variant()],
            [QUuidVariant::VarUnknown, QUuidVariant::DCE]
        );
    }

    #[test]
    fn quuid_version() {
        assert_eq!(
            [
                QUuid::null().version(),
                QUuid::create_uuid_v3(NAMESPACE_DNS, &[]).version(),
                QUuid::create_uuid().version(),
                QUuid::create_uuid_v5(NAMESPACE_DNS, &[]).version(),
            ],
            [
                QUuidVersion::VerUnknown,
                QUuidVersion::Md5,
                QUuidVersion::Random,
                QUuidVersion::Sha1
            ]
        )
    }

    #[test]
    fn quuid_to_rfc_4122() {
        let bytes = <[u8; 16]>::try_from("random test data".as_bytes()).unwrap();
        assert_eq!(Vec::from(&QUuid::from_bytes(bytes).to_rfc_4122()), bytes)
    }

    #[test]
    fn quuid_null() {
        assert_eq!(QUuid::null(), QUuid::from_u128(0));
    }

    #[test]
    fn quuid_new_v3() {
        assert_eq!(
            QUuid::create_uuid_v3(NAMESPACE_DNS, "testdata".as_bytes()),
            QUuid::from_u128(0x5157facac7e1345c927671c2c6d41e7a)
        );
    }

    #[test]
    fn quuid_new_v4() {
        assert_ne!(QUuid::create_uuid(), QUuid::create_uuid());
    }

    #[test]
    fn quuid_new_v5() {
        assert_eq!(
            QUuid::create_uuid_v5(NAMESPACE_DNS, "testdata".as_bytes()),
            QUuid::from_u128(0x7e95e361a22c51c18c297ac24cb61e83)
        );
    }

    #[test]
    fn quuid_to_string() {
        assert_eq!(
            QUuid::from_u128(0x7e95e361a22c51c18c297ac24cb61e83).to_string(),
            "{7e95e361-a22c-51c1-8c29-7ac24cb61e83}"
        )
    }

    #[test]
    fn quuid_qstring_round_trip() {
        let uuid = QUuid::create_uuid();
        let roundtrip = QUuid::from(&QString::from(&uuid.to_string()));
        assert_eq!(uuid, roundtrip)
    }

    #[test]
    fn quuid_str_round_trip() {
        let uuid = QUuid::create_uuid();
        let roundtrip = QUuid::from(&uuid.to_string());
        assert_eq!(uuid, roundtrip)
    }

    #[test]
    fn quuid_fields_round_trip() {
        let uuid = QUuid::create_uuid();
        let (d1, d2, d3, &d4) = uuid.as_fields();
        let roundtrip = QUuid::from_fields(d1, d2, d3, d4);
        assert_eq!(uuid, roundtrip)
    }

    #[test]
    fn quuid_bytes_round_trip() {
        let uuid = QUuid::create_uuid();
        let roundtrip = QUuid::from_bytes(uuid.to_bytes());
        assert_eq!(uuid, roundtrip)
    }

    #[test]
    fn quuid_qbytearray_round_trip() {
        let uuid = QUuid::create_uuid();
        let roundtrip = QUuid::from_rfc_4122(&uuid.to_rfc_4122());
        assert_eq!(uuid, roundtrip)
    }

    #[test]
    fn quuid_u128_round_trip() {
        let uuid = QUuid::create_uuid();
        let roundtrip = QUuid::from_u128(uuid.to_u128());
        assert_eq!(uuid, roundtrip)
    }
}
