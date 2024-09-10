use core::{
    borrow::{Borrow, BorrowMut},
    fmt,
    hint::assert_unchecked,
    mem::{self, ManuallyDrop},
    ptr::addr_of,
};

use crate::slot::{Slot, Status};

use super::ContentMut;

#[repr(transparent)]
pub struct Occupied<'a, T> {
    pub(crate) slot: &'a Slot<T>,
}

impl<'a, T> Occupied<'a, T> {
    #[inline]
    #[must_use]
    pub fn slot(self) -> &'a Slot<T> {
        self.slot
    }

    #[inline]
    #[must_use]
    pub fn value(self) -> &'a T {
        unsafe {
            assert_unchecked(self.slot.is_occupied());

            &self.slot.data.value
        }
    }
}

impl<T> Clone for Occupied<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Occupied<'_, T> {}

impl<T> AsRef<T> for Occupied<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.value()
    }
}

impl<T> AsRef<Slot<T>> for Occupied<'_, T> {
    #[inline]
    fn as_ref(&self) -> &Slot<T> {
        self.slot()
    }
}

impl<T> Borrow<T> for Occupied<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.value()
    }
}

impl<T> Borrow<Slot<T>> for Occupied<'_, T> {
    #[inline]
    fn borrow(&self) -> &Slot<T> {
        self.slot()
    }
}

impl<T: fmt::Debug> fmt::Debug for Occupied<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Occupied")
            .field("gen", &self.slot.gen.get())
            .field("value", self.value())
            .finish()
    }
}

#[repr(transparent)]
pub struct OccupiedMut<'a, T> {
    pub(crate) slot: &'a mut Slot<T>,
}

#[no_mangle]
fn take<'a>(
    occupied: OccupiedMut<'a, [u8; 1024]>,
    index: u32,
    next_vacant: &mut Option<u32>,
    next_poison: &mut Option<u32>,
) -> ([u8; 1024], &'a mut Slot<[u8; 1024]>) {
    occupied.take(index, next_vacant, next_poison)
}

impl<'a, T> OccupiedMut<'a, T> {
    #[inline]
    #[must_use]
    pub(crate) fn take(
        self,
        index: u32,
        next_vacant: &mut Option<u32>,
        next_poison: &mut Option<u32>,
    ) -> (T, &'a mut Slot<T>) {
        unsafe {
            assert_unchecked(self.slot.is_occupied());
        }

        let slot = self.slot;
        let value = unsafe { addr_of!(slot.data.value).read() };

        slot.gen = slot.gen.next();

        // unsafe { assert_unchecked(!slot.gen.is_occupied()) };

        if slot.gen.is_vacant() {
            slot.data.next_vacant = mem::replace(next_vacant, Some(index));
        } else {
            slot.data.next_poison = mem::replace(next_poison, Some(index));
        }

        (ManuallyDrop::into_inner(value), slot)
    }

    #[inline]
    #[must_use]
    pub fn value(&self) -> &T {
        unsafe {
            assert_unchecked(self.slot.is_occupied());

            &self.slot.data.value
        }
    }

    #[inline]
    #[must_use]
    pub fn value_mut(&mut self) -> &mut T {
        unsafe {
            assert_unchecked(self.slot.is_occupied());

            &mut self.slot.data.value
        }
    }

    #[inline]
    #[must_use]
    pub fn into_value(self) -> &'a mut T {
        unsafe {
            assert_unchecked(self.slot.is_occupied());

            &mut self.slot.data.value
        }
    }

    #[inline]
    #[must_use]
    pub fn slot(&self) -> &Slot<T> {
        self.slot
    }

    #[inline]
    #[must_use]
    pub fn slot_mut(&mut self) -> &mut Slot<T> {
        self.slot
    }

    #[inline]
    #[must_use]
    pub fn into_slot(self) -> &'a mut Slot<T> {
        self.slot
    }

    #[inline]
    #[must_use]
    pub fn reborrow(&self) -> Occupied<'_, T> {
        Occupied { slot: self.slot }
    }

    #[inline]
    #[must_use]
    pub fn reborrow_mut(&mut self) -> OccupiedMut<'_, T> {
        OccupiedMut { slot: self.slot }
    }

    #[inline]
    #[must_use]
    pub fn into_shared(self) -> Occupied<'a, T> {
        Occupied { slot: self.slot }
    }
}

impl<T> AsRef<T> for OccupiedMut<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.value()
    }
}

impl<T> AsMut<T> for OccupiedMut<'_, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.value_mut()
    }
}

impl<T> Borrow<T> for OccupiedMut<'_, T> {
    #[inline]
    fn borrow(&self) -> &T {
        self.value()
    }
}

impl<T> BorrowMut<T> for OccupiedMut<'_, T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.value_mut()
    }
}

impl<T> AsRef<Slot<T>> for OccupiedMut<'_, T> {
    #[inline]
    fn as_ref(&self) -> &Slot<T> {
        self.slot()
    }
}

impl<T> AsMut<Slot<T>> for OccupiedMut<'_, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut Slot<T> {
        self.slot_mut()
    }
}

impl<T> Borrow<Slot<T>> for OccupiedMut<'_, T> {
    #[inline]
    fn borrow(&self) -> &Slot<T> {
        self.slot()
    }
}

impl<T> BorrowMut<Slot<T>> for OccupiedMut<'_, T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Slot<T> {
        self.slot_mut()
    }
}

impl<T: fmt::Debug> fmt::Debug for OccupiedMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedMut")
            .field("gen", &self.slot.gen.get())
            .field("value", self.value())
            .finish()
    }
}
