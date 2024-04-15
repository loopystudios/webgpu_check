//! Easily detect if WebGPU is supported on the current target platform.
//!
//! # Usage
//! ```rust
//! if !webgpu_check::is_webgpu_available() {
//!     panic!("This platform doesn't support WebGPU!");
//! }
//! // Proceed to run your WebGPU application...
//! ```

/// Detect if WebGPU is supported on this target.
///
/// For current browser compatibility, see https://caniuse.com/webgpu
#[cfg(target_arch = "wasm32")]
pub fn is_webgpu_available() -> bool {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        navigator.gpu().is_object()
    } else {
        // No global window! This is probably running from a headless environment.
        // e.g. `wasm-pack test --node`
        false
    }
}

/// Detect if WebGPU is supported on this target.
///
/// For current browser compatibility, see https://caniuse.com/webgpu
#[cfg(not(target_arch = "wasm32"))]
pub fn is_webgpu_available() -> bool {
    true
}
