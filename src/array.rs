#[derive(Clone, Debug)]
pub struct Array<T> {
    elements: Box<[T]>,
    dimensions: Box<[usize]>,
}

macro_rules! array_type {
    () => { T };
    ($n:ident, $($rest:ident,)*) => {
        [array_type!($($rest,)*); $n]
    }
}
macro_rules! impl_from_array {
    ($($n:ident,)*) => {
        impl<T, $(const $n: usize),*> From<array_type!($($n,)*)> for Array<T> {
            fn from(value: array_type!($($n,)*)) -> Self {
                todo!()
            }
        }
    };
}

const N: usize = 3;
type T = usize;
const TEST1: array_type!(N,) = todo!();
const TEST2: array_type!(N, N,) = todo!();

impl_from_array!(N1,N2,);

impl<T, const N: usize> From<[T; N]> for Array<T> {
    fn from(value: [T; N]) -> Self {
        Array {
            elements: value.into(),
            dimensions:
        }
    }
}
