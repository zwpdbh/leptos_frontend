use leptos::*;

#[component]
pub fn DemoControlFlow() -> impl IntoView {
    view! {
        <div class="section">
            <h1 class="title">Demo control flow</h1>
            <p>"It is used for should I render this part of the view or not"</p>

            <ul>
                <li>
                    <Demo01/>
                </li>

                <li>
                    <Demo02/>
                </li>

                <li>
                    <Demo03/>
                </li>
            </ul>
        </div>
    }
}

#[component]
pub fn Demo01() -> impl IntoView {
    let message = move || {
        if is_odd() {
            Some("Ding ding ding!")
        } else {
            None
        }
    };
    view! {
        <div class="container">
            <p class="subtitle">demo 01</p>
            <p>{message}</p>
        </div>
    }
}

fn is_odd() -> bool {
    true
}

#[component]
fn Big() -> impl IntoView {
    view! { <p>big component</p> }
}

#[component]
fn Small() -> impl IntoView {
    view! { <p>small component</p> }
}

#[component]
pub fn Demo02() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    let message = move || {
        if value() > 5 {
            logging::log!("{}: rendering Big", value());
            "Big"
        } else {
            logging::log!("{}: rendering Small", value());
            "Small"
        }
    };

    view! {
        <div class="container">
            <p class="subtitle">demo 02: conditional render</p>
            <p>
                Be careful when use "ReadSignal<i32>"
                in the condition because it will trigger rerender due to reactive
            </p>
            <button on:click=move |_| {
                set_value.update(|n| *n += 1)
            }>"click me:" {move || value}</button>
            <p>{message}</p>

            <p>solution is to use "<show/>"</p>
            <Show when=move || { value() > 5 } fallback=|| view! { <Big/> }>
                <Small/>
            </Show>
        </div>
    }
}

#[component]
pub fn Demo03() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    view! {
        <div class="container">
            <p class="subtitle">"Demo03: Type conversion for match"</p>
            <button on:click=move |_| {
                set_value.update(|n| *n += 1)
            }>"click me:" {move || value}</button>
            <div>
                {move || match is_odd() {
                    true if value() == 1 => view! { <pre>"One"</pre> }.into_any(),
                    false if value() == 2 => view! { <p>"Two"</p> }.into_any(),
                    _ => view! { <textarea>{value()}</textarea> }.into_any(),
                }}

            </div>
        </div>
    }
}
