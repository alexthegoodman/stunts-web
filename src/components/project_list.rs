use chrono::{DateTime, Local};
use leptos::prelude::*;
use leptos::suspense::Suspense;
use leptos::{server::LocalResource, *};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::components::project_browser::ProjectItem;
use crate::fetchers::projects::get_projects;
use crate::helpers::projects::ProjectInfo;

#[component]
pub fn ProjectsList() -> impl IntoView {
    let projects = LocalResource::new(|| get_projects());

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
                            .as_deref()
                            .expect("Couldn't deref projects")
                            .into_iter()
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
                    }}
                </div>
            </Suspense>
        </div>
    }
}
