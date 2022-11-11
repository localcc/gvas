pub struct ScopedStackEntry<T> {
    stack: *mut Vec<T>,
}

impl<T> ScopedStackEntry<T> {
    pub fn new(stack: &mut Vec<T>, value: T) -> Self {
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
