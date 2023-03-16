use std::ops::{Add, AddAssign, Mul, MulAssign};

use super::{GenericMatrix, Matrix, MatrixContent};

/// SCALAR MULTIPLICATION

impl<T: Clone + MulAssign> Mul<T> for MatrixContent<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.entries_mut().for_each(|v| *v *= rhs.clone());
        self
    }
}

impl<T: Clone + MulAssign> Mul<T> for Matrix<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.entries_mut().for_each(|v| *v *= rhs.clone());
        self
    }
}

impl Mul<f64> for GenericMatrix {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.entries_mut().for_each(|v| *v *= rhs.clone());
        self
    }
}

impl<T: Clone + MulAssign<T>> MulAssign<T> for MatrixContent<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.entries_mut().for_each(|v| *v *= rhs.clone());
    }
}

impl<T: Clone + MulAssign<T>> MulAssign<T> for Matrix<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.content.mul_assign(rhs)
    }
}

impl MulAssign<f64> for GenericMatrix {
    fn mul_assign(&mut self, rhs: f64) {
        self.content.mul_assign(rhs)
    }
}

impl<'a, T: Clone + AddAssign<&'a T>> Add<&'a MatrixContent<T>> for &MatrixContent<T> {
    type Output = MatrixContent<T>;

    fn add(self, rhs: &'a MatrixContent<T>) -> Self::Output {
        if self.dimension != rhs.dimension {
            panic!("Dimensions must be equal in order to add two matrices: Use plus method to obtain option and avoid panic");
        }

        let mut content = MatrixContent::clone(self);
        let mut iter = rhs.entries();

        content
            .entries_mut()
            .for_each(|v| *v += iter.next().unwrap());

        content
    }
}

impl<'a, T: Clone + AddAssign<&'a T>> Add<&'a Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &'a Matrix<T>) -> Self::Output {
        Matrix {
            content: self.content.add(&rhs.content),
        }
    }
}

impl<'a> Add<&'a GenericMatrix> for &GenericMatrix {
    type Output = GenericMatrix;

    fn add(self, rhs: &'a GenericMatrix) -> Self::Output {
        GenericMatrix {
            content: self.content.add(&rhs.content),
        }
    }
}
