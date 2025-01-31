use crate::components::project_browser::ProjectItem;
use crate::components::project_list::ProjectsList;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Projects() -> impl IntoView {
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
            <ProjectsList />
        </ErrorBoundary>
    }
}
