use leptonic::prelude::*;
use leptos::{svg::view, *};
use leptos_meta::*;
use leptos_router::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::{blog::Blog, home::Home};
#[component]
pub fn container(children: Children) -> impl IntoView {
    view! {
        <div class="container">
            {children()}
        </div>
    }
}

#[component]
pub fn PageHeader() -> impl IntoView {
    view! {
        <section class="hero is-primary">
            <div class="hero-body">
                <p class="title">
                    "Ruud Erie"
                </p>
            </div>
        </section>
    }
}

#[component]
pub fn PageFooter() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="content has-text-centered">
                <p>
                    "© 2021 Ruud Salym Erie"
                </p>
            </div>
        </footer>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                // You can add a logo or brand name here
            </div>
            <div class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item" href="/">
                        "Home"
                    </a>
                    <a class="navbar-item" href="/blog">
                        "Blog"
                    </a>
                    // ... add more menu items if needed
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    view! { 
        <div class="column is-one-quarter"> // Adjust column width as needed
            <aside class="menu">
                <p class="menu-label">
                    "Sidebar" 
                </p>
                <ul class="menu-list">
                    // ... your sidebar content ...
                </ul>
            </aside>
        </div>
    }
}
#[component]
pub fn Content() -> impl IntoView {
    view! {             <Router fallback=|| {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! {
            <ErrorTemplate outside_errors/>
        }
    }>
        <Routes>
            <Route path="" view=|| view! { <Home/> }/>
            <Route path="/blog" view=|| view! { <Blog/> }/>

        </Routes>
    </Router>
}
}
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Welcome to Ruud Erie.ai "/>
        <Container>
        <PageHeader/>
        <NavBar/>
        <Content/>
        <SideBar/>
        <PageFooter/>
        </Container>
    }
}
