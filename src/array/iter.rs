use crate::Array;
use std::iter::FusedIterator;

impl<T> Array<T> {
    /// Creates an iterator that yields references to values in the array.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.into_iter()
    }

    /// Creates an iterator that yields mutable references to values in the array.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.into_iter()
    }

    /// Creates a consuming iterator that yields elements in the array together with their
    /// corresponding multidimensional indices.
    ///
    /// This is somewhat the multidimensional counterpart to [`std::iter::Enumerate`].
    pub fn into_indexed_iter(self) -> impl Iterator<Item = (Array<usize>, T)> {
        IndexIter::new_inner(self.shape).zip(self.elements.into_iter())
    }

    /// Creates an iterator that yields references to elements in the array together with their
    /// corresponding multidimensional indices.
    ///
    /// This is somewhat the multidimensional counterpart to [`std::iter::Enumerate`].
    pub fn indexed_iter(&self) -> impl Iterator<Item = (Array<usize>, &T)> {
        IndexIter::new_like(self).zip(self.elements.iter())
    }

    /// Creates an iterator that yields mutable references to elements in the array together with
    /// their corresponding multidimensional indices.
    ///
    /// This is somewhat the multidimensional counterpart to [`std::iter::Enumerate`].
    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (Array<usize>, &mut T)> {
        IndexIter::new_like(self).zip(self.elements.iter_mut())
    }
}

impl<T> IntoIterator for Array<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// Creates a consuming iterator that yields elements in the array.
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    /// Creates an iterator that yields references to values in the array.
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Array<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    /// Creates an iterator that yields mutable references to values in the array.
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

/// Iterator that yields multidimensional indices in accordance with a given array shape. Iterates
/// indices by storage order, traversing dimensions from last to first. For 2D arrays, this is the
/// same as row-major order.
#[must_use]
#[derive(Clone)]
pub struct IndexIter {
    shape: Box<[usize]>,
    indices: Box<[usize]>,
    total_len: usize,
    flat_index: usize,
}

impl IndexIter {
    /// Creates an iterator that yields indices of the given shape.
    ///
    /// # Arguments
    ///
    /// * `shape`: Must be a 1D array or an array where at most one dimension has an extent not
    ///            equal to 1. See [`Array::get_1d_extent`]
    ///
    /// # Panics
    ///
    /// Panics if `shape` has an invalid shape.
    pub fn new(shape: Array<usize>) -> IndexIter {
        IndexIter::new_inner(Box::try_from(shape).expect("shape should be a 1D array"))
    }

    /// Create an index iterator with the same shape as `array`.
    pub fn new_like<T>(array: &Array<T>) -> IndexIter {
        IndexIter::new_inner(array.shape.clone())
    }

    fn new_inner(shape: Box<[usize]>) -> IndexIter {
        let rank = shape.len();
        let total_len = shape.iter().product();
        Self {
            shape,
            indices: vec![rank; 0].into(),
            total_len,
            flat_index: 0,
        }
    }

    fn advance_indices(&mut self, n: usize) {
        let mut carry = n;
        // iterate through indices and carry over when indices overflow.
        // e.g. if n = 1, for shape (5, 3, 4), index (2, 4, 3) is increased to (3, 0, 0)
        for i in (0..self.shape.len()).rev() {
            self.indices[i] += carry;
            carry = self.indices[i] / self.shape[i];
            self.indices[i] %= self.shape[i];
            if carry == 0 {
                break;
            }
        }
        self.flat_index += n;
    }
}

impl Iterator for IndexIter {
    type Item = Array<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.flat_index < self.total_len {
            let next = self.indices.clone();
            self.advance_indices(1);
            Some(Array::from(next))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn last(self) -> Option<Self::Item> {
        if self.flat_index < self.total_len {
            // if one dimension is of size 0, this iterator will never return anything
            None
        } else {
            // for shape (5, 4, 3), for example, the last indices would be (4, 3, 2)
            let mut last_indices = self.shape;
            // the if condition guarantees that there is no integer underflow here
            last_indices.iter_mut().for_each(|x| *x -= 1);
            Some(last_indices.into())
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.advance_indices(n);
        self.next()
    }
}

impl ExactSizeIterator for IndexIter {
    fn len(&self) -> usize {
        self.total_len - self.flat_index
    }
}

impl FusedIterator for IndexIter {}
