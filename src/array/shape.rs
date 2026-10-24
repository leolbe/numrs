use crate::Array;
use std::{slice, vec};

/// Represents the shape of an array, determining its rank and the extent of each dimension.
pub struct Shape {
    pub(super) extents: Box<[usize]>,
}

impl Shape {
    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut usize> {
        self.into_iter()
    }
}

impl IntoIterator for Shape {
    type Item = usize;
    type IntoIter = vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.extents.into_iter()
    }
}

impl<'a> IntoIterator for &'a Shape {
    type Item = &'a usize;
    type IntoIter = slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.extents.iter()
    }
}

impl<'a> IntoIterator for &'a mut Shape {
    type Item = &'a mut usize;
    type IntoIter = slice::IterMut<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.extents.iter_mut()
    }
}

impl From<Box<[usize]>> for Shape {
    fn from(value: Box<[usize]>) -> Self {
        Shape { extents: value }
    }
}

impl From<Array<usize>> for Shape {
    fn from(value: Array<usize>) -> Self {
        Shape {
            extents: value.elements,
        }
    }
}

impl<const N: usize> From<[usize; N]> for Shape {
    fn from(value: [usize; N]) -> Self {
        Shape {
            extents: value.into(),
        }
    }
}

impl From<&[usize]> for Shape {
    fn from(value: &[usize]) -> Self {
        Shape {
            extents: value.into(),
        }
    }
}

impl From<usize> for Shape {
    fn from(value: usize) -> Self {
        Shape {
            extents: vec![value].into(),
        }
    }
}

/// Creates a shape representing a matrix with shape (`n`, `n`).
pub fn square(n: usize) -> Shape {
    [n, n].into()
}

/// Creates a shape representing an array with rank `rank` and shape (`n`, `n`, ...)
pub fn hypercube(rank: usize, n: usize) -> Shape {
    vec![n; rank].into()
}
