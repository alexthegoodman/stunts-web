use crate::components::project_list::ProjectsList;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// Default Home Page
#[component]
pub fn Projects() -> impl IntoView {
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
                <div class="flex flex-row gap-2 justify-between w-full">
                    <h1 class="text-lg">Projects</h1>
                    <button
                        on:click=move |ev| {
                            navigate("/create-project", Default::default());
                        }
                        class="group relative w-lg flex justify-center py-2 px-4 border border-transparent
                        text-sm font-medium rounded-md text-white stunts-gradient 
                        focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500
                        disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        Create Project
                    </button>
                </div>
                <ProjectsList />
            </div>
        </ErrorBoundary>
    }
}
