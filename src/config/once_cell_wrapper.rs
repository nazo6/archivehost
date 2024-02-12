use std::ops::Deref;

use once_cell::sync::OnceCell;

pub struct OnceCellWrapper<T>(OnceCell<T>);
impl<T> Deref for OnceCellWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.get().expect("Not initialized")
    }
}
impl<T> OnceCellWrapper<T> {
    pub const fn new() -> Self {
        Self(OnceCell::<T>::new())
    }
    pub fn set(&self, value: T) -> Result<(), T> {
        self.0.set(value)
    }
}
