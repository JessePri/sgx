// Copyright (c) 2022 The MobileCoin Foundation

//! Builds the FFI function bindings for trts (trusted runtime system) of the
//! Intel SGX SDK
use cargo_emit::{rustc_link_lib, rustc_link_search};

const TRTS_FUNCTIONS: &[&str] = &[
    "sgx_is_enclave_crashed",
    "sgx_is_outside_enclave",
    "sgx_is_within_enclave",
    "sgx_ocall",
    "sgx_ocalloc",
    "sgx_ocfree",
    "sgx_rdpkru",
    "sgx_read_rand",
    "sgx_register_exception_handler",
    "sgx_unregister_exception_handler",
    "sgx_wrpkru",
];

fn main() {
    let sgx_library_path = mc_sgx_core_build::sgx_library_path();
    let sgx_suffix = mc_sgx_core_build::sgx_library_suffix();
    rustc_link_lib!(&format!("static=sgx_trts{}", sgx_suffix));
    rustc_link_search!(&format!("{}/lib64", &sgx_library_path));

    let mut builder = mc_sgx_core_build::sgx_builder()
        .header("wrapper.h")
        .clang_arg(&format!("-I{}/include", &sgx_library_path))
        .blocklist_type("*");

    for f in TRTS_FUNCTIONS {
        builder = builder.allowlist_function(f);
    }

    let out_path = mc_sgx_core_build::build_output_path();
    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}