# How to highlight menu dyanmically

To dynamically add the `is-active` class to the menu items in your `LeptosDemoMenu` component based on the currently active route, you can follow these steps:

1. **Retrieve the current route** using `use_params_map`.
2. **Determine if the current route matches the menu item**.
3. **Conditionally apply the `is-active` class** to the `<A>` elements.

Here's an updated version of your `LeptosDemoMenu` component that implements these steps:

```rust
#[component]
#[component]
pub fn LeptosDemoMenu() -> impl IntoView {
    use leptos_router::Outlet;
    use leptos_router::A;

    use leptos_router::use_params_map;

    let params = use_params_map();
    let demo_name =
        move || params.with(|params| params.get("demo_name").cloned().unwrap_or_default());

    let menu_items: Vec<(&str, &str)> = vec![
        ("basic_component", "basic components"),
        ("components_and_pros", "components and props"),
        ("demo_basic_iteration", "basic iterator"),
        ("demo_form_and_input", "form and input"),
        ("demo_error_handling", "error handling"),
        ("demo_reactivity", "reactivity"),
        (
            "demo_parent_children_communication",
            "parent child communication",
        ),
        ("demo_async", "demo async"),
        ("control_flow", "demo control flow"),
        ("demo_nested_route", "demo nested route"),
    ];

    logging::log!("demo name: {}", demo_name().as_str());
    view! {
        <div class="columns">
            <div class="menu column is-one-fifth">
                <p class="menu-label">Demo index</p>
                <ul class="menu-list">

                    {menu_items
                        .into_iter()
                        .map(|(path, label)| {
                            let active_class: &str = if demo_name() == path {
                                "is-active"
                            } else {
                                ""
                            };
                            view! {
                                <li>
                                    <A class=active_class href=path>
                                        {(*label).to_string()}
                                    </A>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}

                </ul>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            </div>
            <div class="section column">
                <Outlet/>
            </div>
        </div>
    }
}
```

## Explanation

1. **Retrieve the current route**: The `use_params_map` hook is used to get the current route parameters.
2. **Determine if the current route matches the menu item**: The `menu_items` vector holds the menu item paths and labels. The `active_class` variable checks if the current route (`demo_name()`) matches the menu item's path.
3. **Conditionally apply the `is-active` class**: The `active_class` variable is assigned `"is-active"` if the route matches, otherwise an empty string. This class is then conditionally added to the `<A>` element.

This approach dynamically highlights the active menu item based on the current route, enhancing the navigation experience by indicating the currently selected demo.