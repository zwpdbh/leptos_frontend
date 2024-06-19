mod bulma_playground;
mod leptos_demo_components;

use crate::demo_nested_route::RoutesForDemoNestedRoute;
use bulma_playground::*;
use leptos::*;
use leptos_demo_components::*;
use leptos_router::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (menu, set_menu) = create_signal(LeptosDemoMenu::new(""));
    provide_context(set_menu);
    provide_context(menu);

    let read_demo_name = move || menu().demo_name;
    view! {
        <Router>
            <header>
                <nav class="navbar" role="navigation" aria-label="main navigation">
                    <div class="navbar-brand"></div>

                    <div id="navbarBasicExample" class="navbar-menu">
                        <div class="navbar-start">
                            <a class="navbar-item" href="/">
                                Home
                            </a>

                            <a class="navbar-item" href="/demos">
                                Leptos frontend demos
                            </a>

                            <div class="navbar-item has-dropdown is-hoverable">
                                <a class="navbar-link">More</a>

                                <div class="navbar-dropdown">
                                    <a class="navbar-item">About</a>
                                    <a class="navbar-item is-selected">Jobs</a>
                                    <a class="navbar-item">Contact</a>
                                    <hr class="navbar-divider"/>
                                    <a class="navbar-item">Report an issue</a>
                                </div>
                            </div>
                        </div>

                        <div class="navbar-end">
                            <div class="navbar-item">
                                <div class="buttons">
                                    <a class="button is-primary">
                                        <strong>Sign up</strong>
                                    </a>
                                    <a class="button is-light">Log in</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </nav>
            </header>
            <main>
                <div class="bd-docs">
                    <Routes>
                        <Route path="/" view=HomePageDiv/>
                        <Route
                            path="/demos"
                            view=move || {
                                view! {
                                    <div class="bd-docs-menu">
                                        <LeptosDemoMenu/>
                                    </div>
                                }
                            }
                        >

                            <Route
                                path=""
                                view=|| {
                                    let setter = use_context::<WriteSignal<LeptosDemoMenu>>()
                                        .expect("WriteSignal<LeptosDemoMenu> provided");
                                    setter
                                        .update(|value| {
                                            *value = LeptosDemoMenu::new("");
                                        });
                                    view! {
                                        <div>
                                            <p>"This is the default view for /demos"</p>
                                            <p>"Select a demo to see detail"</p>
                                        </div>
                                    }
                                }
                            />

                            <Route
                                path=":demo_name"
                                view=|| {
                                    view! {
                                        <div>
                                            // This is needed because "/demos/xxx" matches two parts:
                                            // 1) is the /demos which shows the LeptosDemoMenu.
                                            // 2) is the /demos/xxx which is LeptosDemoContent
                                            <LeptosDemoContent/>
                                        </div>
                                    }
                                }
                            >

                                <Route
                                    path=""
                                    view=move || {
                                        view! {
                                            <Show
                                                when=move || {
                                                    read_demo_name().as_str() == "demo_nested_route"
                                                }

                                                fallback=|| {
                                                    view! {
                                                        <div class="box">
                                                            <p class="subtitle">"default view for other sub-routes"</p>
                                                        </div>
                                                    }
                                                }
                                            >

                                                <div class="box">
                                                    <p class="subtitle">
                                                        "default view for demos/demo_nested_route"
                                                    </p>
                                                    <p>"select one to go further route"</p>
                                                </div>
                                            </Show>
                                        }
                                    }
                                />

                                <Route
                                    path="home"
                                    view=|| {
                                        view! {
                                            <div class="container box">
                                                <h3>"Nested Route Home"</h3>
                                            </div>
                                        }
                                    }
                                />

                                <Route path="contacts" view=demo_nested_route::ContactList>
                                    // if no id specified, fall back
                                    <Route path=":id" view=demo_nested_route::ContactInfo>
                                        <Route
                                            path=""
                                            view=|| view! { <div class="tab">"(Contact Info)"</div> }
                                        />
                                        <Route
                                            path="conversations"
                                            view=|| view! { <div class="tab">"(Conversations)"</div> }
                                        />
                                    </Route>
                                    // if no id specified, fall back
                                    <Route
                                        path=""
                                        view=|| {
                                            view! {
                                                <div class="select-user">
                                                    "Select a user to view contact info."
                                                </div>
                                            }
                                        }
                                    />

                                </Route>
                                <Route path="form_example" view=demo_nested_route::FormExample/>

                                <RoutesForDemoNestedRoute/>
                            </Route>

                        </Route>
                        <Route path="/*any" view=|| view! { <h1>"Route Not Found"</h1> }/>
                    </Routes>
                // all our routes will appear inside <main>

                </div>
            </main>
        </Router>
    }
}

#[component]
pub fn HomePageDiv() -> impl IntoView {
    view! {
        <div class="section">

            <h2 class="title">"Home Page"</h2>
            <BulmaPlayground/>
        </div>
    }
}

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand"></div>

            <div id="navbarBasicExample" class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item">Home</a>

                    <a class="navbar-item">Documentation</a>

                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">More</a>

                        <div class="navbar-dropdown">
                            <a class="navbar-item">About</a>
                            <a class="navbar-item is-selected">Jobs</a>
                            <a class="navbar-item">Contact</a>
                            <hr class="navbar-divider"/>
                            <a class="navbar-item">Report an issue</a>
                        </div>
                    </div>
                </div>

                <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="buttons">
                            <a class="button is-primary">
                                <strong>Sign up</strong>
                            </a>
                            <a class="button is-light">Log in</a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
