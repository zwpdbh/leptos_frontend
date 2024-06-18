use gloo_timers::future::TimeoutFuture;
use leptos::html::Input;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn DemoAsync() -> impl IntoView {
    view! {
        <div class="section">
            <h1 class="title">Demo Async</h1>
            <ul>
                <li>
                    <Demo01/>
                </li>
                <li>
                    <Demo02V1/>
                </li>
                <li>
                    <Demo02V2/>
                </li>
                <li>
                    <Demo03/>
                </li>
                <li>
                    <Demo04/>
                </li>
                <li>
                    <Demo05/>
                </li>
            </ul>
        </div>
    }
}

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
pub fn Demo01() -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".to_string())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <div class="container">
            <h1 class="subtitle">Load data with resource</h1>

            <button on:click=move |_| {
                set_count.update(|n| *n += 1);
            }>

                "Click me"
            </button>
            <p>
                <code>"stable"</code>
                ": "
                {move || stable.get()}
            </p>
            <p>
                <code>"count"</code>
                ": "
                {count}
            </p>
            <p>
                <code>"async_value"</code>
                ": "
                {async_result}
                <br/>
                {is_loading}
            </p>
        </div>
    }
}

#[component]
fn ShowA(a: i32) -> impl IntoView {
    view! { <p>some A: {a}</p> }
}

#[component]
fn ShowB(b: i32) -> impl IntoView {
    view! { <p>some B: {b}</p> }
}

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_a(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(5_000).await;
    value * 10
}

async fn load_b(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    value * 10
}

/// Demo await on multiple resources V1 using match
#[component]
pub fn Demo02V1() -> impl IntoView {
    let (count, _set_count) = create_signal(0);
    let (count2, _set_count2) = create_signal(0);
    let a = create_resource(count, |count| async move { load_a(count).await });
    let b = create_resource(count2, |count| async move { load_b(count).await });

    view! {
        <div class="container">
            <h1 class="subtitle">"Demo: Wait two resources v1"</h1>
            {move || match (a.get(), b.get()) {
                (Some(a), Some(b)) => {
                    view! {
                        <ShowA a/>
                        <ShowB b/>
                    }
                        .into_view()
                }
                _ => view! { <p>"Loading..."</p> }.into_view(),
            }}

        </div>
    }
}

/// Demo await on multiple resources V2 using Suspense
#[component]
pub fn Demo02V2() -> impl IntoView {
    let (count, _set_count) = create_signal(0);
    let (count2, _set_count2) = create_signal(0);
    let a = create_resource(count, |count| async move { load_a(count).await });
    let b = create_resource(count2, |count| async move { load_b(count).await });

    view! {
        <div class="container">
            <h1 class="subtitle">"Demo: Wait two resources v2"</h1>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <h4>"My Data"</h4>
                <h5>"A"</h5>
                {move || { a.get().map(|a| view! { <ShowA a/> }) }}

                <h5>"B"</h5>
                {move || { b.get().map(|b| view! { <ShowB b/> }) }}

            </Suspense>
        </div>
    }
}

async fn fetch_monkeys(monkey: i32) -> i32 {
    // maybe this didn't need to be async
    TimeoutFuture::new(5_000).await;
    monkey * 2
}

/// Demo 6.2 <Await/> to only render loaded resource
/// Not loaded resource are simply not rendered at all.
#[component]
pub fn Demo03() -> impl IntoView {
    view! {
        <div class="container">
            <h3 class="subtitle">
                "Demo: use <Await/> for some future to resolve before rendering"
            </h3>
            <Await
                // `future` provides the `Future` to be resolved
                future=|| fetch_monkeys(3)
                // the data is bound to whatever variable name you provide
                let:data
            >
                // you receive the data by reference and can use it in your view here
                <p>{*data} " little monkeys, jumping on the bed."</p>
            </Await>
        </div>
    }
}

async fn important_api_call(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User not found",
    }
    .to_string()
}

/// Use Transition instead of Suspense to prevent falling back every time when user
/// keeps trigger multiple loading
/// This seperate initial loading and other loadings.
#[component]
pub fn Demo04() -> impl IntoView {
    let (tab, set_tab) = create_signal(0);

    // this will reload every time `tab` changes
    let user_data = create_resource(tab, |tab| async move { important_api_call(tab).await });

    view! {
        <div class="container">
            <h3 class="subtitle">Demo Transition</h3>
            <div class="buttons">
                <button on:click=move |_| set_tab(0) class:selected=move || tab() == 0>
                    "Tab A"
                </button>
                <button on:click=move |_| set_tab(1) class:selected=move || tab() == 1>
                    "Tab B"
                </button>
                <button on:click=move |_| set_tab(2) class:selected=move || tab() == 2>
                    "Tab C"
                </button>
            </div>
            // the fallback will show initially
            // on subsequent reloads, the current child will
            // continue showing
            <Transition fallback=move || view! { <p>"Loading initial data..."</p> }>
                <p>{move || user_data.get()}</p>
            </Transition>
            {move || if user_data.loading().get() { "Hang on..." } else { "" }}
        </div>
    }
}

/// Demo the usage of create_action
/// Usually it is with button to submit something
#[component]
pub fn Demo05() -> impl IntoView {
    // an action takes an async function with single argument
    // it can be a simple type, a struct, or ()
    let add_todo = create_action(|input: &String| {
        // the input is a reference, but we need the Future to own it
        // this is important: we need to clone and move into the Future
        // so it has a 'static lifetime
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });

    // actions provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<Input>();

    view! {
        <div class="container">
            <h3 class="subtitle">Demo Action</h3>
            <form on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                add_todo.dispatch(input.value());
            }>

                <label>"What do you need to do?" <input type="text" node_ref=input_ref/></label>
                <button type="submit">"Add Todo"</button>
            </form>
            <p>{move || pending().then(|| "Loading...")}</p>
            <p>"Submitted: " <code>{move || format!("{:#?}", submitted())}</code></p>
            <p>"Pending: " <code>{move || format!("{:#?}", pending())}</code></p>
            <p>"Todo ID: " <code>{move || format!("{:#?}", todo_id())}</code></p>
        </div>
    }
}

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Think of it as a mutation: some imperative async action you run,
// whereas a resource would be some async data you load
async fn add_todo(text: &str) -> Uuid {
    _ = text;
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    // pretend this is a post ID or something
    Uuid::new_v4()
}
