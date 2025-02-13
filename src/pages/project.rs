use codee::string::JsonSerdeCodec;
use leptos::{logging, prelude::*};
use leptos_use::storage::use_local_storage;
use log::info;
use palette::rgb::Rgb;
use reactive_stores::Store;
use stunts_engine::animations::{BackgroundFill, Sequence};
use stunts_engine::editor::{init_editor_with_model, rgb_to_wgpu, wgpu_to_human, Point, Viewport, WindowSize};
use stunts_engine::polygon::{PolygonConfig, SavedPoint, SavedPolygonConfig, SavedStroke, Stroke};
use undo::Record;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use rand::Rng;

use crate::canvas_renderer::CanvasRenderer;
use crate::components::icon::CreateIcon;
use crate::components::items::{DebouncedInput, NavButton, OptionButton};
use crate::editor_state::EditorState;
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
    SequenceView(String),
}

#[component]
pub fn Project() -> impl IntoView {
    let renderer = LocalResource::new(
        // || (), 
        || async move {
            let viewport = Arc::new(Mutex::new(Viewport::new(
                900.0 as f32,
                450.0 as f32,
            )));
            
            let editor = Arc::new(Mutex::new(init_editor_with_model(viewport)));

            let record = Arc::new(Mutex::new(Record::new()));
            let editor_state = Arc::new(Mutex::new(EditorState::new(editor.clone(), record)));

            let mut renderer  = Arc::new(Mutex::new(CanvasRenderer::new(editor).await));

            let mut renderer_guard = renderer.lock().unwrap();

            renderer_guard.recreate_depth_view(900, 450);

            // better to start in Effect?
            info!("Begin rendering...");
            renderer_guard.begin_rendering();

            drop(renderer_guard);

            (renderer, editor_state)
        }
    );

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

    let (keyframe_count, set_keyframe_count) = signal("4".to_string());
    let (is_curved, set_is_curved) = signal(false);
    let (auto_choreograph, set_auto_choreograph) = signal(true);
    let (auto_fade, set_auto_fade) = signal(true);

    Effect::new(move |_| {
        set_loading.set(true);

        let auth_state = auth_state.get_untracked();

        let renderer = renderer.get();

        if let Some(renderer) = renderer {
            info!("Got renderer!");

            let (canvas_renderer, editor_state) = renderer.take();
            

            spawn_local({
                async move {
                    let response = get_single_project(auth_state.token, project_id()).await;
    
                    let mut editor_state = editor_state.lock().unwrap();
            
                    editor_state.record_state.saved_state = Some(response.project.file_data.clone());

                    let cloned_sequences = response.project.file_data.sequences.clone();
    
                    sequences.set(response.project.file_data.sequences);
                    timeline_state.set(response.project.file_data.timeline_state);

                    drop(editor_state);

                    let canvas_renderer = canvas_renderer.lock().unwrap();
                    let editor = canvas_renderer.editor.clone();    

                    let mut editor = editor.lock().unwrap();
                    
                    cloned_sequences.iter().enumerate().for_each(|(i, s)| {
                        editor.restore_sequence_objects(
                            &s,
                            true,
                        );
                    });

                    drop(editor);

                    set_loading.set(false);
                }
            });
        }
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

    let on_open_sequence = move |sequence_id: String| {
        set_section.set(Sections::SequenceView(sequence_id.clone()));

        println!("Open Sequence...");

        let renderer = renderer.get().expect("Couldn't get renderer");
        let (canvas_renderer, editor_state) = renderer.take();
        let canvas_renderer = canvas_renderer.lock().unwrap();
        let editor = canvas_renderer.editor.clone();

        let mut editor_state = editor_state.lock().unwrap();
        let saved_state = editor_state
            .record_state
            .saved_state
            .as_ref()
            .expect("Couldn't get Saved State");

        let saved_sequence = saved_state
            .sequences
            .iter()
            .find(|s| s.id == sequence_id.clone())
            .expect("Couldn't find matching sequence")
            .clone();

        let mut background_fill = Some(BackgroundFill::Color([
            wgpu_to_human(0.8) as i32,
            wgpu_to_human(0.8) as i32,
            wgpu_to_human(0.8) as i32,
            255,
        ]));

        if saved_sequence.background_fill.is_some() {
            background_fill = saved_sequence.background_fill.clone();
        }

        // for the background polygon and its signal
        editor_state.selected_polygon_id =
            Uuid::from_str(&saved_sequence.id)
                .expect("Couldn't convert string to uuid");

        drop(editor_state);

        println!("Opening Sequence...");

        let mut editor = editor.lock().unwrap();

        let camera = editor.camera.expect("Couldn't get camera");
        let viewport = editor.viewport.lock().unwrap();

        let window_size = WindowSize {
            width: viewport.width as u32,
            height: viewport.height as u32,
        };

        drop(viewport);

        let mut rng = rand::thread_rng();

        // set hidden to false based on sequence
        // also reset all objects to hidden=true beforehand
        editor.polygons.iter_mut().for_each(|p| {
            p.hidden = true;
        });
        editor.image_items.iter_mut().for_each(|i| {
            i.hidden = true;
        });
        editor.text_items.iter_mut().for_each(|t| {
            t.hidden = true;
        });
        editor.video_items.iter_mut().for_each(|t| {
            t.hidden = true;
        });

        saved_sequence.active_polygons.iter().for_each(|ap| {
            let polygon = editor
                .polygons
                .iter_mut()
                .find(|p| p.id.to_string() == ap.id)
                .expect("Couldn't find polygon");
            polygon.hidden = false;
        });
        saved_sequence.active_image_items.iter().for_each(|si| {
            let image = editor
                .image_items
                .iter_mut()
                .find(|i| i.id.to_string() == si.id)
                .expect("Couldn't find image");
            image.hidden = false;
        });
        saved_sequence.active_text_items.iter().for_each(|tr| {
            let text = editor
                .text_items
                .iter_mut()
                .find(|t| t.id.to_string() == tr.id)
                .expect("Couldn't find image");
            text.hidden = false;
        });
        saved_sequence.active_video_items.iter().for_each(|tr| {
            let video = editor
                .video_items
                .iter_mut()
                .find(|t| t.id.to_string() == tr.id)
                .expect("Couldn't find image");
            video.hidden = false;
        });

        match background_fill.expect("Couldn't get default background fill")
        {
            BackgroundFill::Color(fill) => {
                editor.replace_background(
                    Uuid::from_str(&saved_sequence.id)
                        .expect("Couldn't convert string to uuid"),
                    rgb_to_wgpu(
                        fill[0] as u8,
                        fill[1] as u8,
                        fill[2] as u8,
                        fill[3] as f32,
                    ),
                );
            }
            _ => {
                println!("Not supported yet...");
            }
        }

        println!("Objects restored!");

        editor.update_motion_paths(&saved_sequence);

        println!("Motion Paths restored!");

        // drop(editor);

        // selected_sequence_data.set(saved_sequence.clone());
        // selected_sequence_id.set(item.clone());
        // sequence_selected.set(true);
    };

    // Add to renderer Editor
    // Then add save to db
    // Update the context signal
    // Finally persist it in certain structs
    let on_add_square = move |sequence_id: String| {
        info!("Adding Square...");

        let renderer = renderer
            .get()
            .expect("Couldn't get renderer");
        let (canvas_renderer, editor_state) = renderer.take();
        let canvas_renderer = canvas_renderer.lock().unwrap();
        let editor_m = canvas_renderer.editor.clone();

        let mut editor = editor_m.lock().unwrap();
        
        let mut rng = rand::thread_rng();
        let random_number_800 = rng.gen_range(0..=800);
        let random_number_450 = rng.gen_range(0..=450);
        
        let new_id = Uuid::new_v4();
        
        let polygon_config = PolygonConfig {
            id: new_id.clone(),
            name: "Square".to_string(),
            points: vec![
                Point { x: 0.0, y: 0.0 },
                Point { x: 1.0, y: 0.0 },
                Point { x: 1.0, y: 1.0 },
                Point { x: 0.0, y: 1.0 },
            ],
            dimensions: (100.0, 100.0),
            position: Point {
                x: random_number_800 as f32,
                y: random_number_450 as f32,
            },
            border_radius: 0.0,
            fill: [1.0, 1.0, 1.0, 1.0],
            stroke: Stroke {
                fill: [1.0, 1.0, 1.0, 1.0],
                thickness: 2.0,
            },
            layer: -2,
        };

        editor
            .add_polygon(
                polygon_config.clone(),
                "Polygon".to_string(),
                new_id,
                sequence_id.clone(),
            );
        
        drop(editor);
        
        let mut editor_state = editor_state.lock().unwrap();
        
        editor_state
            .add_saved_polygon(
                sequence_id.clone(),
                SavedPolygonConfig {
                    id: polygon_config.id.to_string().clone(),
                    name: polygon_config.name.clone(),
                    dimensions: (
                        polygon_config.dimensions.0 as i32,
                        polygon_config.dimensions.1 as i32,
                    ),
                    fill: [
                        polygon_config.fill[0] as i32,
                        polygon_config.fill[1] as i32,
                        polygon_config.fill[2] as i32,
                        polygon_config.fill[3] as i32,
                    ],
                    border_radius: polygon_config.border_radius as i32,
                    position: SavedPoint {
                        x: polygon_config.position.x as i32,
                        y: polygon_config.position.y as i32,
                    },
                    stroke: SavedStroke {
                        thickness: polygon_config.stroke.thickness as i32,
                        fill: [
                            polygon_config.stroke.fill[0] as i32,
                            polygon_config.stroke.fill[1] as i32,
                            polygon_config.stroke.fill[2] as i32,
                            polygon_config.stroke.fill[3] as i32,
                        ],
                    },
                    layer: polygon_config.layer.clone(),
                },
            );
        
        let saved_state = editor_state
            .record_state
            .saved_state
            .as_ref()
            .expect("Couldn't get saved state");
        
        let updated_sequence = saved_state
            .sequences
            .iter()
            .find(|s| s.id == sequence_id.clone())
            .expect("Couldn't get updated sequence");
        
        let sequence_cloned = updated_sequence.clone();
        
        sequences.set(saved_state.sequences.clone());

        drop(editor_state);
        
        let mut editor = editor_m.lock().unwrap();
        
        editor.current_sequence_data = Some(
            sequence_cloned.clone(),
        );
        
        editor.update_motion_paths(&sequence_cloned);
        
        drop(editor);

        info!("Square added!");
    };

    let on_add_text = move |sequence_id: String| {};

    let on_add_image = move |sequence_id: String| {};

    let on_add_video = move |sequence_id: String| {};

    let on_open_capture = move |sequence_id: String| {};

    let aside_width = 260.0;
    let quarters = (aside_width / 4.0) + (5.0 * 4.0);
    let thirds = (aside_width / 3.0) + (5.0 * 3.0);
    let halfs = (aside_width / 2.0) + (5.0 * 2.0);

    let colors = [
        ["#FFE4E1", "#FF6B6B", "#FF0000", "#B22222", "#8B0000"], // red
        ["#FFECD9", "#FFB347", "#FF8C00", "#D95E00", "#993D00"], // orange
        ["#FFFACD", "#FFE66D", "#FFD700", "#DAA520", "#B8860B"], // yellow
        ["#E8F5E9", "#7CB342", "#2E7D32", "#1B5E20", "#0A3D0A"], // green
        ["#E3F2FD", "#64B5F6", "#1E88E5", "#1565C0", "#0D47A1"], // blue
        ["#F3E5F5", "#AB47BC", "#8E24AA", "#6A1B9A", "#4A148C"], // purple
        ["#FCE4EC", "#F06292", "#E91E63", "#C2185B", "#880E4F"], // pink
        ["#E0F2F1", "#4DB6AC", "#00897B", "#00695C", "#004D40"], // teal
        ["#EFEBE9", "#A1887F", "#795548", "#5D4037", "#3E2723"], // brown
        ["#F5F5F5", "#BDBDBD", "#757575", "#424242", "#212121"], // gray
    ];

    // 50 color / text combinations (style portion of format)
    // background_color_index, text_length, font_family_index, font_size, font_color_index
    let themes = [
        [0.0, 120.0, 12.0, 24.0, 0.4],
        [1.2, 80.0, 25.0, 32.0, 1.0],
        [2.1, 150.0, 37.0, 18.0, 2.3],
        [3.3, 200.0, 45.0, 20.0, 3.1],
        [4.4, 100.0, 50.0, 28.0, 4.0],
        [5.2, 90.0, 55.0, 22.0, 5.1],
        [6.0, 130.0, 10.0, 26.0, 6.3],
        [7.2, 110.0, 30.0, 16.0, 7.4],
        [8.1, 140.0, 40.0, 20.0, 8.3],
        [9.3, 180.0, 5.0, 18.0, 9.1],
        [0.1, 95.0, 18.0, 30.0, 0.3],
        [1.3, 110.0, 22.0, 20.0, 1.2],
        [2.2, 130.0, 35.0, 22.0, 2.4],
        [3.0, 160.0, 48.0, 18.0, 3.2],
        [4.1, 75.0, 7.0, 28.0, 4.3],
        [5.4, 140.0, 53.0, 24.0, 5.0],
        [6.2, 100.0, 14.0, 26.0, 6.1],
        [7.1, 120.0, 29.0, 20.0, 7.3],
        [8.2, 150.0, 42.0, 18.0, 8.4],
        [9.0, 200.0, 3.0, 16.0, 9.2],
        [0.3, 85.0, 20.0, 32.0, 0.2],
        [1.4, 105.0, 26.0, 24.0, 1.1],
        [2.0, 115.0, 38.0, 20.0, 2.3],
        [3.2, 170.0, 47.0, 18.0, 3.4],
        [4.2, 90.0, 9.0, 30.0, 4.1],
        [5.1, 125.0, 54.0, 22.0, 5.3],
        [6.3, 135.0, 16.0, 24.0, 6.2],
        [7.0, 145.0, 31.0, 18.0, 7.4],
        [8.3, 155.0, 43.0, 20.0, 8.1],
        [9.4, 180.0, 6.0, 16.0, 9.0],
        [0.4, 100.0, 23.0, 28.0, 0.1],
        [1.0, 115.0, 27.0, 22.0, 1.3],
        [2.3, 140.0, 39.0, 20.0, 2.2],
        [3.1, 160.0, 46.0, 18.0, 3.0],
        [4.3, 80.0, 8.0, 32.0, 4.2],
        [5.0, 130.0, 55.0, 24.0, 5.4],
        [6.1, 95.0, 15.0, 26.0, 6.4],
        [7.3, 110.0, 32.0, 20.0, 7.2],
        [8.4, 165.0, 44.0, 18.0, 8.0],
        [9.2, 190.0, 4.0, 16.0, 9.3],
    ];

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
                <div class="flex flex-row">
                    <div class="flex max-w-[315px] w-full max-h-[50vh] overflow-y-scroll overflow-x-hidden p-4 border-0 rounded-[15px] shadow-[0_0_15px_4px_rgba(0,0,0,0.16)]">
                        {move || {
                            match section.get() {
                                Sections::SequenceList => {
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
                                                                    on:click=move |_| { on_open_sequence(sequence.id.clone()) }
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
                                }
                                Sections::SequenceView(sequence_id) => {
                                    view! {
                                        <div class="flex flex-col w-full gap-4 mb-4">
                                            <div class="flex flex-row items-center">
                                                <button
                                                    class="flex flex-col justify-center items-center text-xs w-[35px] h-[35px] text-center rounded 
                                                    hover:bg-gray-200 hover:cursor-pointer active:bg-[#edda4] transition-colors mr-2"
                                                    disabled=loading
                                                    on:click=move |_| {
                                                        set_section.set(Sections::SequenceList);
                                                    }
                                                >
                                                    <CreateIcon
                                                        icon="arrow-left".to_string()
                                                        size="24px".to_string()
                                                    />
                                                </button>
                                                <h5>"Update Sequence"</h5>
                                            </div>
                                            <div class="flex flex-row gap-2">
                                                <label for="keyframe_count" class="text-xs">
                                                    "Choose keyframe count"
                                                </label>
                                                <select
                                                    id="keyframe_count"
                                                    name="keyframe_count"
                                                    class="text-xs"
                                                    on:change=move |ev| {
                                                        set_keyframe_count.set(event_target_value(&ev));
                                                    }
                                                    prop:value=keyframe_count
                                                >
                                                    <option value="4">"4"</option>
                                                    <option value="6">"6"</option>
                                                </select>
                                                <input
                                                    type="checkbox"
                                                    id="is_curved"
                                                    name="is_curved"
                                                    // checked=false
                                                    on:change=move |ev| {
                                                        set_is_curved.set(event_target_checked(&ev));
                                                    }
                                                    prop:checked=is_curved
                                                />
                                                <label for="is_curved" class="text-xs">
                                                    "Is Curved"
                                                </label>
                                            </div>
                                            <div class="flex flex-row gap-2">
                                                <input
                                                    type="checkbox"
                                                    id="auto_choreograph"
                                                    name="auto_choreograph"
                                                    // checked=true
                                                    on:change=move |ev| {
                                                        set_auto_choreograph.set(event_target_checked(&ev));
                                                    }
                                                    prop:checked=auto_choreograph
                                                />
                                                <label for="auto_choreograph" class="text-xs">
                                                    "Auto-Choreograph"
                                                </label>
                                                <input
                                                    type="checkbox"
                                                    id="auto_fade"
                                                    name="auto_fade"
                                                    // checked=true
                                                    on:change=move |ev| {
                                                        set_auto_fade.set(event_target_checked(&ev));
                                                    }
                                                    prop:checked=auto_fade
                                                />
                                                <label for="auto_fade" class="text-xs">
                                                    "Auto-Fade"
                                                </label>
                                            </div>
                                            <button
                                                type="submit"
                                                class="group relative w-full flex justify-center py-2 px-4 border border-transparent
                                                text-sm font-medium rounded-md text-white stunts-gradient 
                                                focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500
                                                disabled:opacity-50 disabled:cursor-not-allowed"
                                                disabled=loading
                                            >
                                                {move || {
                                                    if loading.get() {
                                                        "Generating..."
                                                    } else {
                                                        "Generate Animation"
                                                    }
                                                }}
                                            </button>
                                            <div class="flex flex-row flex-wrap gap-2">
                                                <OptionButton
                                                    style="".to_string()
                                                    label="Add Square".to_string()
                                                    icon="square".to_string()
                                                    callback=Box::new({
                                                        let sequence_id = sequence_id.clone();
                                                        move || {
                                                            on_add_square(sequence_id.clone());
                                                        }
                                                    })
                                                />
                                                <OptionButton
                                                    style="".to_string()
                                                    label="Add Image".to_string()
                                                    icon="image".to_string()
                                                    callback=Box::new({
                                                        let sequence_id = sequence_id.clone();
                                                        move || {
                                                            on_add_image(sequence_id.clone());
                                                        }
                                                    })
                                                />
                                                <OptionButton
                                                    style="".to_string()
                                                    label="Add Text".to_string()
                                                    icon="text".to_string()
                                                    callback=Box::new({
                                                        let sequence_id = sequence_id.clone();
                                                        move || {
                                                            on_add_text(sequence_id.clone());
                                                        }
                                                    })
                                                />
                                                <OptionButton
                                                    style="".to_string()
                                                    label="Add Video".to_string()
                                                    icon="video".to_string()
                                                    callback=Box::new({
                                                        let sequence_id = sequence_id.clone();
                                                        move || {
                                                            on_add_video(sequence_id.clone());
                                                        }
                                                    })
                                                />
                                                <OptionButton
                                                    style="".to_string()
                                                    label="Screen Capture".to_string()
                                                    icon="video".to_string()
                                                    callback=Box::new({
                                                        let sequence_id = sequence_id.clone();
                                                        move || {
                                                            on_open_capture(sequence_id.clone());
                                                        }
                                                    })
                                                />
                                            </div>
                                            <div class="flex flex-row flex-wrap gap-2">
                                                {themes
                                                    .into_iter()
                                                    .map(|theme: [f64; 5]| {
                                                        let background_color_row = theme[0].trunc() as usize;
                                                        let background_color_column = (theme[0].fract() * 10.0)
                                                            as usize;
                                                        let background_color = colors[background_color_row][background_color_column];
                                                        let background_color: Rgb<Rgb, u8> = Rgb::from_str(
                                                                &background_color,
                                                            )
                                                            .expect("Couldn't get background color");
                                                        let text_color_row = theme[4].trunc() as usize;
                                                        let text_color_column = (theme[4].fract() * 10.0) as usize;
                                                        let text_color = colors[text_color_row][text_color_column];
                                                        let text_color: Rgb<Rgb, u8> = Rgb::from_str(&text_color)
                                                            .expect("Couldn't get text color");
                                                        let font_index = theme[2];

                                                        view! {
                                                            <OptionButton
                                                                style=format!(
                                                                    "color: rgb({},{},{}); background-color: rgb({},{},{})",
                                                                    text_color.red,
                                                                    text_color.green,
                                                                    text_color.blue,
                                                                    background_color.red,
                                                                    background_color.green,
                                                                    background_color.blue,
                                                                )
                                                                label="Apply Theme".to_string()
                                                                icon="brush".to_string()
                                                                callback=Box::new(move || {
                                                                    println!("Apply Theme...");
                                                                })
                                                            />
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                            <label class="text-sm">"Background Color"</label>
                                            <div class="flex flex-row gap-2">
                                                <DebouncedInput
                                                    id="background_red".to_string()
                                                    label="Red".to_string()
                                                    placeholder="Red".to_string()
                                                />
                                                <DebouncedInput
                                                    id="background_green".to_string()
                                                    label="Green".to_string()
                                                    placeholder="Green".to_string()
                                                />
                                                <DebouncedInput
                                                    id="background_blue".to_string()
                                                    label="Blue".to_string()
                                                    placeholder="Blue".to_string()
                                                />
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                        }}
                    </div>
                    <div>
                        <canvas id="scene-canvas" class="w-[900px] h-[450px] border border-black" />
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
