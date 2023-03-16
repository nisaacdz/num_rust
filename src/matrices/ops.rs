use std::ops::Add;

use super::Matrix;

impl<T> Add<&Matrix<T>> for &Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, _rhs: &Matrix<T>) -> Self::Output {
        todo!()
    }
}
