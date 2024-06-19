use leptos::*;

use super::demo_async::DemoAsync;
use super::demo_basics::{BasicComponent, ComponentsAndProps};
use super::demo_control_flow::DemoControlFlow;
use super::demo_error_handling::DemoErrorHandling;
use super::demo_form_and_input::DemoFormAndInput;
use super::demo_iteration::DemoBasicIteration;
use super::demo_parent_children_communication::DemoParentChildrenCommunication;
use super::demo_reactivity::DemoReactivity;

pub mod demo_async;
pub mod demo_basics;
pub mod demo_control_flow;
pub mod demo_error_handling;
pub mod demo_form_and_input;
pub mod demo_iteration;
pub mod demo_nested_route;
pub mod demo_parent_children_communication;
pub mod demo_reactivity;

#[derive(Clone)]
pub struct LeptosDemoMenu {
    demo_name: String,
}

impl LeptosDemoMenu {
    pub fn new(demo_name: &str) -> Self {
        LeptosDemoMenu {
            demo_name: demo_name.to_string(),
        }
    }
    pub fn demo_name(self) -> String {
        self.demo_name
    }
}

#[component]
pub fn LeptosDemoMenu() -> impl IntoView {
    use leptos_router::Outlet;
    use leptos_router::A;

    // Get the context for the setter
    let menu =
        use_context::<ReadSignal<LeptosDemoMenu>>().expect("ReadSignal<LeptosDemoMenu> provided");

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

    let selected_menu_class = move |path: &str| {
        if menu().demo_name == path {
            "is-active"
        } else {
            ""
        }
    };

    view! {
        <div class="columns">
            <div class="menu column is-one-fifth">
                <p class="menu-label">Demo index</p>
                <ul class="menu-list">

                    {menu_items
                        .into_iter()
                        .map(move |(path, label)| {
                            view! {
                                <li>
                                    // Be careful about this: whenever we use signal to acheive reactivity, we need to use to use closure
                                    // This is not working:
                                    // <A class=selected_menu_class(path) href=path>
                                    // {(*label).to_string()}
                                    // </A>
                                    // Because selected_menu_class(path) give us a value not a closure!
                                    <A class=move || { selected_menu_class(path) } href=path>
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

#[component]
pub fn LeptosDemoContent() -> impl IntoView {
    // Get the context for the setter
    let setter =
        use_context::<WriteSignal<LeptosDemoMenu>>().expect("WriteSignal<LeptosDemoMenu> provided");

    // Get the route parameters
    let params = leptos_router::use_params_map();
    let demo_name =
        move || params.with(|params| params.get("demo_name").cloned().unwrap_or_default());

    create_effect(move |_| {
        setter.update(|value| {
            *value = LeptosDemoMenu::new(demo_name().as_str());
        })
    });

    let component = move || match demo_name().as_str() {
        "basic_component" => view! { <BasicComponent/> },
        "components_and_pros" => view! { <ComponentsAndProps/> },
        "demo_basic_iteration" => view! { <DemoBasicIteration/> },
        "demo_form_and_input" => view! { <DemoFormAndInput/> },
        "demo_error_handling" => view! { <DemoErrorHandling/> },
        "demo_reactivity" => view! { <DemoReactivity/> },
        "demo_parent_children_communication" => view! { <DemoParentChildrenCommunication/> },
        "demo_async" => view! { <DemoAsync/> },
        "control_flow" => view! { <DemoControlFlow/> },
        _ => view! { <MessageComponent/> },
    };

    component.into_view()
}

#[component]
pub fn MessageComponent() -> impl IntoView {
    view! { <p>"ComponentNotFound"</p> }
}
