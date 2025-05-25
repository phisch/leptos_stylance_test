use leptos::prelude::*;
use stylance::import_crate_style;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> })
}

import_crate_style!(style, "src/test.scss");

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class=style::foo>
            <h1>"Some Title"</h1>
            <p>"Lorem ipsum dolor sit amet."</p>
        </div>
    }
}
