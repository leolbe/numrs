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
        Array::from_elements(elements, self.shape)
    }
}

impl<T> From<T> for Array<T> {
    /// Converts `value` to a scalar (0D) array.
    fn from(value: T) -> Self {
        Array::from_elements(vec![value].into(), [])
    }
}

impl<T> From<Array<T>> for Box<[T]> {
    /// Flattens an array into a 1D boxed slice, ordering the elements by storage order.
    /// (See [`array`](crate::array))
    fn from(value: Array<T>) -> Self {
        value.elements
    }
}

impl<T> From<Box<[T]>> for Array<T> {
    /// Converts a boxed slice to a 1D array.
    fn from(value: Box<[T]>) -> Self {
        let len = value.len();
        Array::from_elements(value, len)
    }
}

impl<T> FromIterator<T> for Array<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let elements: Box<[T]> = FromIterator::from_iter(iter);
        elements.into()
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
                let elements: Box<[T]> = impl_from_std_array!(@ value, $($n,)*).collect();
                Array::from_elements(elements, [$($n,)*])
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
