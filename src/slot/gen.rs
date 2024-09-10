//! There are definitely better ways of encoding
//! the poison value.
//!
//! However, encoding `u32::MAX-1..=u32::MAX` as the poison range
//! of values for me is the easiest encoding to reason about.
//!
//! The intuition is this:
//!
//! 1. Vacant generations are always even.
//!
//! 2. Occupied generations are always odd.
//!
//! 3. Poisoning should only ever occur if the previous
//!    generation was occupied.
//!
//! 4. [`u32::MAX`] is odd, and therefore encodes for occupancy.
//!
//! 5. Since poisoning should only ever occur after occupancy, and
//!    the value we want to treat as the poison value encodes occupancy,
//!    we also mark the previous generation before [`u32::MAX`], `u32::MAX - 1`,
//!    as encoding poisoning.
//!
//! This has the disadvantage of using two values that encode the same semantics.
//!
//! However, considering how rare poisoning should realistically be (in most contexts
//! it is unlikely to ever even occur), this should be a non-issue.
//!
//! As for performance, Rust seems to do a pretty good job with turning operations into branchless
//! ones as the logic for the vacancy and poison cases are pretty similar usually.
//!
//! If it proves to be a problem, trickery to help hint the branch predictor for which case is most
//! likely should be trivial to implement in most contexts.

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Gen(pub(crate) u32);

impl Gen {
    pub const VACANT: Gen = Gen(0);

    pub const POISON_MIN: Gen = Gen(u32::MAX - 1);
    pub const POISON_MAX: Gen = Gen(u32::MAX);

    /// Get the integer representation of this generation.
    #[inline]
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Does this generation encode poisoning?
    #[inline]
    #[must_use]
    #[no_mangle]
    pub const fn is_poisoned(self) -> bool {
        // const MIN: u32 = Gen::POISON_MIN.0;
        // const MAX: u32 = Gen::POISON_MAX.0;

        // matches!(self.0, MIN..=MAX)
        self.status().is_poisoned()
    }

    /// Does this generation encode vacancy?
    #[inline]
    #[must_use]
    #[no_mangle]
    pub const fn is_vacant(self) -> bool {
        // (self.0 < Gen::POISON_MIN.0) & (self.0 % 2 == 0)
        self.status().is_vacant()
    }

    /// Does this generation encode occupancy?
    #[inline]
    #[must_use]
    #[no_mangle]
    pub const fn is_occupied(self) -> bool {
        // (self.0 < Gen::POISON_MIN.0) & (self.0 % 2 != 0)
        self.status().is_occupied()
    }

    /// Get the status of this generation.
    #[inline]
    #[must_use]
    pub const fn status(self) -> Status {
        // This offers better codegen than calling the individual methods.
        if self.0 >= Gen::POISON_MIN.0 {
            Status::Poisoned
        } else if self.0 % 2 == 0 {
            Status::Vacant
        } else {
            Status::Occupied
        }
    }

    /// Get the generation that follows this one.
    #[inline]
    #[must_use]
    pub const fn next(self) -> Gen {
        Gen(self.0.saturating_add(1))
    }
}

/// An enum that indicates the current status of a slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Status {
    #[default]
    Vacant,
    Occupied,
    Poisoned,
}

impl Status {
    #[inline]
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Status::Vacant => "Vacant",
            Status::Occupied => "Occupied",
            Status::Poisoned => "Poisoned",
        }
    }

    /// Does this status indicate an alive slot?
    #[inline]
    #[must_use]
    pub const fn is_living(self) -> bool {
        matches!(self, Status::Vacant | Status::Occupied)
    }

    /// Returns whether this is [`Status::Vacant`].
    #[inline]
    #[must_use]
    pub const fn is_vacant(self) -> bool {
        matches!(self, Self::Vacant)
    }

    /// Returns whether this is [`Status::Occupied`].
    #[inline]
    #[must_use]
    pub const fn is_occupied(self) -> bool {
        matches!(self, Self::Occupied)
    }

    /// Returns whether this si [`Status::Poisoned`].
    #[inline]
    #[must_use]
    pub const fn is_poisoned(self) -> bool {
        matches!(self, Self::Poisoned)
    }
}
