use crate::slot::Slot;

#[repr(transparent)]
pub struct Vacant<'a, T> {
    pub(crate) slot: &'a Slot<T>,
}

impl<'a, T> Clone for Vacant<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Vacant<'a, T> {}

#[repr(transparent)]
pub struct VacantMut<'a, T> {
    pub(crate) slot: &'a mut Slot<T>,
}
