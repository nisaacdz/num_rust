use std::ops::Index;

use super::{indices::RowIndex, MatrixContent, MatrixIter};

pub struct MatrixColumn<'a, T> {
    pub(crate) mat: &'a MatrixContent<T>,
    pub(crate) col: isize,
}

impl<'a, T> Index<RowIndex> for MatrixColumn<'a, T> {
    type Output = T;

    fn index(&self, RowIndex(row): RowIndex) -> &Self::Output {
        self.mat.index((row, self.col))
    }
}

impl<'a, T> Index<isize> for MatrixColumn<'a, T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        self.mat.index((index, self.col))
    }
}

impl<'a, T> IntoIterator for MatrixColumn<'a, T> {
    type Item = &'a T;

    type IntoIter = MatrixIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let pos = self.col;
        let end = self.col + (self.mat.dimension.height() - 1) * self.mat.dimension.width();
        MatrixIter {
            step: self.mat.dimension.width() as usize,
            end: end as usize,
            pos: pos as usize,
            mat: self.mat,
        }
    }
}
