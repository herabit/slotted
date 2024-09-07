use core::{fmt, mem::ManuallyDrop};

pub struct Slot<T> {
    pub(crate) gen: Gen,
    pub(crate) data: SlotData<T>,
}

impl<T> Drop for Slot<T> {
    #[inline]
    fn drop(&mut self) {
        if self.gen.is_occupied() {
            unsafe { ManuallyDrop::drop(&mut self.data.value) }
        }
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

/// Represents the generation of a slot.
///
/// # Semantics
///
/// - Zero: A slot is poisoned and must not be used anymore.
///         this is is considered a special case.
/// - Odd:  A slot is vacant and can be occupied.
/// - Even: A slot is occupied.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Gen(#[doc(hidden)] pub(crate) u32);

impl Gen {
    /// The default and starting generation.
    pub const NEW: Gen = Gen(1);

    /// The poison value.
    pub const POISON: Gen = Gen(0);

    /// Create a new generation
    #[inline]
    #[must_use]
    pub const fn new(gen: u32) -> Gen {
        Gen(gen)
    }

    /// Get the underlying generation value.
    #[inline]
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Check if this generation is the poison value (zero).
    #[inline]
    #[must_use]
    pub const fn is_poisoned(self) -> bool {
        self.0 == Gen::POISON.0
    }

    /// Check if this generation is vacant (odd).
    #[inline]
    #[must_use]
    pub const fn is_vacant(self) -> bool {
        self.0 % 2 > 0
    }

    /// Check if this generation is occupied (nonzero and even).
    #[inline]
    #[must_use]
    pub const fn is_occupied(self) -> bool {
        !self.is_poisoned() & !self.is_vacant()
    }

    /// Get the next generation after this one.
    #[inline]
    #[must_use]
    pub const fn next(self) -> Gen {
        match self.0.checked_add(1) {
            Some(gen) => Gen(gen),
            None => Gen::POISON,
        }
    }

    /// Get the current state of this generation.
    #[inline]
    #[must_use]
    pub const fn state(self) -> State {
        State::from_generation(self.0)
    }
}

impl fmt::Debug for Gen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state() {
            State::Vacant => f.debug_tuple("Vacant").field(&self.0).finish(),
            State::Occupied => f.debug_tuple("Occupied").field(&self.0).finish(),
            State::Posioned => f.debug_tuple("Poisoned").finish(),
        }
    }
}

impl Default for Gen {
    #[inline]
    fn default() -> Self {
        Gen::NEW
    }
}

/// Represents the state of a slot without any additional information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum State {
    /// Indicates that a slot is vacant.
    #[default]
    Vacant,
    /// Indicates that a slot is occupied with some value.
    Occupied,
    /// Indicates that a slot is poisoned, meaning it cannot be used to store
    /// values again.
    ///
    /// This should only ever occur if a slot's
    /// # Generations
    ///
    /// Poisoned slots have a generation which is equal to `0`.
    Posioned,
}

impl State {
    /// Get the state of a slot from its generation.
    ///
    /// - Odd generations indicate vacancy.
    /// - Nonzero even generations indicate occupancy.
    /// - Zero indicates poisoning.
    #[inline]
    #[must_use]
    pub const fn from_generation(gen: u32) -> State {
        if gen % 2 > 0 {
            State::Vacant
        } else if gen != 0 {
            State::Occupied
        } else {
            State::Posioned
        }
    }

    /// Get the next state of a slot.
    ///
    /// This does not account for a generation overflowing.
    #[inline]
    #[must_use]
    pub const fn next(self) -> State {
        match self {
            State::Vacant => State::Occupied,
            State::Occupied => State::Vacant,
            State::Posioned => State::Posioned,
        }
    }
}
