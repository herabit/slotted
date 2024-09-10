#![no_std]

use alloc::vec::Vec;
use slot::Slot;

extern crate alloc;

pub mod slot;

pub struct Slotted<T> {
    slots: Vec<Slot<T>>,
    next_vacant: Option<u32>,
    next_poison: Option<u32>,
}
