mod leptos_demo_components;

use leptos::*;
use leptos_demo_components::demo_nested_route::*;
use leptos_demo_components::*;
use leptos_router::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
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
                        // all our routes will appear inside <main>
                        <Routes>
                            <Route path="/" view=HomePageDiv/>
                            <Route
                                path="/demos"
                                view=|| {
                                    view! {
                                        <div class="bd-docs-menu">
                                            <LeptosDemoMenu/>
                                        </div>
                                    }
                                }
                            >

                                <RoutesForDemoNestedRoute/>
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
                            <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                        </Routes>
                    </div>
                </main>

            </Router>
        </div>
    }
}

#[component]
pub fn HomePageDiv() -> impl IntoView {
    view! {
        <div>
            <h2>"Home Page"</h2>
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
