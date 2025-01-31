use std::ops::Deref;

use chrono::{DateTime, Local};
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos::suspense::Suspense;
use leptos::{server::LocalResource, *};
use leptos_use::storage::use_local_storage;
use serde::{Deserialize, Serialize};

use crate::components::project_browser::ProjectItem;
use crate::fetchers::projects::get_projects;
use crate::helpers::projects::ProjectInfo;
use crate::helpers::users::AuthToken;

#[component]
pub fn ProjectsList() -> impl IntoView {
    let (auth_state, set_auth_state, _) =
        use_local_storage::<AuthToken, JsonSerdeCodec>("auth-token");
    let projects = LocalResource::new(move || get_projects(auth_state.get().token.clone()));

    view! {
        <div>
            <Suspense fallback=move || {
                view! { <div>"Loading projects..."</div> }
            }>
                <div class="space-y-2">
                    // <For /< not needed as Suspense assures this list will be static
                    {move || {
                        projects
                            .get()
                            .map(|project_items| {
                                let project_items = project_items.deref();
                                project_items
                                    .iter()
                                    .map(|project| {

                                        view! {
                                            <ProjectItem
                                                // project_info=project.clone()
                                                project_label=project.project_name.clone()
                                                icon="folder-plus".to_string()
                                            />
                                        }
                                    })
                                    .collect_view()
                            })
                    }}
                </div>
            </Suspense>
        </div>
    }
}
