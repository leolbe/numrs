mod conversions;

pub struct Array<T> {
    elements: Box<[T]>,
    shape: Box<[usize]>,
}
