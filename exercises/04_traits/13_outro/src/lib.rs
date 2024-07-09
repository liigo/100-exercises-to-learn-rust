// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct SaturatingU16(u16);

// impl From<T>

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16(value)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16(*value)
    }
}

impl  From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16(value as u16)
    }
}

impl  From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16(*value as u16)
    }
}

// impl Add<T>

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(rhs))
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16(self.0.saturating_add(*rhs))
    }
}

// PartialEq PartialOrd

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u16> for SaturatingU16 {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}