use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    ptr::NonNull, fmt::Display,
};

pub mod columns;
pub mod indices;
pub mod rows;

use indices::Index as MatIndex;

use crate::dimension::Dimension;

use self::{
    columns::MatrixColumn,
    indices::{ColumnIndex, RowIndex},
    rows::MatrixRow,
};

pub struct Matrix<T> {
    pub content: MatrixContent<T>,
}

impl<T> Matrix<T> {
    pub fn from_content(content: MatrixContent<T>) -> Self {
        Self { content }
    }
    pub fn width(&self) -> isize {
        self.content.dimension.width()
    }

    pub fn height(&self) -> isize {
        self.content.dimension.height()
    }
}


impl<T: Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..self.height() {
            for col in 0..self.width() {
                let value = &self.content[(row, col)];
                output.push_str(&format!("{} ", value));
            }
            output.push('\n');
        }
        output.pop();
        write!(f, "[\n{}\n]", output)
    }
}


pub struct MatrixContent<T> {
    pub(crate) dimension: Dimension,
    pub(crate) buffer: Box<[T]>,
}

impl<T> MatrixContent<T> {
    pub fn new(dimension: Dimension, buffer: Vec<T>) -> Self {
        Self {
            dimension,
            buffer: buffer.into_boxed_slice(),
        }
    }

    fn reflect_row(&self, row: isize) -> isize {
        if row < 0 {
            self.dimension.height() + row
        } else {
            row
        }
    }

    fn reflect_col(&self, col: isize) -> isize {
        if col < 0 {
            self.dimension.width() + col
        } else {
            col
        }
    }

    fn reflect(&self, (row, col): (isize, isize)) -> (isize, isize) {
        (self.reflect_row(row), self.reflect_col(col))
    }
}

impl<T> Index<MatIndex> for MatrixContent<T> {
    type Output = T;

    fn index(&self, MatIndex(row, col): MatIndex) -> &Self::Output {
        self.index((row, col))
    }
}

impl<T> IndexMut<MatIndex> for MatrixContent<T> {
    fn index_mut(&mut self, MatIndex(row, col): MatIndex) -> &mut Self::Output {
        self.index_mut((row, col))
    }
}

impl<T> Index<(isize, isize)> for MatrixContent<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        let (row, col) = self.reflect(index);
        let index = col as usize + (row * self.dimension.width()) as usize;

        &self.buffer[index]
    }
}

impl<T> IndexMut<(isize, isize)> for MatrixContent<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        let (row, col) = self.reflect(index);
        let index = col as usize + (row * self.dimension.width()) as usize;

        &mut self.buffer[index]
    }
}

pub trait Get<'a, Idx> {
    type Output;
    fn get(&'a self, index: Idx) -> Option<Self::Output>;
}

pub trait GetMut<'a, Idx> {
    type Output;
    fn get_mut(&'a mut self, index: Idx) -> Option<Self::Output>;
}

impl<'a, T: 'a> Get<'a, RowIndex> for Matrix<T> {
    type Output = MatrixRow<'a, T>;

    fn get(&'a self, RowIndex(row): RowIndex) -> Option<Self::Output> {
        let row = self.content.reflect_row(row);
        if row < 0 || row >= self.height() {
            None
        } else {
            Some(MatrixRow {
                mat: &self.content,
                row,
            })
        }
    }
}

impl<'a, T: 'a> Get<'a, ColumnIndex> for Matrix<T> {
    type Output = MatrixColumn<'a, T>;

    fn get(&'a self, ColumnIndex(col): ColumnIndex) -> Option<Self::Output> {
        let col = self.content.reflect_row(col);
        if col < 0 || col >= self.width() {
            None
        } else {
            Some(MatrixColumn {
                mat: &self.content,
                col,
            })
        }
    }
}

#[derive(Clone)]
pub struct MatrixIter<'a, T> {
    mat: &'a MatrixContent<T>,
    pos: usize,
    step: usize,
    end: usize,
}

impl<'a, T> Iterator for MatrixIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos <= self.end {
            let index = self.pos;
            self.pos += self.step;
            Some(self.mat.buffer.index(index))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatrixIterMut<'a, T: 'a> {
    ptr: NonNull<T>,
    end: *mut T,
    step: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> MatrixIterMut<'a, T> {
    pub unsafe fn new(start: usize, end: usize, slice: &'a mut [T], step: usize) -> Self {
        assert!(start <= end);
        assert!(step > 0);
        let ptr = slice.as_mut_ptr().add(start);
        let end = slice.as_mut_ptr().add(end).add(step);
        Self {
            ptr: NonNull::new_unchecked(ptr),
            end,
            step,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for MatrixIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next_ptr = self.ptr.as_ptr().add(self.step);
            if next_ptr <= self.end {
                let curr_ptr = self.ptr.as_mut();
                self.ptr = NonNull::new_unchecked(next_ptr);
                Some(curr_ptr)
            } else {
                None
            }
        }
    }
}

#[macro_export]
macro_rules! matrix {
    [($rows:expr, $cols:expr), $($elem:expr),*] => {
        {
            use std::convert::TryInto;
            use num_rust::dimension::Dimension;
            use num_rust::matrices::{Matrix, MatrixContent};

            let dim = Dimension::new($rows.try_into().unwrap(), $cols.try_into().unwrap());
            let content_vec = vec![$($elem),*];
            let content = MatrixContent::new(dim, content_vec);
            Matrix::from_content(content)
        }
    }
}
