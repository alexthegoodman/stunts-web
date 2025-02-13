use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use phosphor_leptos::{Icon, IconWeight, IconWeightData};
use wasm_bindgen_futures::spawn_local;

use crate::{
    components::icon::CreateIcon,
    helpers::{
        projects::{ProjectInfo, StoredProject},
        users::AuthToken,
    },
};

#[component]
pub fn ProjectItem(project_id: String, project_label: String, icon: String) -> impl IntoView {
    let (stored_project, set_stored_project, _) =
        use_local_storage::<StoredProject, JsonSerdeCodec>("stored-project");

    let navigate = use_navigate();

    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = {
        let navigate = navigate.clone();

        move |ev: leptos::web_sys::MouseEvent| {
            ev.prevent_default();
            set_loading.set(true);
            set_error.set(None);

            let project_id = project_id.clone();

            set_stored_project.set(StoredProject {
                project_id: project_id.clone(),
            });

            navigate(&format!("/project/{}", project_id), Default::default());

            set_loading.set(false);
        }
    };

    view! {
        <button
            class="w-64 rounded-xl flex items-center justify-start py-2 bg-white
            border-b border-gray-200 hover:bg-gray-200 hover:cursor-pointer 
            active:bg-[#edda4] transition-colors"
            disabled=loading
            on:click=on_submit
        >

            <div class="w-6 h-6 text-black mr-2">
                <CreateIcon icon=icon size="24px".to_string() />
            </div>
            <span>{project_label}</span>
        </button>
    }
}

#[component]
pub fn NavButton(label: String, icon: String, destination: String) -> impl IntoView {
    let navigate = use_navigate();

    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = {
        let navigate = navigate.clone();

        move |ev: leptos::web_sys::MouseEvent| {
            ev.prevent_default();
            set_loading.set(true);
            set_error.set(None);

            let destination = destination.clone();

            navigate(&destination, Default::default());

            set_loading.set(false);
        }
    };

    view! {
        <button
            class="w-[70px] h-[70px] flex flex-col justify-center items-center border-0 rounded-[15px]
            shadow-[0_0_15px_4px_rgba(0,0,0,0.16)] transition-colors duration-200 ease-in-out 
            hover:bg-gray-200 hover:cursor-pointer focus-visible:border-2 focus-visible:border-blue-500"
            disabled=loading
            on:click=on_submit
        >
            <div class="text-black mb-1">
                <CreateIcon icon=icon size="32px".to_string() />
            </div>
            <span class="text-xs">{label}</span>
        </button>
    }
}

#[component]
pub fn OptionButton(
    style: String,
    label: String,
    icon: String,
    mut callback: Box<dyn FnMut() -> ()>,
) -> impl IntoView {
    // let navigate = use_navigate();

    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = {
        // let navigate = navigate.clone();

        move |ev: leptos::web_sys::MouseEvent| {
            ev.prevent_default();
            // set_loading.set(true);
            // set_error.set(None);

            // set_loading.set(false);
            callback();
        }
    };

    view! {
        <button
            class="w-[60px] h-[60px] flex flex-col justify-center items-center border border-gray-400 rounded-[15px]
            transition-colors duration-200 ease-in-out hover:bg-gray-200 hover:cursor-pointer 
            focus-visible:border-2 focus-visible:border-blue-500"
            style=style
            disabled=loading
            on:click=on_submit
        >
            <div class="text-black mb-1">
                <CreateIcon icon=icon size="24px".to_string() />
            </div>
            <span class="text-xs">{label}</span>
        </button>
    }
}

use leptos_use::{storage::use_local_storage, use_debounce_fn, DebounceOptions};

#[component]
pub fn DebouncedInput(id: String, label: String, placeholder: String) -> impl IntoView {
    let (value, set_value) = signal("".to_string());
    let (debounced_value, set_debounced_value) = signal("".to_string());

    let mut debounced_fn = use_debounce_fn(
        move || {
            // do something
            set_debounced_value.set(value.get());
        },
        1000.0,
    );

    view! {
        <div class="space-y-4">
            <label for=id.clone() class="text-xs">
                {label}
            </label>
            <input
                id=id.clone()
                name=id
                placeholder=placeholder
                type="text"
                value=value
                on:input=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_value.set(new_value.clone());
                    debounced_fn();
                }
                class="border rounded px-2 py-1 w-full min-w-2 text-xs"
            />

        // <div>
        // <p>"Current value: " {value}</p>
        // <p>"Debounced value: " {debounced_value}</p>
        // </div>
        </div>
    }
}
