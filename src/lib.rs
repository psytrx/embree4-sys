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

impl Default for RTCRay {
    fn default() -> Self {
        Self {
            org_x: Default::default(),
            org_y: Default::default(),
            org_z: Default::default(),
            tnear: Default::default(),
            dir_x: Default::default(),
            dir_y: Default::default(),
            dir_z: Default::default(),
            time: Default::default(),
            tfar: std::f32::INFINITY,
            mask: -1_i32 as u32,
            id: Default::default(),
            flags: Default::default(),
        }
    }
}

impl Default for RTCHit {
    fn default() -> Self {
        Self {
            Ng_x: Default::default(),
            Ng_y: Default::default(),
            Ng_z: Default::default(),
            u: Default::default(),
            v: Default::default(),
            primID: Default::default(),
            geomID: RTC_INVALID_GEOMETRY_ID,
            instID: [RTC_INVALID_GEOMETRY_ID],
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for RTCBounds {
    fn default() -> Self {
        Self {
            lower_x: Default::default(),
            lower_y: Default::default(),
            lower_z: Default::default(),
            align0: Default::default(),
            upper_x: Default::default(),
            upper_y: Default::default(),
            upper_z: Default::default(),
            align1: Default::default(),
        }
    }
}

unsafe impl Send for RTCIntersectArguments {}
unsafe impl Sync for RTCIntersectArguments {}
