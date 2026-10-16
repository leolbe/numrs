use crate::array::Array;

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
