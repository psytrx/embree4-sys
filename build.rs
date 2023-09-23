use std::{env, path::PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = PathBuf::from(&manifest_dir).join("lib");
    let wrapper_path = PathBuf::from(&lib_dir).join("wrapper.h");

    println!("cargo:rustc-link-lib=embree4");
    println!("cargo:rerun-if-changed={}", wrapper_path.display());

    let bindings = bindgen::Builder::default()
        .header(format!("{}", wrapper_path.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("rtc.*")
        .allowlist_type("RTC.*")
        .allowlist_var("RTC.*")
        .rustified_enum("RTCFormat")
        .rustified_enum("RTCBuildQuality")
        .rustified_enum("RTCDeviceProperty")
        .rustified_enum("RTCError")
        .rustified_enum("RTCBufferType")
        .rustified_enum("RTCGeometryType")
        .rustified_enum("RTCSubdivisionMode")
        .bitfield_enum("RTC.*Flags")
        .generate()
        .expect("Unable to generate bindings");

    let bindings_source = bindings
        .to_string()
        .replace("RTC_FORMAT_", "")
        .replace("RTC_BUILD_QUALITY_", "")
        .replace("RTC_RAY_QUERY_FLAG_", "")
        .replace("RTC_DEVICE_PROPERTY_", "")
        .replace("RTC_ERROR_", "")
        .replace("RTC_BUFFER_TYPE_", "")
        .replace("RTC_GEOMETRY_TYPE_", "")
        .replace("RTC_SUBDIVISION_MODE_", "")
        .replace("RTC_CURVE_FLAG_", "")
        .replace("RTC_SCENE_FLAG_", "")
        .replace("RTC_BUILD_FLAG_", "")
        .replace("RTC_FORMAT_", "")
        .replace(
            "pub type size_t = ::std::os::raw::c_ulong",
            "pub type size_t = usize",
        )
        .replace(
            "pub type __ssize_t = ::std::os::raw::c_long",
            "pub type __ssize_t = isize",
        );

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_dir.join("bindings.rs");

    std::fs::write(out_path, bindings_source).expect("Couldn't write bindings!");
}
