#[cfg(not(target_arch = "wasm32"))]
#[test]
fn can_check_native() {
    webgpu_check::is_webgpu_available();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn can_check_wasm() {
    webgpu_check::is_webgpu_available();
}
