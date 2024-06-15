use leptos::*;

fn main() {
    leptos_bulma::build("./style");
    mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
