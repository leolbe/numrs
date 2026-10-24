trait Sealed {}

/// Types that can be used as an array shape.
pub trait Shape: Into<Box<[usize]>> + Sealed {}

impl<T: Into<Box<[usize]>>> Sealed for T {}
impl<T: Into<Box<[usize]>>> Shape for T {}
