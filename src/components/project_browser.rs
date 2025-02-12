use leptos::prelude::*;
use phosphor_leptos::{Icon, IconWeight, IconWeightData};

use crate::{components::icon::CreateIcon, helpers::projects::ProjectInfo};

#[component]
pub fn ProjectItem(
    // project_info: ProjectInfo,
    // sortable_items: RwSignal<Vec<ProjectInfo>>,
    project_label: String,
    icon: String,
) -> impl IntoView {
    view! {
        <div class="w-64 rounded-xl flex items-center justify-start py-2 bg-white
        border-b border-gray-200 hover:bg-gray-200 hover:cursor-pointer 
        active:bg-[#edda4] transition-colors">
            <div class="w-6 h-6 text-black mr-2">
                <CreateIcon icon=icon size="24px".to_string() />
            </div>
            <span>{project_label}</span>
        </div>
    }
}
