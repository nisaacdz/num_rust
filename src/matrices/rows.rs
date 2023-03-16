use super::{indices::ColumnIndex, MatrixContent, MatrixIter};

pub struct MatrixRow<'a, T> {
    pub(crate) mat: &'a MatrixContent<T>,
    pub(crate) row: isize,
}

impl<'a, T> MatrixRow<'a, T> {
    pub fn iter(&'a self) -> MatrixIter<'a, T> {
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

impl<'a, T: std::fmt::Display> std::fmt::Display for MatrixRow<'a, T> {
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

impl<'a, T: std::fmt::Debug> std::fmt::Debug for MatrixRow<'a, T> {
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

impl<'a, T> std::ops::Index<ColumnIndex> for MatrixRow<'a, T> {
    type Output = T;

    fn index(&self, ColumnIndex(col): ColumnIndex) -> &Self::Output {
        self.mat.index((self.row, col))
    }
}

impl<'a, T> std::ops::Index<isize> for MatrixRow<'a, T> {
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

pub struct RowsIter<'a, T> {
    pub(crate) mat: &'a MatrixContent<T>,
    pub(crate) pos: isize,
}

impl<'a, T> Iterator for RowsIter<'a, T> {
    type Item = MatrixRow<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.mat.dimension.height() {
            let pos = self.pos;
            self.pos += 1;
            Some(MatrixRow {
                mat: self.mat,
                row: pos,
            })
        } else {
            None
        }
    }
}
