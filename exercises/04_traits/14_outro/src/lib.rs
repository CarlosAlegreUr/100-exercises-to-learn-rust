// TODO: Define a new `SaturatingU16` type.
//   (DONE) It should hold a `u16` value.
//   (DONE) It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   (DONE) It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   (DONE) It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   (DONE) It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::cmp::PartialEq;
use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

// From

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self {
            value: value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self {
            value: *value as u16,
        }
    }
}

// Equals

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

// Addition

impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: Self) -> SaturatingU16 {
        let sum = self.value.saturating_add(other.value);
        Self { value: sum }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> SaturatingU16 {
        let sum: u16 = self.value.saturating_add(other.value);
        Self { value: sum }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: u16) -> SaturatingU16 {
        let sum: u16 = self.value.saturating_add(other);
        Self { value: sum }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u16) -> SaturatingU16 {
        let sum: u16 = self.value.saturating_add(*other);
        Self { value: sum }
    }
}
