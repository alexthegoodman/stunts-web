use codee::string::{FromToStringCodec, JsonSerdeCodec};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};
use stunts_engine::timelines::SavedTimelineStateConfig;
use wasm_bindgen_futures::spawn_local;

use crate::{fetchers::{projects::create_project, users::login_user}, helpers::{users::AuthToken, utilities::SavedState}};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

#[component]
pub fn ProjectForm() -> impl IntoView {
    let navigate = use_navigate();
    let (auth_state, set_auth_state, _) =
        use_local_storage::<AuthToken, JsonSerdeCodec>("auth-token");

    let (project_name, set_project_name) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = {
        let navigate = navigate.clone();

        move |ev: leptos::web_sys::SubmitEvent| {
            ev.prevent_default();
            set_loading.set(true);
            set_error.set(None);

            let auth_state = auth_state.get();
            let project_name = project_name.get();

            spawn_local({
                let navigate = navigate.clone();

                async move {
                    let saved_state = SavedState {
                        // id: project_id,
                        sequences: Vec::new(),
                        timeline_state: SavedTimelineStateConfig {
                            timeline_sequences: Vec::new(),
                        },
                    };

                    let response = create_project(auth_state.token, project_name, saved_state).await;

                    set_loading.set(false);

                    navigate("/projects", Default::default());
                }
            });
        }
    };

    view! {
        <div class="flex items-center justify-center bg-gray-50 py-8 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Create a new project"
                    </h2>
                </div>
                <form class="mt-8 space-y-6" on:submit=on_submit>
                    <div class="rounded-md shadow-sm space-y-4">
                        <div>
                            <label for="email" class="sr-only">
                                "Project name"
                            </label>
                            <input
                                id="project_name"
                                name="project_name"
                                type="text"
                                required
                                class="appearance-none rounded-md relative block w-full px-3 py-2 border
                                border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none 
                                focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Project name"
                                on:input=move |ev| {
                                    set_project_name.set(event_target_value(&ev));
                                }
                                prop:value=project_name
                            />
                        </div>
                    </div>

                    // {move || {
                    // error()
                    // .map(|err| view! { <div class="text-red-500 text-sm mt-2">{err}</div> })
                    // }}

                    <div>
                        <button
                            type="submit"
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent
                            text-sm font-medium rounded-md text-white stunts-gradient 
                            focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500
                            disabled:opacity-50 disabled:cursor-not-allowed"
                            disabled=loading
                        >
                            {move || {
                                if loading.get() { "Creating Project..." } else { "Create Project" }
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
