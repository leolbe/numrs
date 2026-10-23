use std::ops::{Index, IndexMut};

mod conversions;

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

    /// Returns the extent of the array's lowest dimension, unless the array is multidimensional.
    ///
    /// If the array is of shape (), i.e. is scalar, returns `Some(1)`
    /// If the array is of shape (1, 1, ..., n), returns `Some(n)`
    /// If any dimension is of extent 0, e.g. (1, 0, n), returns `Some(0)`
    /// Otherwise, returns `None`
    pub fn get_1d_extent(&self) -> Option<usize> {
        // for each index except the first
        for x in self.shape.iter().skip(1) {
            match x {
                // if 0, then the whole len is 0 anyway
                0 => return Some(0),
                1 => {}
                // if > 1, then the array is not 1D
                _ => return None,
            }
        }
        // return extent of lowest dimension, or `Some(1)` if array is scalar
        self.shape.get(0).copied().or(Some(1))
    }
}

impl<T> TryFrom<Array<T>> for Box<[T]> {
    type Error = ();

    /// Converts the array to a boxed slice, failing if the array is not 1D.
    fn try_from(value: Array<T>) -> Result<Self, Self::Error> {
        value.get_1d_extent().ok_or(())?;
        Ok(value.elements)
    }
}

impl<T> From<Box<[T]>> for Array<T> {
    /// Converts a boxed slice to a 1D array.
    fn from(value: Box<[T]>) -> Self {
        let extent = value.len();
        Self {
            elements: value,
            shape: vec![extent].into(),
        }
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
