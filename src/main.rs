mod app;
mod upload;
mod components;
mod template;

// use app::*;
use leptos::prelude::*;
use template::Landingpage;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <Landingpage /> }
    })   
}
