use crate::Array;
use std::{slice, vec};

/// Represents the shape of an array, determining its rank and the extent of each dimension.
pub struct Shape {
    pub(super) extents: Box<[usize]>,
}

impl Shape {
    /// Creates an iterator that yields references to each dimension's extent.
    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.into_iter()
    }

    /// Creates an iterator that yields mutable references to each dimension's extent.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut usize> {
        self.into_iter()
    }

    /// The number of dimensions.
    pub fn rank(&self) -> usize {
        self.extents.len()
    }

    /// The total number of elements in an array of this shape.
    pub fn len(&self) -> usize {
        self.extents.iter().product()
    }

    /// The extent of this shape's smallest dimension, if the shape has more than one dimension.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use numrs::array::shape::Shape;
    ///
    /// let shape: Shape = [5, 3, 2].into();
    /// assert_eq!(Some(2), shape.min_extent());
    /// ```
    pub fn min_extent(&self) -> Option<usize> {
        self.iter().min().copied()
    }

    /// The extent of this shape's largest dimension, if the shape has more than one dimension.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use numrs::array::shape::Shape;
    ///
    /// let shape: Shape = [5, 3, 2].into();
    /// assert_eq!(Some(5), shape.max_extent());
    /// ```
    pub fn max_extent(&self) -> Option<usize> {
        self.iter().max().copied()
    }

    /// The stride between the index of each diagonal element, i.e. the difference of their flat
    /// indices.
    pub(super) fn diag_stride(&self) -> Option<usize> {
        // e.g. for shape (l, m, n):
        // index 0 has stride m * n
        // index 1 has stride n
        // index 2 has stride 1
        //
        // each diagonal stride is at an index offset of
        // 1 + n + m * n = 1 + n * (1 + m)
        // more generally,
        // stride = 1 + n_n-1 + n_n-1 * n_n-2 + n_n-1 * n_n-2 + n_n-3 + ...
        //        = 1 + n_n-1 * (1 + n_n-2 * (1 + n_n-3 * (...)))
        self.extents
            .iter()
            .copied()
            .reduce(|n0, n1| n1 * (1 + n0))
            .map(|s| s + 1)
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
    vec![n; rank].into_boxed_slice().into()
}
