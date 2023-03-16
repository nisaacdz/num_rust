use std::ops::Index;

use super::{indices::RowIndex, MatrixContent, MatrixIter};

pub struct MatrixColumn<'a, T> {
    pub(crate) mat: &'a MatrixContent<T>,
    pub(crate) col: isize,
}

impl<'a, T> MatrixColumn<'a, T> {
    pub fn iter(&'a self) -> MatrixIter<'a, T> {
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

impl<'a, T: std::fmt::Display> std::fmt::Display for MatrixColumn<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("[");
        for item in self.iter() {
            output.push_str(&item.to_string());
            output.push_str(", ");
        }
        output.pop();
        output.pop();
        output.push(']');

        write!(f, "{}", output)
    }
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for MatrixColumn<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("[");
        for item in self.iter() {
            output.push_str(&format!("{:?}", item));
            output.push_str(", ");
        }
        output.pop();
        output.pop();
        output.push(']');

        write!(f, "{}", output)
    }
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

pub struct ColumnsIter<'a, T> {
    pub(crate) mat: &'a MatrixContent<T>,
    pub(crate) pos: isize,
}

impl<'a, T> Iterator for ColumnsIter<'a, T> {
    type Item = MatrixColumn<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.mat.dimension.width() {
            let pos = self.pos;
            self.pos += 1;
            Some(MatrixColumn {
                mat: self.mat,
                col: pos,
            })
        } else {
            None
        }
    }
}
