#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "collections")]
pub mod collections;
#[cfg(feature = "numbers")]
pub mod numbers;

#[doc = include_str ! ("../README.md")]
type _DoctestReadme = ();
