use std::ops::{Index, IndexMut};



pub trait IndexSet: IntoIterator<Item = usize> {}

pub trait ArrayRepr<T> {
    type Partial: ?Sized + ArrayRepr<T>;
    fn partial_index(&self, i: usize) -> &Self::Partial;
    fn partial_index_mut(&mut self, i: usize) -> &mut Self::Partial;
}

impl<T, I: IndexSet> Index<I> for ArrayRepr<T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let mut partial: Self = self;
        for i in index {
            partial = partial.partial_index(i);
        }

    }
}

impl<T: Index + IndexMut> ArrayRepr for T {}


mod test {
    fn test() {
        // goal:
        // let x = [
        //     [1, 2, 3],
        //     [4, 5, 6]
        // ];

        // let x11 = x[[1, 1]];
    }
}
