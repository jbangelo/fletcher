#![no_std]

pub mod fletcher16;
pub mod fletcher32;
pub mod fletcher64;
pub mod generic_fletcher;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate byteorder;
