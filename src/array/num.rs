use crate::Array;
use crate::array::shape::{Shape, square};
use num_traits::{One, Zero};
use std::iter;

impl<T: Zero> Array<T> {
    /// Creates an array with a given shape filled with zeros.
    ///
    /// See [`num_traits::Zero`].
    pub fn zeros(shape: impl Into<Shape>) -> Array<T> {
        Array::filled_with(shape, T::zero)
    }

    /// Creates an array with a shape like `other` filled with zeros.
    ///
    /// See [`num_traits::Zero`].
    pub fn zeros_like<U>(other: &Array<U>) -> Array<T> {
        Array::zeros(other.shape.clone())
    }

    /// Creates an array with a given shape with diagonal elements given by `diagonal` and zeros on
    /// the off-diagonal. The diagonal elements are those elements where each dimension's index is
    /// equal, e.g. `(0, 0, ...), (1, 1, ...)` etc. The number of diagonal elements is given by the
    /// smallest dimension's extent.
    ///
    /// # Panics
    ///
    /// Panics if `diagonal` yields `None` after less than `n` iterations, where `n` is the smallest
    /// dimension's extent.
    pub fn from_diagonal(
        shape: impl Into<Shape>,
        diagonal: impl IntoIterator<Item = T>,
    ) -> Array<T> {
        let shape = shape.into();
        // the dimension with the lowest extent determines the number of diagonal elements
        let min_extent = shape.min_extent().unwrap_or(1);
        // the count of indices between each diagonal element
        let stride = shape.diag_stride().unwrap_or(0);

        let mut out = Array::zeros(shape);
        let mut elements = diagonal.into_iter();
        for i in 0..min_extent {
            let el = elements
                .next()
                .expect("diagonal iterator should yield an element for each diagonal index");
            out[i * stride] = el;
        }
        out
    }

    /// Creates an array with a shape like `other` with diagonal elements given by `diagonal` and
    /// zeros on the off-diagonal.
    ///
    /// See [`Array::from_diagonal`].
    ///
    /// # Panics
    ///
    /// Panics if `diagonal` yields `None` after less than `n` iterations, where `n` is `other`'s
    /// smallest dimension's extent.
    pub fn from_diagonal_like<U>(
        other: &Array<U>,
        diagonal: impl IntoIterator<Item = T>,
    ) -> Array<T> {
        Array::from_diagonal(other.shape.clone(), diagonal)
    }
}

impl<T: One> Array<T> {
    /// Creates an array with the given shape filled with ones.
    ///
    /// See [`num_traits::One`].
    pub fn ones(shape: impl Into<Shape>) -> Array<T> {
        Array::filled_with(shape, T::one)
    }

    /// Creates an array with a shape like `other` filled with zeros.
    ///
    /// See [`num_traits::One`].
    pub fn ones_like<U>(other: &Array<U>) -> Array<T> {
        Array::ones(other.shape.clone())
    }
}

impl<T: Zero + One> Array<T> {
    /// Creates a square matrix array with shape `(n, n)` with ones along the diagonal and zeros on
    /// the off-diagonal.
    ///
    /// See [`num_traits::Zero`], [`num_traits::One`].
    pub fn identity(n: usize) -> Array<T> {
        Array::from_diagonal(square(n), iter::repeat_with(T::one))
    }
}
