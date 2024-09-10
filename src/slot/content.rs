use super::{Occupied, OccupiedMut, Poisoned, PoisonedMut, Vacant, VacantMut};

pub enum Content<'a, T> {
    Occupied(Occupied<'a, T>),
    Vacant(Vacant<'a, T>),
    Poisoned(Poisoned<'a, T>),
}

impl<T> Clone for Content<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Content<'_, T> {}

pub enum ContentMut<'a, T> {
    Occupied(OccupiedMut<'a, T>),
    Vacant(VacantMut<'a, T>),
    Poisoned(PoisonedMut<'a, T>),
}
