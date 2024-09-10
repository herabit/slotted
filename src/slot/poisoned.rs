use super::Slot;

#[repr(transparent)]
pub struct Poisoned<'a, T> {
    pub(crate) slot: &'a Slot<T>,
}

impl<'a, T> Clone for Poisoned<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Poisoned<'a, T> {}

#[repr(transparent)]
pub struct PoisonedMut<'a, T> {
    pub(crate) slot: &'a mut Slot<T>,
}
