use crate::tracing::info;
use leptos::*;

pub mod demo_async;
pub mod demo_basics;
pub mod demo_control_flow;
pub mod demo_error_handling;
pub mod demo_form_and_input;
pub mod demo_iteration;
pub mod demo_nested_route;
pub mod demo_parent_children_communication;
pub mod demo_reactivity;

#[component]
pub fn LeptosDemoMenu() -> impl IntoView {
    use leptos_router::Outlet;
    use leptos_router::A;

    use leptos_router::use_params_map;

    let params = use_params_map();
    let demo_name =
        move || params.with(|params| params.get("demo_name").cloned().unwrap_or_default());

    info!("{}", demo_name().as_str());

    view! {
        <div class="columns">
            <div class="menu column is-one-fifth">
                <p class="menu-label">Demo index</p>
                <ul class="menu-list">
                    <li>
                        <A href="basic_component">basic components</A>
                    </li>
                    <li>
                        <A href="components_and_pros">components and props</A>
                    </li>
                    <li>
                        <A href="demo_basic_iteration">basic iterator</A>
                    </li>
                    <li>
                        <A href="demo_form_and_input">form and input</A>
                    </li>
                    <li>
                        <A href="demo_error_handling">error handling</A>
                    </li>
                    <li>
                        <A href="demo_reactivity">reactivity</A>
                    </li>
                    <li>
                        <A href="demo_parent_children_communication">parent child communication</A>
                    </li>
                    <li>
                        <A href="demo_async">demo async</A>
                    </li>
                    <li>
                        <A href="control_flow">demo control flow</A>
                    </li>
                    <li>
                        <A href="demo_nested_route">demo nested route</A>
                    </li>
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
    // we can access the :id param reactively with `use_params_map`.
    use leptos_router::use_params_map;

    use super::demo_async::DemoAsync;
    use super::demo_basics::{BasicComponent, ComponentsAndProps};
    use super::demo_control_flow::DemoControlFlow;
    use super::demo_error_handling::DemoErrorHandling;
    use super::demo_form_and_input::DemoFormAndInput;
    use super::demo_iteration::DemoBasicIteration;
    use super::demo_parent_children_communication::DemoParentChildrenCommunication;
    use super::demo_reactivity::DemoReactivity;

    let params = use_params_map();
    let demo_name =
        move || params.with(|params| params.get("demo_name").cloned().unwrap_or_default());

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
