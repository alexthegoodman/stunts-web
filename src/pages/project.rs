use leptos::prelude::*;

use crate::components::items::NavButton;

use leptos::Params;
use leptos_router::hooks::{use_params, use_query};
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct ProjectParams {
    project_id: Option<String>,
}

#[component]
pub fn Project() -> impl IntoView {
    let params = use_params::<ProjectParams>();

    let project_id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.project_id.clone())
            .unwrap_or_default()
    };

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
            <div class="p-4">
                <div class="flex flex-col gap-4">
                    <NavButton
                        label="Motion".to_string()
                        icon="brush".to_string()
                        destination=format!("/project/{}", project_id())
                    />
                    <NavButton
                        label="Settings".to_string()
                        icon="gear".to_string()
                        destination="/settings".to_string()
                    />
                </div>
            </div>
        </ErrorBoundary>
    }
}
