use leptos::prelude::*;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use stunts_engine::animations::ObjectType;
use stunts_engine::polygon::PolygonConfig;
use stunts_engine::st_image::StImageConfig;
use stunts_engine::st_video::StVideoConfig;
use stunts_engine::text_due::TextRendererConfig;
use uuid::Uuid;

use crate::canvas_renderer::CanvasRenderer;
use crate::components::icon::CreateIcon;
use crate::editor_state::EditorState;

#[derive(Clone, PartialEq, Eq)]
pub struct Layer {
    pub instance_id: Uuid,
    pub instance_name: String,
    pub instance_kind: ObjectType,
    pub initial_layer_index: i32,
}

impl Layer {
    pub fn from_polygon_config(config: &PolygonConfig) -> Self {
        Layer {
            instance_id: config.id,
            instance_name: config.name.clone(),
            instance_kind: ObjectType::Polygon,
            initial_layer_index: config.layer,
        }
    }
    pub fn from_image_config(config: &StImageConfig) -> Self {
        Layer {
            instance_id: Uuid::from_str(&config.id).expect("Couldn't convert uuid to string"),
            instance_name: config.name.clone(),
            instance_kind: ObjectType::ImageItem,
            initial_layer_index: config.layer,
        }
    }
    pub fn from_text_config(config: &TextRendererConfig) -> Self {
        Layer {
            instance_id: config.id,
            instance_name: config.name.clone(),
            instance_kind: ObjectType::TextItem,
            initial_layer_index: config.layer,
        }
    }
    pub fn from_video_config(config: &StVideoConfig) -> Self {
        Layer {
            instance_id: Uuid::from_str(&config.id).expect("Couldn't convert uuid to string"),
            instance_name: config.name.clone(),
            instance_kind: ObjectType::VideoItem,
            initial_layer_index: config.layer,
        }
    }
}

#[component]
pub fn SortableItem<F, FB, FC>(
    // editor: RwSignal<Vec<Layer>>,
    // renderer: LocalResource<(Arc<Mutex<CanvasRenderer>>, Arc<Mutex<EditorState>>)>,
    sortable_items: RwSignal<Vec<Layer>>,
    dragger_id: RwSignal<Uuid>,
    item_id: Uuid,
    kind: ObjectType,
    layer_name: String,
    icon_name: &'static str,
    on_items_updated: F,
    on_item_duplicated: FB,
    on_item_deleted: FC,
) -> impl IntoView
where
    F: Fn() + Clone + 'static,
    FB: Fn(Uuid, ObjectType) + Clone + 'static,
    FC: Fn(Uuid, ObjectType) + Clone + 'static,
{
    view! {
        <div
            class="flex flex-row w-full justify-between items-center p-1 rounded-lg cursor-row-resize"
            draggable="true"
            on:dragstart=move |_| dragger_id.set(item_id)
            on:dragover=move |_| {
                let dragger_pos = sortable_items
                    .get()
                    .iter()
                    .position(|layer| layer.instance_id == dragger_id.get_untracked())
                    .unwrap_or(usize::MAX);
                let hover_pos = sortable_items
                    .get()
                    .iter()
                    .position(|layer| layer.instance_id == item_id)
                    .unwrap_or(usize::MAX);
                if dragger_pos != hover_pos {
                    sortable_items
                        .update(|items| {
                            if dragger_pos < items.len() && hover_pos < items.len() {
                                let item = items.remove(dragger_pos);
                                items.insert(hover_pos, item);
                            }
                        });
                }
            }
            on:dragend=move |_| on_items_updated()
        >
            <div class="flex items-center gap-2">
                <CreateIcon
                    icon=icon_name.to_string()
                    size="24px".to_string()
                />
                <span class="text-gray-800 text-xs">{layer_name}</span>
            </div>
            <div class="flex gap-2">
                <button
                    class="bg-gray-100 text-black px-1 py-1 rounded hover:bg-gray-300"
                    on:click={
                        let kind = kind.clone();

                        move |_| on_item_duplicated(item_id, kind.clone())
                    }
                >
                    <CreateIcon
                        icon="copy".to_string()
                        size="20px".to_string()
                    />
                </button>
                <button
                    class="bg-gray-100 text-black px-1 py-1 rounded hover:bg-gray-300"
                    on:click=move |_| on_item_deleted(item_id, kind.clone())
                >
                    <CreateIcon
                        icon="trash".to_string()
                        size="20px".to_string()
                    />
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn LayerPanel(
    // renderer: LocalResource<(Arc<Mutex<CanvasRenderer>>, Arc<Mutex<EditorState>>)>,
    layers: RwSignal<Vec<Layer>>,
    dragger_id: RwSignal<Uuid>,
    on_items_updated: impl Fn() + Clone + Send + Sync + 'static,
    on_item_duplicated: impl Fn(Uuid, ObjectType) + Clone + Send + Sync + 'static,
    on_item_deleted: impl Fn(Uuid, ObjectType) + Clone + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full">
            <h3 class="text-lg font-semibold mb-3">"Scene"</h3>
            <div class="space-y-2 max-h-80 overflow-y-auto">
                <For
                    each=move || layers.get()
                    key=|layer| layer.instance_id.clone()
                    children=move |layer: Layer| {
                        let icon_name = match layer.instance_kind {
                            ObjectType::Polygon => "square",
                            ObjectType::TextItem => "text",
                            ObjectType::ImageItem => "image",
                            ObjectType::VideoItem => "video",
                        };

                        view! {
                            <SortableItem
                                // renderer=renderer
                                sortable_items=layers
                                dragger_id=dragger_id
                                item_id=layer.instance_id
                                kind=layer.instance_kind
                                layer_name=layer.instance_name.clone()
                                icon_name=icon_name
                                on_items_updated=on_items_updated.clone()
                                on_item_duplicated=on_item_duplicated.clone()
                                on_item_deleted=on_item_deleted.clone()
                            />
                        }
                    }
                />
            </div>
        </div>
    }
}
