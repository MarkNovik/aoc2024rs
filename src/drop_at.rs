pub trait DropAt {
    fn drop_at(self, index: usize) -> Self;
}

impl<T> DropAt for Vec<T> {
    fn drop_at(mut self, index: usize) -> Self {
        self.remove(index);
        self
    }
}
