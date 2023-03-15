use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
    ptr::NonNull,
};

use crate::dimension::Dimension;
pub mod indices;
pub mod rows;
pub mod columns;

use indices::Index as MatIndex;

use self::{
    indices::{ColumnIndex, RowIndex},
    rows::MatrixRow,
};

pub struct Matrix<T> {
    pub content: MatrixContent<T>,
    pub width: isize,
    pub height: isize,
}

impl<T> Matrix<T> {
    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }
}

pub struct MatrixContent<T> {
    pub(crate) dimension: Dimension,
    pub(crate) buffer: Box<[T]>,
}

impl<T> MatrixContent<T> {
    fn reflect_row(&self, row: isize) -> isize {
        (if row < 0 {
            self.dimension.height() + row
        } else {
            row
        })
    }

    fn reflect_col(&self, col: isize) -> isize {
        (if col < 0 {
            self.dimension.width() + col
        } else {
            col
        })
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
    pub(crate) unsafe fn new(start: usize, end: usize, slice: &'a mut [T], step: usize) -> Self {
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
