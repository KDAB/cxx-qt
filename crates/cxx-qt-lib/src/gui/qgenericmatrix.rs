// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::slice;

use cxx::{type_id, ExternType};

/// The QGenericMatrix class is a template class that represents a NxM
/// transformation matrix with N columns and M rows.
///
/// Note: CXX-Qt currently only supports QGenericMatrix of f32, while the C++
/// QGenericMatrix is generic over the contained type.
///
/// Qt Documentation: [QGenericMatrix](https://doc.qt.io/qt/qgenericmatrix.html#details)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct QGenericMatrix<const N: usize, const M: usize> {
    data: [[f32; M]; N],
}

impl<const N: usize, const M: usize> QGenericMatrix<N, M> {
    /// Constructs a matrix from floating-point `values` in row-major order.
    pub const fn new(values: &[[f32; N]; M]) -> Self {
        let mut data = [[0.0; M]; N];
        let mut col = 0;
        while col < N {
            let mut row = 0;
            while row < M {
                data[col][row] = values[row][col];
                row += 1;
            }
            col += 1;
        }
        Self { data }
    }

    /// Returns a reference to the raw data of this matrix.
    pub const fn data(&self) -> &[f32] {
        // TODO: Replace with `array::as_flattened` once MSRV is 1.80.0.
        unsafe { slice::from_raw_parts(self.data.as_ptr().cast(), N * M) }
    }

    /// Returns a mutable reference to the raw data of this matrix.
    pub fn data_mut(&mut self) -> &mut [f32] {
        // TODO: Replace with `array::as_flattened_mut` once MSRV is 1.80.0.
        unsafe { slice::from_raw_parts_mut(self.data.as_mut_ptr().cast(), N * M) }
    }

    /// Retrieves the N * M items in this matrix and copies them to `values` in row-major order.
    pub fn copy_data_to(&self, values: &mut [f32]) {
        for (col, data) in self.data.iter().enumerate() {
            for (row, &value) in data.iter().enumerate() {
                values[row * N + col] = value;
            }
        }
    }

    /// Fills all elements of this matrix with `value`.
    pub fn fill(&mut self, value: f32) {
        self.data_mut().fill(value);
    }

    /// Constructs a matrix with all values set to `value`.
    pub const fn filled(value: f32) -> Self {
        Self {
            data: [[value; M]; N],
        }
    }

    /// Constructs a NxM identity matrix.
    pub const fn identity() -> Self {
        let mut data = [[0.0; M]; N];
        let mut i = 0;
        let size = if M < N { M } else { N };
        while i < size {
            data[i][i] = 1.0;
            i += 1;
        }
        Self { data }
    }

    /// Returns `true` if this matrix is the identity; `false` otherwise.
    pub fn is_identity(&self) -> bool {
        self == &Self::identity()
    }

    /// Constructs a two-dimensional array from the matrix in row-major order.
    pub const fn rows(&self) -> [[f32; N]; M] {
        self.transposed().data
    }

    /// Sets this matrix to the identity.
    pub fn set_to_identity(&mut self) {
        for (col, data) in self.data.iter_mut().enumerate() {
            for (row, value) in data.iter_mut().enumerate() {
                *value = if row == col { 1.0 } else { 0.0 };
            }
        }
    }

    /// Returns this matrix, transposed about its diagonal.
    pub const fn transposed(&self) -> QGenericMatrix<M, N> {
        let mut transposed = [[0.0; N]; M];
        let mut col = 0;
        while col < N {
            let mut row = 0;
            while row < M {
                transposed[row][col] = self.data[col][row];
                row += 1;
            }
            col += 1;
        }
        QGenericMatrix { data: transposed }
    }
}

impl<const N: usize, const M: usize> Default for QGenericMatrix<N, M> {
    /// Constructs a NxM identity matrix.
    fn default() -> Self {
        Self::identity()
    }
}

impl<const N: usize, const M: usize> std::ops::Index<(usize, usize)> for QGenericMatrix<N, M> {
    type Output = f32;

    /// Returns a reference to the element at position (row, column) in this matrix.
    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.data[column][row]
    }
}

impl<const N: usize, const M: usize> std::ops::IndexMut<(usize, usize)> for QGenericMatrix<N, M> {
    /// Returns a mutable reference to the element at position (row, column) in this matrix.
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        &mut self.data[column][row]
    }
}

impl<const N: usize, const M: usize> std::ops::AddAssign for QGenericMatrix<N, M> {
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.data_mut().iter_mut().zip(rhs.data()) {
            *lhs += rhs;
        }
    }
}

impl<const N: usize, const M: usize> std::ops::Add for QGenericMatrix<N, M> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const N: usize, const M: usize> std::ops::SubAssign for QGenericMatrix<N, M> {
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.data_mut().iter_mut().zip(rhs.data()) {
            *lhs -= rhs;
        }
    }
}

impl<const N: usize, const M: usize> std::ops::Sub for QGenericMatrix<N, M> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const N: usize, const M: usize> std::ops::MulAssign<f32> for QGenericMatrix<N, M> {
    fn mul_assign(&mut self, rhs: f32) {
        for value in self.data_mut() {
            *value *= rhs;
        }
    }
}

impl<const N: usize, const M: usize> std::ops::Mul<f32> for QGenericMatrix<N, M> {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const N: usize, const M: usize> std::ops::DivAssign<f32> for QGenericMatrix<N, M> {
    fn div_assign(&mut self, rhs: f32) {
        for value in self.data_mut() {
            *value /= rhs;
        }
    }
}

impl<const N: usize, const M: usize> std::ops::Div<f32> for QGenericMatrix<N, M> {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<const N: usize, const M: usize> std::ops::Neg for QGenericMatrix<N, M> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for value in self.data_mut() {
            *value = -*value;
        }
        self
    }
}

