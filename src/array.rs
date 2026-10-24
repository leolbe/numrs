use crate::array::iter::IndexIter;
use crate::array::shape::Shape;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

pub mod conversions;
pub mod iter;
#[cfg(feature = "num")]
pub mod num;
pub mod shape;

/// A multidimensional array type with elements of type `T`.
#[derive(Clone)]
pub struct Array<T> {
    elements: Box<[T]>,
    shape: Shape,
}

impl<T> Array<T> {
    /// The number of dimensions in the array.
    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    /// The shape of the array, which determines the extent of each dimension.
    pub fn shape(&self) -> Array<usize> {
        self.shape.extents.clone().into()
    }

    /// The total number of elements in the array.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Creates an array with a given shape where the elements are determined by their indices using
    /// `f`.
    pub fn from_fn<F: FnMut(Array<usize>) -> T>(shape: impl Into<Shape>, f: F) -> Array<T> {
        let shape = shape.into();

        let mut indices = IndexIter::new_inner(shape);
        let elements: Box<[T]> = indices.by_ref().map(f).collect();

        // move shape back out of iterator to avoid cloning
        let shape = indices.into_shape();
        Array::from_elements(elements, shape)
    }

    /// Creates an array with a given shape where the elements are given by calling `f`.
    pub fn filled_with<F: FnMut() -> T>(shape: impl Into<Shape>, mut f: F) -> Array<T> {
        let shape = shape.into();
        let len = shape.iter().product();

        let elements: Box<[T]> = (0..len).by_ref().map(|_| f()).collect();
        Array::from_elements(elements, shape.extents)
    }

    /// Creates an array with a shape like `other` and where the elements are given by calling `f`.
    pub fn filled_with_like<U, F: FnMut() -> T>(other: &Array<U>, f: F) -> Array<T> {
        Array::filled_with(other.shape.clone(), f)
    }

    /// Creates an array of uninitialized values with a given shape.
    pub fn uninit(shape: impl Into<Shape>) -> Array<MaybeUninit<T>> {
        Array::filled_with(shape, || MaybeUninit::uninit())
    }

    /// Creates an array of uninitialized values with a shape like `other`.
    pub fn uninit_like<U>(other: &Array<U>) -> Array<MaybeUninit<T>> {
        Array::uninit(other.shape.clone())
    }

    fn from_elements(elements: Box<[T]>, shape: impl Into<Shape>) -> Array<T> {
        Array {
            elements,
            shape: shape.into(),
        }
    }
}

impl<T: Clone> Array<T> {
    /// Creates an array with a given shape filled with `value`.
    pub fn filled(shape: impl Into<Shape>, value: &T) -> Array<T> {
        Array::filled_with(shape, || value.clone())
    }

    /// Creates an array with a shape like `other` filled with `value`.
    pub fn filled_like<U>(other: &Array<U>, value: &T) -> Array<T> {
        Array::filled(other.shape.clone(), value)
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
