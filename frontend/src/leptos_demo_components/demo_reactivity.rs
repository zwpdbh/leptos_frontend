use leptos::html::Input;
use leptos::*;

#[component]
pub fn DemoReactivity() -> impl IntoView {
    view! {
        <h1>Demo Reactivity</h1>
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
            <li>
                <Demo04/>
            </li>
        </ul>
    }
}

#[component]
pub fn Demo01() -> impl IntoView {
    let (names, set_names) = create_signal(Vec::new());

    // This is not efficient
    if names().is_empty() {
        set_names(vec!["Alice".to_string()]);
    }

    // This is good
    if names.with(|names| names.is_empty()) {
        set_names.update(|names| names.push("Alice".to_string()));
    }

    view! { <p>Demo01: usage of with and update</p> }
}

#[component]
pub fn Demo02() -> impl IntoView {
    let (first, _) = create_signal("Bob".to_string());
    let (middle, _) = create_signal("J.".to_string());
    let (last, _) = create_signal("Smith".to_string());

    // This is not good
    let _name = move || {
        first.with(|first| {
            middle.with(|middle| last.with(|last| format!("{first} {middle} {last}")))
        })
    };

    let name = move || with!(|first, middle, last| format!("{first} {middle} {last}"));

    view! {
        <p>Demo02: with! macro</p>
        <p>name is: {name}</p>
    }
}

#[component]
pub fn Demo03() -> impl IntoView {
    // B is a function of A. Create a signal for A and a derived signal or memo for B.
    let (count, _set_count) = create_signal(1);
    let _derived_signal_double_count = move || count() * 2;
    let _memoized_double_count = create_memo(move |_| count() * 2);

    // C is a function of A and some other thing B. Create signals for A and B and a derived signal or memo for C.
    let (first_name, _set_first_name) = create_signal("Bridget".to_string());
    let (last_name, _set_last_name) = create_signal("Jones".to_string());
    let _full_name = move || with!(|first_name, last_name| format!("{first_name} {last_name}"));

    // A and B are independent signals, but sometimes updated at the same time. When you make the call to update A, make a separate call to update B.
    let (_age, set_age) = create_signal(32);
    let (_favorite_number, set_favorite_number) = create_signal(42);
    // use this to handle a click on a `Clear` button
    let _clear_handler = move |_: i32| {
        set_age(0);
        set_favorite_number(0);
    };

    view! { <h3>Making singals depends on each other</h3> }
}

#[derive(Copy, Clone)]
struct LogContext(RwSignal<Vec<String>>);

#[component]
pub fn Demo04() -> impl IntoView {
    // Just making a visible log here
    // You can ignore this...
    let log = create_rw_signal::<Vec<String>>(vec![]);
    let logged = move || log().join("\n");

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `RwSignal<Vec<String>>` contexts
    // and makes it easier to refer to it
    provide_context(LogContext(log));

    view! {
        <CreateAnEffect/>
        <pre>{logged}</pre>
    }
}

#[component]
fn CreateAnEffect() -> impl IntoView {
    let (first, set_first) = create_signal(String::new());
    let (last, set_last) = create_signal(String::new());
    let (use_last, set_use_last) = create_signal(true);

    // this will add the name to the log
    // any time one of the source signals changes
    create_effect(move |_| {
        log(if use_last() {
            with!(|first, last| format!("{first} {last}"))
        } else {
            first()
        })
    });

    view! {
        <h1>
            <code>"create_effect"</code>
            " Version"
        </h1>
        <form>
            <label>
                "First Name"
                <input
                    type="text"
                    name="first"
                    prop:value=first
                    on:change=move |ev| set_first(event_target_value(&ev))
                />
            </label>
            <label>
                "Last Name"
                <input
                    type="text"
                    name="last"
                    prop:value=last
                    on:change=move |ev| set_last(event_target_value(&ev))
                />
            </label>
            <label>
                "Show Last Name"
                <input
                    type="checkbox"
                    name="use_last"
                    prop:checked=use_last
                    on:change=move |ev| set_use_last(event_target_checked(&ev))
                />
            </label>
        </form>
    }
}

#[component]
fn ManualVersion() -> impl IntoView {
    let first = create_node_ref::<Input>();
    let last = create_node_ref::<Input>();
    let use_last = create_node_ref::<Input>();

    let mut prev_name = String::new();
    let on_change = move |_| {
        log("      listener");
        let first = first.get().unwrap();
        let last = last.get().unwrap();
        let use_last = use_last.get().unwrap();
        let this_one = if use_last.checked() {
            format!("{} {}", first.value(), last.value())
        } else {
            first.value()
        };

        if this_one != prev_name {
            log(&this_one);
            prev_name = this_one;
        }
    };

    view! {
        <h1>"Manual Version"</h1>
        <form on:change=on_change>
            <label>"First Name" <input type="text" name="first" node_ref=first/></label>
            <label>"Last Name" <input type="text" name="last" node_ref=last/></label>
            <label>
                "Show Last Name" <input type="checkbox" name="use_last" checked node_ref=use_last/>
            </label>
        </form>
    }
}

#[component]
fn EffectVsDerivedSignal() -> impl IntoView {
    let (my_value, set_my_value) = create_signal(String::new());
    // Don't do this.
    /*let (my_optional_value, set_optional_my_value) = create_signal(Option::<String>::None);

    create_effect(move |_| {
        if !my_value.get().is_empty() {
            set_optional_my_value(Some(my_value.get()));
        } else {
            set_optional_my_value(None);
        }
    });*/

    // Do this
    let my_optional_value =
        move || (!my_value.with(String::is_empty)).then(|| Some(my_value.get()));

    view! {
        <input prop:value=my_value on:input=move |ev| set_my_value(event_target_value(&ev))/>

        <p>
            <code>"my_optional_value"</code>
            " is "
            <code>
                <Show when=move || my_optional_value().is_some() fallback=|| view! { "None" }>
                    "Some(\""
                    {my_optional_value().unwrap()}
                    "\")"
                </Show>
            </code>
        </p>
    }
}

#[component]
pub fn Show<F, W, IV>(
    /// The components Show wraps
    children: Box<dyn Fn() -> Fragment>,
    /// A closure that returns a bool that determines whether this thing runs
    when: W,
    /// A closure that returns what gets rendered if the when statement is false
    fallback: F,
) -> impl IntoView
where
    W: Fn() -> bool + 'static,
    F: Fn() -> IV + 'static,
    IV: IntoView,
{
    let memoized_when = create_memo(move |_| when());

    move || match memoized_when.get() {
        true => children().into_view(),
        false => fallback().into_view(),
    }
}

fn log(msg: impl std::fmt::Display) {
    let log = use_context::<LogContext>().unwrap().0;
    log.update(|log| log.push(msg.to_string()));
}
