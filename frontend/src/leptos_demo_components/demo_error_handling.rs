use leptos::*;

#[component]
pub fn DemoErrorHandling() -> impl IntoView {
    view! {
        <h1>Demo error handling</h1>
        <ul>
            <li>
                <Demo01/>
            </li>

            <li>
                <Demo02/>
            </li>
        </ul>
    }
}

#[component]
pub fn Demo01() -> impl IntoView {
    view! { <NumericInputV1/> }
}

#[component]
pub fn Demo02() -> impl IntoView {
    view! { <NumericInputV2/> }
}

#[component]
fn NumericInputV1() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <label>
            "Type a number (or not!)" <input type="number" on:input=on_input/>
            <p>"You entered " <strong>{value}</strong></p>
        </label>
    }
}

#[component]
fn NumericInputV2() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            // the fallback receives a signal containing current errors
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}

                        </ul>
                    </div>
                }
            }>

                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
