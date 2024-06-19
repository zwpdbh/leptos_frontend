mod bulma_playground;
mod leptos_demo_components;

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

    view! {
        <p>"current menu: " {move || { menu().demo_name() }}</p>
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
                                path=":demo_name"
                                view=|| {
                                    view! {
                                        <div>
                                            <LeptosDemoContent/>
                                        </div>
                                    }
                                }
                            >

                                <Route
                                    path=""
                                    view=|| {
                                        view! {
                                            <div>
                                                <LeptosDemoContent/>
                                            </div>
                                        }
                                    }
                                />

                            </Route>

                            <Route
                                path=""
                                view=|| {
                                    view! { <div>"Select a demo to see the details."</div> }
                                }
                            />

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
