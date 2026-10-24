use crate::array::Array;
use std::mem::MaybeUninit;

#[derive(Clone, Debug, PartialEq)]
pub struct ConvertError;

impl<T> Array<T> {
    pub fn into_scalar(self) -> Result<T, ConvertError> {
        if self.elements.len() == 1 {
            let scalar = self
                .elements
                .into_iter()
                .next()
                .expect("array should not be empty");
            Ok(scalar)
        } else {
            Err(ConvertError)
        }
    }
}

impl<T> Array<MaybeUninit<T>> {
    /// Converts to an `Array<T>`.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that each element in the array is initialized. Performing this
    /// conversion when any element is uninitialized constitutes undefined behavior, even when `T`
    /// is a type that is valid for any bit pattern, such as an integer type.
    ///
    /// See [`MaybeUninit`].
    pub unsafe fn assume_init(self) -> Array<T> {
        let elements = unsafe { self.elements.assume_init() };
        Array {
            elements,
            shape: self.shape,
        }
    }
}

impl<T> From<T> for Array<T> {
    /// Converts `value` to a scalar (0D) array.
    fn from(value: T) -> Self {
        Self {
            elements: vec![value].into(),
            shape: vec![].into(),
        }
    }
}

impl<T> TryFrom<Array<T>> for Box<[T]> {
    type Error = ConvertError;

    /// Converts a 1D array to a boxed slice.
    ///
    /// # Arguments
    ///
    /// * `value`: Must be either a scalar (0D array), a 1D array, or a higher rank array where the
    ///            rest of the dimensions' extents are invariably 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use numrs::Array;
    /// use std::assert_matches;
    ///
    /// // ok!
    ///
    /// // shape ()
    /// assert_eq!(
    ///     Box::<[i32]>::try_from(array![5_i32]),
    ///     Ok(vec![5].into())
    /// );
    /// // shape (1, 3)
    /// assert_eq!(
    ///     Box::<[i32]>::try_from(array![4, 5, 6;]),
    ///     Ok(vec![4, 5, 6].into())
    /// );
    /// // shape (3, 1)
    /// assert_eq!(
    ///     Box::<[i32]>::try_from(array![7; 8; 9]),
    ///     Ok(vec![7, 8, 9].into())
    /// );
    ///
    /// // error!
    ///
    /// // shape (2, 2)
    /// assert_matches!(
    ///     Box::<[i32]>::try_from(array![1, 2; 3, 4]),
    ///     Err(_)
    /// );
    /// ```
    fn try_from(value: Array<T>) -> Result<Self, Self::Error> {
        value.get_1d_extent().ok_or(ConvertError)?;
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

macro_rules! array_type {
    () => { T };
    ($n:ident, $($rest:ident,)*) => {
        [array_type!($($rest,)*); $n]
    }
}

macro_rules! impl_from_std_array {
    (@ $value:ident, $n:ident,) => {
        $value.into_iter()
    };
    (@ $value:ident, $n:ident, $($rest:ident,)*) => {
        impl_from_std_array!(@ $value, $($rest,)*).flatten()
    };
    ($($n:ident),*) => {
        impl<T, $(const $n: usize),*> From<array_type!($($n,)*)> for Array<T> {
            /// Converts an std array into a multidimensional array type.
            ///
            /// # Examples
            ///
            /// ```
            /// use numrs::Array;
            ///
            /// let a = Array::from([[1, 2, 3], [4, 5, 6]]);
            ///
            /// assert_eq!(6, a[[1, 2]]);
            /// ```
            fn from(value: array_type!($($n,)*)) -> Self {
                Array {
                    elements: impl_from_std_array!(@ value, $($n,)*).collect(),
                    shape: [$($n,)*].into(),
                }
            }
        }
    };
}

impl_from_std_array!(N1);
impl_from_std_array!(N1, N2);
impl_from_std_array!(N1, N2, N3);
impl_from_std_array!(N1, N2, N3, N4);
impl_from_std_array!(N1, N2, N3, N4, N5);
impl_from_std_array!(N1, N2, N3, N4, N5, N6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_boxed() {
        assert_eq!(Box::<[i32]>::try_from(Array::from(5)), Ok(vec![5].into()));
        assert_eq!(
            Box::<[i32]>::try_from(<Array::<i32> as From<[i32; 0]>>::from([])),
            Ok(vec![].into())
        );
        assert_eq!(
            Box::<[i32]>::try_from(<Array::<i32> as From<[[i32; 0]; 1]>>::from([[]])),
            Ok(vec![].into())
        );
        assert_eq!(
            Box::<[i32]>::try_from(Array::from([1, 2, 3])),
            Ok(vec![1, 2, 3].into())
        );
        assert_eq!(
            Box::<[i32]>::try_from(<Array::<i32> as From<[[i32; 3]; 1]>>::from([[4, 5, 6]])),
            Ok(vec![4, 5, 6].into())
        );
        assert_eq!(
            Box::<[i32]>::try_from(<Array::<i32> as From<[[i32; 1]; 3]>>::from([[7], [8], [9]])),
            Ok(vec![7, 8, 9].into())
        );
        assert_eq!(
            Box::<[i32]>::try_from(<Array::<i32> as From<[[i32; 0]; 2]>>::from([[], []])),
            Err(ConvertError)
        );
        assert_eq!(
            Box::<[i32]>::try_from(<Array::<i32> as From<[[i32; 2]; 2]>>::from([
                [1, 2],
                [3, 4]
            ])),
            Err(ConvertError)
        );
    }
}
