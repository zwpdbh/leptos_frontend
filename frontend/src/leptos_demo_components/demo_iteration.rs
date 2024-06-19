use leptos::*;

#[component]
pub fn DemoBasicIteration() -> impl IntoView {
    let values = vec![0, 1, 2];

    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! {
        <div class="section">
            <h1 class="title">"Demo iteration: static views and dynamic views"</h1>
            <div class="container">
                <h2 class="subtitle">"Static List"</h2>
                <div class="box">
                    <p>{values.clone()}</p>
                </div>

                <div class="box">
                    // or we can wrap them in <li>
                    <p>"we can wrap them in <li>"</p>
                    <ul>
                        {values
                            .clone()
                            .into_iter()
                            .map(|n| view! { <li>{n}</li> })
                            .collect::<Vec<_>>()}
                    </ul>
                    // .collect_view() helper function that allows you to collect any iterator of T: IntoView into Vec<View>.
                    <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>

                </div>

                <div class="box">
                    <p>
                        "The fact that the list is static doesn’t mean the interface needs to be static. "
                    </p>
                    <ul>{counter_buttons}</ul>
                </div>

            </div>

            <div class="container">
                <h2 class="subtitle">"Dynamic List"</h2>
                <p>"Use this pattern if the rows in your list will change."</p>
                <div class="box">
                    <DynamicList initial_length=5/>
                </div>
            </div>
        </div>
    }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
    /// The number of counters to begin with.
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button class="button" on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=counters
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    class="button"
                                    on:click=move |_| { set_count.update(|n| *n += 1) }
                                >
                                    {count}
                                </button>
                                <button
                                    class="button"
                                    on:click=move |_| {
                                        set_counters
                                            .update(|counters| {
                                                counters
                                                    .retain(|(counter_id, (signal, _))| {
                                                        if counter_id == &id {
                                                            signal.dispose();
                                                        }
                                                        counter_id != &id
                                                    })
                                            });
                                    }
                                >

                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />

            </ul>
        </div>
    }
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn DemoComplexDataIteration() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);

    view! {
        <h1>Demo iterating over mote complex data</h1>

        // when we click, update each row,
        // doubling its value
        <button
            class="button"
            on:click=move |_| {
                set_data
                    .update(|data| {
                        for row in data {
                            row.value *= 2;
                        }
                    });
            }
        >

            "Update Values"
        </button>
        <p>"This won't work because each.value is not reactive type"</p>
        <For
            each=data
            key=|each| each.key.clone()
            children=move |each| {
                view! { <p>{each.value}</p> }
            }
        />

        // iterate over the rows and display each value
        // <For each=data key=|state| state.key.clone() let:child>
        // <p>{child.value}</p>
        // </For>
        <p>
            "Use `create_momo` to create a derived computation that only triggers a reactive update when its value has changed"
        </p>
        <For
            each=move || data().into_iter().enumerate()
            key=|(_index, state)| state.key.clone()
            children=move |(index, _)| {
                let value = create_memo(move |_| {
                    data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
                });
                view! { <p>{value}</p> }
            }
        />
    }
}
