use leptos::*;

#[component]
pub fn DemoFormAndInput() -> impl IntoView {
    view! {
        <h1>"Demo form and input"</h1>
        <ul>
            <li>
                <DemoControlledInputs/>
            </li>
            <li>
                <DemoUncontrolledInputs/>
            </li>
            <li>
                <DemoTextArea/>
            </li>
            <li>
                <DemoSelect/>
            </li>
        </ul>
    }
}

#[component]
pub fn DemoControlledInputs() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());
    view! {
        <h2>Controlled input</h2>
        <input
            type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }

            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn DemoUncontrolledInputs() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name(value);
    };

    view! {
        <h2>Uncontrolled input</h2>
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
pub fn DemoTextArea() -> impl IntoView {
    let (some_value, set_some_value) = create_signal("text area".to_string());
    view! {
        <h2>Text area</h2>
        <textarea
            prop:value=move || some_value.get()
            on:input=move |ev| {
                set_some_value(event_target_value(&ev));
            }
        >

            {move || some_value.get_untracked()}
        </textarea>

        <p>"text is: " {some_value}</p>
    }
}

#[component]
pub fn DemoSelect() -> impl IntoView {
    let (value, set_value) = create_signal("A".to_string());
    view! {
        <h2>Demo select</h2>
        <select on:change=move |ev| {
            let new_value = event_target_value(&ev);
            set_value(new_value);
        }>
            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>
        <p>"Your selection is: " {value}</p>
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option value=is selected=move || value() == is>
            {is}
        </option>
    }
}
