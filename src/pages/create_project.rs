use crate::components::project_form::ProjectForm;
use crate::components::project_list::ProjectsList;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// Default Home Page
#[component]
pub fn CreateProject() -> impl IntoView {
    let navigate = use_navigate();

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
            <div class="container mx-auto py-4">
                <ProjectForm />
            </div>
        </ErrorBoundary>
    }
}
