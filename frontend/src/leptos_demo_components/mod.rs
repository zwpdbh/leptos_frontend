use leptos::*;
use leptos_router::Params;

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
pub struct LeptosMenu {
    demo_name: String,
}

impl LeptosMenu {
    pub fn new(demo_name: &str) -> Self {
        LeptosMenu {
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

    let (menu, set_menu) = create_signal(LeptosMenu {
        demo_name: "".to_string(),
    });
    provide_context(set_menu);

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

    view! {
        <div class="columns">
            <div class="menu column is-one-fifth">
                <p class="menu-label">Demo index</p>
                <ul class="menu-list">

                    {menu_items
                        .into_iter()
                        .map(move |(path, label)| {
                            logging::log!(
                                "LeptosDemoMenu is selected to be => demo name: {}", menu.get()
                                .demo_name
                            );
                            let menu = menu();
                            let active_class: &str = if menu.demo_name == path {
                                "is-active"
                            } else {
                                ""
                            };
                            view! {
                                // let active_class = move || {
                                // if leptos_menu.demo_name == path { "is-active" } else { "" }
                                // };
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

#[component]
pub fn LeptosDemoContent() -> impl IntoView {
    // Get the context for the setter
    let setter =
        use_context::<WriteSignal<LeptosMenu>>().expect("WriteSignal<LeptosMenu> provided");

    // Get the route parameters
    let params = leptos_router::use_params_map();
    let demo_name =
        move || params.with(|params| params.get("demo_name").cloned().unwrap_or_default());

    // Update the setter when the demo name changes
    create_effect(move |_| {
        setter.update(|value| {
            *value = LeptosMenu::new(demo_name().as_str());
            logging::log!("LeptosDemoContent changed => {}", demo_name())
        });
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
