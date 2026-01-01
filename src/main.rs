mod components;
mod data;
mod layout;
mod models;
mod routes;

use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <crate::routes::roadmap::RoadmapPage />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
