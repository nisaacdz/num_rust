use std::ops::Index;

use super::{indices::ColumnIndex, MatrixContent, MatrixIter};

pub struct MatrixRow<'a, T> {
    pub(crate) mat: &'a MatrixContent<T>,
    pub(crate) row: isize,
}

impl<'a, T> Index<ColumnIndex> for MatrixRow<'a, T> {
    type Output = T;

    fn index(&self, ColumnIndex(col): ColumnIndex) -> &Self::Output {
        self.mat.index((self.row, col))
    }
}

impl<'a, T> Index<isize> for MatrixRow<'a, T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.mat.index((self.row, index))
    }
}

impl<'a, T> IntoIterator for MatrixRow<'a, T> {
    type Item = &'a T;

    type IntoIter = MatrixIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let pos = (self.row * self.mat.dimension.width()) as usize;
        let end = pos + self.mat.dimension.width() as usize - 1;
        MatrixIter {
            step: 1,
            end,
            pos,
            mat: self.mat,
        }
    }
}
