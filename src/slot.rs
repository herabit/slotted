mod content;
mod gen;
mod occupied;
mod poisoned;
mod vacant;

pub use content::*;
pub use gen::*;
pub use occupied::*;
pub use poisoned::*;
pub use vacant::*;

use core::mem::ManuallyDrop;

/// A storage slot that has storage for a value and its current generation.
///
/// It can be in one of the following states:
///
/// - **Occupied**: This slot contains a valid, instantiated `T`.
/// - **Vacant**:   This slot is currently empty, but it can be initialized.
/// - **Poisoned**: This slot had a generation that overflowed, and cannot
///                 be used to store another `T`.
pub struct Slot<T> {
    pub(crate) gen: Gen,
    pub(crate) data: SlotData<T>,
}

impl<T> Slot<T> {
    #[inline]
    #[must_use]
    pub const fn gen(&self) -> Gen {
        self.gen
    }

    #[inline]
    #[must_use]
    pub const fn is_vacant(&self) -> bool {
        self.gen.is_vacant()
    }

    #[inline]
    #[must_use]
    pub const fn is_occupied(&self) -> bool {
        self.gen.is_occupied()
    }

    #[inline]
    #[must_use]
    pub const fn is_poisoned(&self) -> bool {
        self.gen.is_poisoned()
    }

    #[inline]
    #[must_use]
    pub const fn status(&self) -> Status {
        self.gen.status()
    }
}

impl<T> Drop for Slot<T> {
    #[inline]
    fn drop(&mut self) {
        if self.is_occupied() {
            unsafe { ManuallyDrop::drop(&mut self.data.value) }
        }
    }
}

impl<T> Clone for Slot<T> {
    #[inline]
    fn clone(&self) -> Self {
        todo!()
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        todo!()
    }
}

pub(crate) union SlotData<T> {
    /// Value for occupied slots.
    pub(crate) value: ManuallyDrop<T>,
    /// Stores an index to the next vacant slot in the
    /// vacant list.
    pub(crate) next_vacant: Option<u32>,
    /// Stores an index to the next poisoned slot in the
    /// poison list.
    pub(crate) next_poison: Option<u32>,
}
