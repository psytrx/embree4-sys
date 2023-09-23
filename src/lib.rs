#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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
