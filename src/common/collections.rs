
pub trait ForEach {
    type Item;
    fn for_each(self, action: fn(&Self::Item) -> ());
}

impl<T> ForEach for &[T] {
    type Item = T;

    #[inline(always)]
    fn for_each(self, action: fn(&Self::Item) -> ()) {
        let mut iter = self.iter();
        while let Option::Some(item) = iter.next() {
            action(item);
        }
    }
}