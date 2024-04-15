fn main() {
    if webgpu_check::is_webgpu_available() {
        success_message();
        // Run your WebGPU application...
    } else {
        fail_message();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn fail_message() {
    eprintln!("WebGPU is NOT supported on this target!");
    _ = notify_rust::Notification::new()
        .summary("WebGPU")
        .body("WebGPU is NOT supported on this target!")
        .timeout(notify_rust::Timeout::Milliseconds(6000)) //milliseconds
        .show();
}

#[cfg(target_arch = "wasm32")]
fn fail_message() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.set_inner_html(
        r#"<style>
        p {
            margin: 2em 10em;
            font-family: sans-serif;
        }
        </style>
        <p><a href="https://caniuse.com/webgpu">WebGPU</a>
        is not enabled. Make sure your browser is updated to
        <a href="https://chromiumdash.appspot.com/schedule">Chrome M113</a> or
        another browser compatible with WebGPU.</p>"#,
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn success_message() {
    println!("WebGPU is enabled!");
    _ = notify_rust::Notification::new()
        .summary("WebGPU")
        .body("WebGPU is enabled!")
        .timeout(notify_rust::Timeout::Milliseconds(6000)) //milliseconds
        .show();
}

#[cfg(target_arch = "wasm32")]
fn success_message() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.set_inner_html(
        r#"
        <style>
        p {
            margin: 2em 10em;
            font-family: sans-serif;
        }
        </style>
        <p><a href="https://caniuse.com/webgpu">WebGPU</a> is enabled!</p>"#,
    );
}
