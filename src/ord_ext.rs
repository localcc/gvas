pub trait OrdExt<I> {
    fn between(&self, start: I, end: I) -> bool;
}

impl<I: Ord> OrdExt<I> for I {
    fn between(&self, start: I, end: I) -> bool {
        *self >= start && *self <= end
    }
}
