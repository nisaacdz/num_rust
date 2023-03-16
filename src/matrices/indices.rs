pub struct RowIndex(pub(crate) isize);

pub struct ColumnIndex(pub(crate) isize);

pub struct MatrixIndex(pub(crate) isize, pub(crate) isize);


impl From<isize> for RowIndex {
    fn from(value: isize) -> Self {
        Self(value)
    }
}

impl From<isize> for ColumnIndex {
    fn from(value: isize) -> Self {
        Self(value)
    }
}

