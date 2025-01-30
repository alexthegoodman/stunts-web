use crate::components::project_browser::ProjectItem;
use leptos::prelude::*;
use phosphor_leptos::{Icon, IconWeight, CUBE, HEART, HORSE};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <div class="p-4 bg-blue-100">
                <h1 class="text-2xl font-bold">"Hello, Leptos with Tailwind!"</h1>
                <div class="buttons">
                    <ProjectItem
                        project_label="Generate Motion".to_string()
                        icon="brush".to_string()
                    />
                </div>
                <div class="icons">
                    <Icon icon=HORSE />
                    <Icon icon=HEART color="#AE2983" weight=IconWeight::Fill size="32px" />
                    <Icon icon=CUBE color="teal" weight=IconWeight::Duotone />
                </div>
            </div>
        </ErrorBoundary>
    }
}
