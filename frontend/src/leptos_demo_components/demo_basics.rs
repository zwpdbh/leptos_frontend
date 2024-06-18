use leptos::*;

#[component]
pub fn BasicComponent() -> impl IntoView {
    view! {
        <div class="section"></div>

        <h1 class="title">"Basic Component (3.1 and 3.2)"</h1>
        <Counter/>
        <DynamicAttributes/>
    }
}

/// For shows examples from 3.1
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="container">
            <h1 class="subtitle">Simple click button</h1>
            <button
                class="button"
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
            >

                "Click me: "
                {move || count()}
            </button>
        </div>
    }
}

/// For shows examples from 3.2
#[component]
pub fn DynamicAttributes() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);

    let double_count = move || count() * 2;
    view! {
        <h2 class="subtitle">Dynamic Classes</h2>

        <p>click the below button to change the progress bar</p>
        <ul>
            <li>
                <button
                    class="button"
                    on:click=move |_| {
                        set_count.update(|n| *n += 1);
                    }

                    class:red=move || count() % 2 == 1
                    class=("button-20", move || count() % 2 == 1)
                >

                    "Click me to change progress bar: "
                    {move || count}
                </button>
            </li>
        </ul>

        <h2 class="subtitle">Dynamic Sytle</h2>
        <ul>

            <li>
                <button
                    class="button"
                    on:click=move |_| {
                        set_x.update(|n| *n += 50);
                    }

                    // set the `style` attribute
                    style="position: relative"
                    // and toggle individual CSS properties with `style:`
                    style:left=move || format!("{}px", x() + 100)
                    style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
                    style:max-width="400px"
                    // Set a CSS variable for stylesheet use
                    style=("--columns", x)
                >
                    "Click to Move"
                </button>
            </li>

        </ul>

        <h2 class="subtitle">Dynamic Attributes</h2>
        <ul>
            <li>
                <progress
                    max="50"
                    // signals are functions, so `value=count` and `value=move || count.get()`
                    // are interchangeable.
                    value=count
                ></progress>
            </li>

        </ul>

        <h3 class="subtitle">Derived Signals</h3>
        <p>"It shows a signal could be derived from another signal "</p>
        <ul>
            <li>
                <progress
                    max="50"
                    // we use it once here
                    value=double_count
                ></progress>

            </li>
            <li>
                <p>
                    // and again here
                    "Double Count: " {double_count}
                </p>
            </li>
        </ul>
    }
}

#[component]
pub fn ComponentsAndProps() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <div class="section">
            <h1 class="title">Components And Props</h1>

            // now we use our component!
            <ul>
                <p>The following example shows how we pass the progress prop</p>
                <button
                    class="button"
                    on:click=move |_| {
                        set_count.update(|n| *n += 1);
                    }
                >
                    "Click me"
                </button>

                <li>
                    <ProgressBarV1 progress=count/>
                </li>
                <li>
                    <ProgressBarV1 progress=double_count/>
                </li>
                <p>"Show the use of #[prop(into)]"</p>
                <li>
                    <ProgressBarV2 progress=count/>
                </li>
                <li>
                    <ProgressBarV2 progress=Signal::derive(double_count)/>
                </li>
            </ul>
        </div>
    }
}

/// If a component property that will change over time, we need to pass the prop as signal.
#[component]
pub fn ProgressBarV1<F>(
    // mark this prop optional
    // you can specify it or not when you use <ProgressBar/>
    #[prop(default = 100)] max: u16,
    progress: F,
) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! { <progress max=max value=progress></progress> }
}

/// "#[prop(into)]"" can be useful when defining APIs for components you’ll want to reuse while passing different sorts of signals.
#[component]
fn ProgressBarV2(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}
