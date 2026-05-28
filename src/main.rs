mod app;
mod base64;
mod csv_json;
mod dashboard;
mod json_fmt;
mod json_to_go;
mod json_to_rust;
mod jwt_decoder;
mod mermaid_editor;
mod qr_generator;
mod sql_fmt;
mod timestamp_conv;
mod url_codec;
mod uuid_gen;
mod yaml_json;

use app::App;

fn main() {
    // Provide nice panic logging in browser console
    console_error_panic_hook::set_once();

    // Mount the root App component
    leptos::mount::mount_to_body(App);
}
