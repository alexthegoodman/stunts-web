use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use pages::{create_project::CreateProject, projects::Projects};

// Modules
mod components;
mod fetchers;
mod helpers;
mod pages;

// Top-Level pages
use crate::pages::home::Home;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        <Title text="Stunts | Web" />

        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/projects") view=Projects />
                <Route path=path!("/create-project") view=CreateProject />
            </Routes>
        </Router>
    }
}
