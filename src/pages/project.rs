use codee::string::JsonSerdeCodec;
use leptos::{logging, prelude::*};
use leptos_use::storage::use_local_storage;
use reactive_stores::Store;
use stunts_engine::animations::{BackgroundFill, Sequence};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

use crate::components::items::NavButton;
use crate::fetchers::projects::{get_single_project, update_sequences};
use crate::helpers::users::AuthToken;
use crate::helpers::utilities::{SavedState, SavedStateStoreFields};

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
    let (auth_state, set_auth_state, _) =
        use_local_storage::<AuthToken, JsonSerdeCodec>("auth-token");

    let state = expect_context::<Store<SavedState>>();

    // this gives us reactive access to the `sequences` field only
    let sequences = state.sequences();
    let timeline_state = state.timeline_state();

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

    Effect::new(move |_| {
        set_loading.set(true);

        let auth_state = auth_state.get_untracked();

        spawn_local({
            async move {
                let response = get_single_project(auth_state.token, project_id()).await;

                sequences.set(response.project.file_data.sequences);
                timeline_state.set(response.project.file_data.timeline_state);

                set_loading.set(false);
            }
        });
    });      

    let on_create_sequence = {
        let navigate = navigate.clone();

        move |ev: leptos::web_sys::MouseEvent| {
            ev.prevent_default();
            set_loading.set(true);
            set_error.set(None);

            let mut new_sequences = sequences.get();

            new_sequences.push(Sequence {
                id: Uuid::new_v4().to_string(),
                name: "New Sequence".to_string(),
                background_fill: Some(BackgroundFill::Color([200, 200, 200, 255])),
                duration_ms: 20000,
                active_polygons: Vec::new(),
                polygon_motion_paths: Vec::new(),
                active_text_items: Vec::new(),
                active_image_items: Vec::new(),
                active_video_items: Vec::new(),
            });

            sequences.set(new_sequences.clone());

            let auth_state = auth_state.get();

            spawn_local({
                // let navigate = navigate.clone();
                let new_sequences = new_sequences.clone();
    
                async move {
                    let response = update_sequences(auth_state.token, project_id(), new_sequences).await;
    
                    set_loading.set(false);
                }
            });
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
                                            disabled=loading
                                            on:click=on_create_sequence
                                        >
                                            "New Sequence"
                                        </button>
                                    </div>
                                    // Sequence List
                                    <div class="flex flex-col w-full mt-2">
                                        <For
                                            each=move || sequences.get()
                                            key=|sequence| sequence.id.clone()
                                            children=move |sequence: Sequence| {
                                                view! {
                                                    <div class="flex flex-row">
                                                        <button
                                                            class="text-xs w-full text-left p-2 rounded hover:bg-gray-200
                                                            hover:cursor-pointer active:bg-[#edda4] transition-colors"
                                                            disabled=loading
                                                        >
                                                            "Open "
                                                            {move || sequence.name.clone()}
                                                        </button>
                                                        // <button class="text-xs w-full text-left p-2 rounded hover:bg-gray-200
                                                        // hover:cursor-pointer active:bg-[#edda4] transition-colors">
                                                        // "Duplicate"
                                                        // </button>
                                                        <button
                                                            class="text-xs w-full text-left p-2 rounded hover:bg-gray-200
                                                            hover:cursor-pointer active:bg-[#edda4] transition-colors"
                                                            disabled=loading
                                                        >
                                                            "Add to Timeline"
                                                        </button>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
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
