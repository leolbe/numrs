use crate::Array;

/// Types that can be used as an array shape.
pub trait Shape {
    /// Returns: A slice of dimension extents.
    fn shape(self) -> Box<[usize]>;
}

impl Shape for Box<[usize]> {
    fn shape(self) -> Box<[usize]> {
        self
    }
}

impl Shape for Array<usize> {
    fn shape(self) -> Box<[usize]> {
        self.elements
    }
}

impl<const N: usize> Shape for [usize; N] {
    fn shape(self) -> Box<[usize]> {
        self.into()
    }
}

impl Shape for &[usize] {
    fn shape(self) -> Box<[usize]> {
        self.into()
    }
}

impl Shape for usize {
    /// Creates a shape representing a 1D array with shape (`self`)
    fn shape(self) -> Box<[usize]> {
        vec![self].into_boxed_slice()
    }
}

/// Creates a shape representing a matrix with shape (`n`, `n`).
pub fn square(n: usize) -> impl Shape {
    vec![n, n].into_boxed_slice()
}

/// Creates a shape representing an array with rank `rank` and shape (`n`, `n`, ...)
pub fn hypercube(rank: usize, n: usize) -> impl Shape {
    vec![n; rank].into_boxed_slice()
}
