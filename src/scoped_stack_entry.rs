pub(crate) struct ScopedStackEntry<T> {
    stack: *mut Vec<T>,
}

/// This struct adds an entry to a Vec on creation and removes an entry when going out of scope
impl<T> ScopedStackEntry<T> {
    /// Create new instance of ScopedStackEntry
    pub(crate) fn new(stack: &mut Vec<T>, value: T) -> Self {
        stack.push(value);
        Self {
            stack: stack as *mut Vec<T>,
        }
    }
}

impl<T> Drop for ScopedStackEntry<T> {
    fn drop(&mut self) {
        if let Some(stack) = unsafe { self.stack.as_mut() } {
            stack.pop();
        }
    }
}
