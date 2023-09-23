//! Rust FFI bindings for
//! [Intel's Embree](https://www.embree.org/)
//! 4 high-performance ray tracing library.
//!
//! Bindings are generated via
//! [rust-bindgen](https://github.com/rust-lang/rust-bindgen).
//!
//! A valid Embree installation is required. See
//! [Installation of Embree](https://github.com/embree/embree#installation-of-embree)
//! from the Embree docs.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const RTC_INVALID_GEOMETRY_ID: u32 = (-1_i32) as u32;

impl Default for RTCBuildQuality {
    fn default() -> Self {
        Self::MEDIUM
    }
}

#[allow(clippy::derivable_impls)]
impl Default for RTCSceneFlags {
    fn default() -> Self {
        Self(Default::default())
    }
}
