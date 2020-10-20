#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use elf_cam::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!();

// #[wasm_bindgen_test]
// fn test_detect_go_runtime() {
//     let buffer = std::fs::read("tests/data/hello-world-go").expect("failed to load binary file");

//     let runtime = detect_runtime(&buffer)
//         .expect("failed to detect runtime")
//         .expect("failed to return some runtime");
//     assert_eq!(Runtime::Go, runtime);
// }

// #[wasm_bindgen_test]
// fn test_detect_rust_runtime() {
//     let buffer = std::fs::read("tests/data/hello-world-rs").expect("failed to load binary file");

//     let runtime = detect_runtime(&buffer)
//         .expect("failed to detect runtime")
//         .expect("failed to return some runtime");
//     assert_eq!(Runtime::Rust, runtime);
// }
