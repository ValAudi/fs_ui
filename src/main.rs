mod app;
mod upload;
mod components;

use app::*;
use leptos::prelude::*;
use upload::Upload;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Upload />
        }
    })
}