impl<const N: usize, const M: usize> TryFrom<&[f32]> for QGenericMatrix<N, M> {
    type Error = &'static str;

    /// Constructs a matrix from the given N * M floating-point `values`. The contents of the array `values` is assumed to be in row-major order.
    fn try_from(values: &[f32]) -> Result<Self, Self::Error> {
        if values.len() != M * N {
            return Err("invalid array length");
        }
        let mut matrix = [[0.0; M]; N];
        for (col, data) in matrix.iter_mut().enumerate() {
            for (row, value) in data.iter_mut().enumerate() {
                *value = values[row * N + col];
            }
        }
        Ok(Self { data: matrix })
    }
}

impl<const N: usize, const M: usize> From<&[[f32; N]; M]> for QGenericMatrix<N, M> {
    /// Constructs a matrix from the given N * M floating-point `values` in row-major order.
    fn from(values: &[[f32; N]; M]) -> Self {
        Self::new(values)
    }
}

impl<const N: usize, const M: usize> From<&QGenericMatrix<N, M>> for [[f32; N]; M] {
    /// Constructs a two-dimensional array from the matrix in row-major order.
    fn from(value: &QGenericMatrix<N, M>) -> Self {
        value.rows()
    }
}

macro_rules! impl_matrix {
    ($i:ident, $id:literal, $n:literal, $m:literal) => {
        pub type $i = QGenericMatrix<$n, $m>;
        // Safety:
        //
        // Static checks on the C++ side.
        unsafe impl ExternType for $i {
            type Id = type_id!($id);
            type Kind = cxx::kind::Trivial;
        }
    };
}

impl_matrix!(QMatrix2x2, "QMatrix2x2", 2, 2);
impl_matrix!(QMatrix2x3, "QMatrix2x3", 2, 3);
impl_matrix!(QMatrix2x4, "QMatrix2x4", 2, 4);
impl_matrix!(QMatrix3x2, "QMatrix3x2", 3, 2);
impl_matrix!(QMatrix3x3, "QMatrix3x3", 3, 3);
impl_matrix!(QMatrix3x4, "QMatrix3x4", 3, 4);
impl_matrix!(QMatrix4x2, "QMatrix4x2", 4, 2);
impl_matrix!(QMatrix4x3, "QMatrix4x3", 4, 3);

#[cfg(test)]
mod test {
    use super::*;

    #[rustfmt::skip]
    const MATRIX: &QGenericMatrix<4, 2> = &QGenericMatrix::new(&[
        [5.0, 4.0, 3.0, 2.0],
        [6.0, 7.0, 8.0, 9.0],
    ]);

    #[test]
    fn index() {
        assert_eq!(MATRIX[(1, 2)], 8.0);
    }

    #[test]
    fn data() {
        assert_eq!(MATRIX.data(), [5.0, 6.0, 4.0, 7.0, 3.0, 8.0, 2.0, 9.0]);
    }

    #[test]
    fn copy_data_to() {
        let mut dest = [0.0; 8];
        MATRIX.copy_data_to(&mut dest);
        assert_eq!(dest, [5.0, 4.0, 3.0, 2.0, 6.0, 7.0, 8.0, 9.0]);
    }

    #[test]
    fn fill() {
        let mut filled = *MATRIX;
        filled.fill(11.0);
        assert_eq!(filled.data(), [11.0; 8]);
    }

    #[test]
    fn filled() {
        let filled = QGenericMatrix::<4, 2>::filled(11.0);
        assert_eq!(filled.data(), [11.0; 8]);
    }

    #[test]
    fn identity() {
        let matrix = QGenericMatrix::<4, 2>::identity();
        assert_eq!(matrix.rows(), [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]]);
    }

    #[test]
    fn is_identity() {
        let matrix = QGenericMatrix::new(&[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]);
        assert!(matrix.is_identity());
    }

    #[test]
    fn is_not_identity() {
        let matrix = QGenericMatrix::new(&[
            [1.0, 1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]);
        assert!(!matrix.is_identity());
    }

    #[test]
    fn rows() {
        let rows = [[5.0, 4.0, 3.0, 2.0], [6.0, 7.0, 8.0, 9.0]];
        let matrix = QGenericMatrix::new(&rows);
        assert_eq!(matrix.rows(), rows);
    }

    #[test]
    fn set_to_identity() {
        let mut matrix = *MATRIX;
        matrix.set_to_identity();
        assert_eq!(matrix, QGenericMatrix::identity());
    }

    #[test]
    fn transposed() {
        let rows = MATRIX.transposed().rows();
        assert_eq!(rows, [[5.0, 6.0], [4.0, 7.0], [3.0, 8.0], [2.0, 9.0]]);
    }

    #[test]
    fn try_from_valid() {
        let matrix =
            QGenericMatrix::<4, 2>::try_from([5.0, 4.0, 3.0, 2.0, 6.0, 7.0, 8.0, 9.0].as_slice());
        assert_eq!(matrix, Ok(*MATRIX));
    }

    #[test]
    fn try_from_too_short() {
        let matrix =
            QGenericMatrix::<4, 2>::try_from([5.0, 4.0, 3.0, 2.0, 6.0, 7.0, 8.0].as_slice());
        matrix.expect_err("Expected error, got");
    }

    #[test]
    fn try_from_too_long() {
        let matrix = QGenericMatrix::<4, 2>::try_from(
            [5.0, 4.0, 3.0, 2.0, 6.0, 7.0, 8.0, 9.0, 1.0].as_slice(),
        );
        matrix.expect_err("Expected error, got");
    }
}
