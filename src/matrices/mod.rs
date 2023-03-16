pub mod columns;
pub mod indices;
mod ops;
mod macros;
pub mod rows;

use std::ops::{Index, IndexMut};

use indices::MatrixIndex;

use crate::dimension::Dimension;
pub use ops::*;
pub use macros::*;

use self::{
    columns::{ColumnsIter, MatrixColumn},
    indices::{ColumnIndex, RowIndex},
    rows::{MatrixRow, RowsIter},
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

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.content, f)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.content, f)
    }
}

impl<T> std::ops::Deref for Matrix<T> {
    type Target = MatrixContent<T>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T> std::ops::DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Matrix<T> {
    type Error = MisAlignment;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        let width = value[0].len();
        let height = value.len();
        for v in value {
            if v.len() != width {
                return Err(MisAlignment);
            }
            vec.extend(v.into_iter())
        }

        Ok(Matrix {
            content: MatrixContent::new(Dimension::new(width as isize, height as isize), vec),
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
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

    pub fn entries(&self) -> MatrixEntries<T> {
        MatrixEntries { mat: self, pos: 0 }
    }

    pub fn entries_mut(&mut self) -> MatrixIterMut<T> {
        unsafe { MatrixIterMut::new(0, self.buffer.len() - 1, &mut self.buffer, 1) }
    }

    pub fn rows(&self) -> RowsIter<T> {
        RowsIter { mat: self, pos: 0 }
    }

    pub fn columns(&self) -> ColumnsIter<T> {
        ColumnsIter { mat: self, pos: 0 }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for MatrixContent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..self.dimension.height() {
            for col in 0..self.dimension.width() {
                output.push_str(&format!("{}, ", self[(row, col)]));
            }
            output.pop();
            output.pop();
            output.push('\n');
        }
        output.pop();
        write!(f, "{}", output)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for MatrixContent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..self.dimension.height() {
            for col in 0..self.dimension.width() {
                output.push_str(&format!("{:?}, ", self[(row, col)]));
            }
            output.pop();
            output.pop();
            output.push('\n');
        }
        output.pop();
        write!(f, "{}", output)
    }
}

impl<T> Index<MatrixIndex> for MatrixContent<T> {
    type Output = T;

    fn index(&self, MatrixIndex(row, col): MatrixIndex) -> &Self::Output {
        self.index((row, col))
    }
}

impl<T> IndexMut<MatrixIndex> for MatrixContent<T> {
    fn index_mut(&mut self, MatrixIndex(row, col): MatrixIndex) -> &mut Self::Output {
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

impl<T> Index<isize> for MatrixContent<T> {
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let index = if index < 0 {
            self.dimension.len() + index
        } else {
            index
        };

        self.buffer.index(index as usize)
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

impl<'a, T: 'a> Get<'a, RowIndex> for MatrixContent<T> {
    type Output = MatrixRow<'a, T>;

    fn get(&'a self, RowIndex(row): RowIndex) -> Option<Self::Output> {
        let row = self.reflect_row(row);
        if row < 0 || row >= self.dimension.height() {
            None
        } else {
            Some(MatrixRow { mat: self, row })
        }
    }
}

impl<'a, T: 'a> Get<'a, ColumnIndex> for MatrixContent<T> {
    type Output = MatrixColumn<'a, T>;

    fn get(&'a self, ColumnIndex(col): ColumnIndex) -> Option<Self::Output> {
        let col = self.reflect_row(col);
        if col < 0 || col >= self.dimension.width() {
            None
        } else {
            Some(MatrixColumn { mat: self, col })
        }
    }
}

impl<'a, T: 'a> Get<'a, MatrixIndex> for MatrixContent<T> {
    type Output = &'a T;
    fn get(&'a self, MatrixIndex(row, col): MatrixIndex) -> Option<Self::Output> {
        let (row, col) = self.reflect((row, col));
        if row < 0 || row >= self.dimension.height() {
            None
        } else if col < 0 || col >= self.dimension.width() {
            None
        } else {
            Some(self.index((row, col)))
        }
    }
}

impl<'a, T: 'a> GetMut<'a, MatrixIndex> for MatrixContent<T> {
    type Output = &'a mut T;
    fn get_mut(&'a mut self, MatrixIndex(row, col): MatrixIndex) -> Option<Self::Output> {
        let (row, col) = self.reflect((row, col));
        if row < 0 || row >= self.dimension.height() {
            None
        } else if col < 0 || col >= self.dimension.width() {
            None
        } else {
            Some(self.index_mut((row, col)))
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MisAlignment;

impl<T> TryFrom<Vec<Vec<T>>> for MatrixContent<T> {
    type Error = MisAlignment;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        let width = value[0].len();
        let height = value.len();
        for v in value {
            if v.len() != width {
                return Err(MisAlignment);
            }
            vec.extend(v.into_iter())
        }

        Ok(Self::new(
            Dimension::new(width as isize, height as isize),
            vec,
        ))
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

pub struct MatrixEntries<'a, T> {
    mat: &'a MatrixContent<T>,
    pos: usize,
}

impl<'a, T> Iterator for MatrixEntries<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.mat.buffer.len() {
            let pos = self.pos;
            self.pos += 1;
            Some(self.mat.buffer.index(pos))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatrixIterMut<'a, T: 'a> {
    ptr: std::ptr::NonNull<T>,
    end: *mut T,
    step: usize,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> MatrixIterMut<'a, T> {
    /// Creates a mutable iterator over the elements in a mutable slice from start to end inclusive
    /// and with the steps indicated
    pub unsafe fn new(start: usize, end: usize, slice: &'a mut [T], step: usize) -> Self {
        assert!(start <= end);
        assert!(step > 0);
        let ptr = slice.as_mut_ptr().add(start);
        let end = slice.as_mut_ptr().add(end).add(step);
        Self {
            ptr: std::ptr::NonNull::new_unchecked(ptr),
            end,
            step,
            _marker: std::marker::PhantomData,
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
                self.ptr = std::ptr::NonNull::new_unchecked(next_ptr);
                Some(curr_ptr)
            } else {
                None
            }
        }
    }
}

#[derive(Clone)]
pub struct GenericMatrix {
    pub(crate) content: MatrixContent<f64>,
}

impl GenericMatrix {
    pub fn from_content(content: MatrixContent<f64>) -> Self {
        Self { content }
    }
    pub fn width(&self) -> isize {
        self.content.dimension.width()
    }

    pub fn height(&self) -> isize {
        self.content.dimension.height()
    }

    pub fn rows(&self) -> RowsIter<f64> {
        RowsIter {
            mat: &self.content,
            pos: 0,
        }
    }

    pub fn columns(&self) -> ColumnsIter<f64> {
        ColumnsIter {
            mat: &self.content,
            pos: 0,
        }
    }
}

impl std::fmt::Display for GenericMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.content, f)
    }
}

impl std::fmt::Debug for GenericMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.content, f)
    }
}

impl std::ops::Deref for GenericMatrix {
    type Target = MatrixContent<f64>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl std::ops::DerefMut for GenericMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

impl TryFrom<Vec<Vec<f64>>> for GenericMatrix {
    type Error = MisAlignment;

    fn try_from(value: Vec<Vec<f64>>) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        let width = value[0].len();
        let height = value.len();
        for v in value {
            if v.len() != width {
                return Err(MisAlignment);
            }
            vec.extend(v.into_iter())
        }

        Ok(GenericMatrix {
            content: MatrixContent::new(Dimension::new(width as isize, height as isize), vec),
        })
    }
}

