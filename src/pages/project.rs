use leptos::prelude::*;

use crate::components::items::NavButton;

use leptos::Params;
use leptos_router::hooks::{use_navigate, use_params, use_query};
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct ProjectParams {
    project_id: Option<String>,
}

#[derive(Clone, PartialEq)]
enum Sections {
    SequenceList,
    SequenceView,
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

    let (section, set_section) = signal(Sections::SequenceList);

    let navigate = use_navigate();

    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_create_sequence = {
        let navigate = navigate.clone();

        move |ev: leptos::web_sys::MouseEvent| {
            ev.prevent_default();
            set_loading.set(true);
            set_error.set(None);

            // let destination = destination.clone();

            // navigate(&destination, Default::default());

            // set_loading.set(false);
        }
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
            <div class="flex flex-row p-4">
                <div class="flex flex-col gap-4 mr-4">
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
                <div class="flex max-w-[300px] w-full p-4 border-0 rounded-[15px] shadow-[0_0_15px_4px_rgba(0,0,0,0.16)]">
                    {move || {
                        if section.get() == Sections::SequenceList {
                            view! {
                                <div class="flex flex-col w-full">
                                    <div class="flex flex-row justify-between align-center w-full">
                                        <h5>"Sequences"</h5>
                                        <button
                                            class="text-xs rounded-md text-white stunts-gradient px-2 py-1"
                                            on:click=on_create_sequence
                                        >
                                            "New Sequence"
                                        </button>
                                    </div>
                                    // Sequence List
                                    <div class="flex flex-col w-full"></div>
                                </div>
                            }
                                .into_any()
                        } else if section.get() == Sections::SequenceView {
                            view! {
                                <div class="flex flex-col w-full">
                                    <h5>"Update Sequence"</h5>
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div class="flex flex-col w-full">
                                    <h5>"Not implemented"</h5>
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </div>
        </ErrorBoundary>
    }
}
