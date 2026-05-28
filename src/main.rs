mod app;
mod base64;
mod json_fmt;
mod dashboard;
mod url_codec;
mod uuid_gen;
mod jwt_decoder;
mod mermaid_editor;
mod timestamp_conv;
mod json_to_go;
mod json_to_rust;
mod qr_generator;

use app::App;

fn main() {
    // Provide nice panic logging in browser console
    console_error_panic_hook::set_once();
    
    // Mount the root App component
    leptos::mount::mount_to_body(App);
}
