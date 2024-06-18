use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn DemoParentChildrenCommunication() -> impl IntoView {
    view! {
        <h1>Demo parent children communication</h1>
        <p>
            It is easy for the parent to communicate to the child: pass ReadSignal, or a Signal, or even a MaybeSignal as a prob
        </p>
        <p>
            How can a child send notifications about events or state changes back up to the parent?
        </p>

        <ul>
            <li>
                <Approch01/>
            </li>

            <li>
                <Approch02/>
            </li>
            <li>
                <Approch03/>
            </li>

            <li>
                <Approch04/>
            </li>
        </ul>
    }
}

#[component]
pub fn Approch01() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>Pass WriteSignal from parent down to the child</p>
        <p>"Toggled? " {toggled}</p>
        <Approch01Child setter=set_toggled/>
    }
}

#[component]
pub fn Approch01Child(setter: WriteSignal<bool>) -> impl IntoView {
    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}

#[component]
pub fn Approch02() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>Use callbback or closure</p>
        <p>"Toggled? " {toggled}</p>
        <Approch02Child on_click=move |_| set_toggled.update(|value| *value = !*value)/>
        <Approch02ChildV2 on_click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}

#[component]
pub fn Approch02Child(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView {
    view! { <button on:click=on_click>"Toggle"</button> }
}

#[component]
pub fn Approch02ChildV2<F>(on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { <button on:click=on_click>Toggle</button> }
}

#[component]
pub fn Approch03() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>Use event listener</p>
        <p>"Toggled? " {toggled}</p>
        <Approch03Child on:click=move |_| { set_toggled.update(|value| *value = !*value) }/>
    }
}

#[component]
pub fn Approch03Child() -> impl IntoView {
    view! { <button>"Toggle"</button> }
}

#[component]
pub fn Approch04() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);

    // share `set_toggled` with all children of this component
    provide_context(set_toggled);

    view! {
        <p>Providing a context</p>
        <p>
            Contexts are identified by the type of the data you provide and they exist in a top-down tree that follows the contours of your UI tree.
        </p>
        <p>"Toggled? " {toggled}</p>

        <Approch04Layout/>
    }
}

#[component]
pub fn Approch04Layout() -> impl IntoView {
    view! {
        <div>
            <p>My layout</p>
        </div>
        <Approch04Content/>
    }
}

#[component]
pub fn Approch04Content() -> impl IntoView {
    view! {
        <div class="content">
            <Approch04Button/>
        </div>
    }
}

#[component]
pub fn Approch04Button() -> impl IntoView {
    // use_context searches up the context tree, hoping to
    // find a `WriteSignal<bool>`
    // in this case, I .expect() because I know I provided it
    let setter = use_context::<WriteSignal<bool>>().expect("WriteSignal<bool> provided");
    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}
