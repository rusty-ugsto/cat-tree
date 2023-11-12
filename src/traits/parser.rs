pub trait Parser<T> {
    fn parse(&self) -> T;
}
