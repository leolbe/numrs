use std::ops::{Index, IndexMut};

pub mod conversions;
pub mod iter;

/// A multidimensional array type with elements of type `T`.
#[derive(Clone)]
pub struct Array<T> {
    elements: Box<[T]>,
    shape: Box<[usize]>,
}

impl<T> Array<T> {
    /// The number of dimensions in the array.
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// The shape of the array, which determines the extent of each dimension.
    pub fn shape(&self) -> Array<usize> {
        Array {
            elements: self.shape.iter().rev().copied().collect(),
            shape: vec![self.rank()].into(),
        }
    }

    /// The total number of elements in the array.
    pub fn len(&self) -> usize {
        self.shape.iter().product()
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    /// Indexes an array by storage order, traversing dimensions in reverse order. For 2D arrays,
    /// this is the same as row-major order.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds, i.e. larger than or equal to the number of elements in
    /// the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use numrs::Array;
    ///
    /// let a: Array<usize> = Array::from([
    ///     [[0, 1, 2], [3, 4, 5]],
    ///     [[6, 7, 8], [9, 10, 11]]
    /// ]);
    ///
    /// assert_eq!(a[7], 7);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> IndexMut<usize> for Array<T> {
    /// Indexes an array by storage order, returning a mutable reference.
    ///
    /// See the [`Index`] impl.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}
