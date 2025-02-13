use std::sync::{Arc, Mutex};

use canvas_renderer::CanvasRenderer;
use helpers::utilities::SavedState;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use pages::{create_project::CreateProject, project::Project, projects::Projects};
use reactive_stores::Store;
use stunts_engine::{
    editor::{init_editor_with_model, Editor, Viewport},
    timelines::SavedTimelineStateConfig,
};

// Modules
mod canvas_renderer;
mod components;
mod editor_state;
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

    provide_context(Store::new(SavedState {
        sequences: Vec::new(),
        timeline_state: SavedTimelineStateConfig {
            timeline_sequences: Vec::new(),
        },
    }));

    // provide_context(renderer);

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
                <Route path=path!("/project/:project_id") view=Project />
            </Routes>
        </Router>
    }
}
