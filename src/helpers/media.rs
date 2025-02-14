use std::str::FromStr;
use std::sync::{Arc, Mutex};
use futures::future::join_all;
use log::info;
use stunts_engine::st_video::StVideo;
use stunts_engine::{
    animations::Sequence,
    editor::{Editor, Point},
    polygon::{Polygon, Stroke},
    st_image::{StImage, StImageConfig},
    st_video::{SourceData, StVideoConfig},
    text_due::{TextRenderer, TextRendererConfig},
};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

use crate::fetchers::media::fetch_image;

pub fn restore_sequence_objects(
    editor: Arc<Mutex<Editor>>,
    saved_sequences: Vec<Sequence>,
    hidden: bool,
    token: String
) {
    let saved_sequences = saved_sequences.clone();
        let editor_m = editor.clone();
        
    spawn_local(
        async move {
            info!("Spawned..."); 

            // let mut editor = editor_m.lock().unwrap();

            // saved_sequences.iter().enumerate().for_each(|(i, saved_sequence)| {
            for (i, saved_sequence) in saved_sequences.iter().enumerate() {
                
                    // let saved_sequence = saved_sequence.clone();
                    // let editor = editor_m.clone();
                    let mut editor = editor_m.lock().unwrap();

                    info!("Restoring sequence..."); 

                    saved_sequence.active_polygons.iter().for_each(|p| {

                        info!("Restoring polygon..."); 

                        let camera = editor.camera.as_ref().expect("Couldn't get camera");
                        let window_size = &camera.window_size;

                        let gpu_resources = editor
                            .gpu_resources
                            .as_ref()
                            .expect("Couldn't get gpu resources");

                        let device = &gpu_resources.device;
                        let queue = &gpu_resources.queue;

                        let mut restored_polygon = Polygon::new(
                            &window_size,
                            &device,
                            &queue,
                            &editor
                                .model_bind_group_layout
                                .as_ref()
                                .expect("Couldn't get model bind group layout"),
                            &editor
                                .group_bind_group_layout
                                .as_ref()
                                .expect("Couldn't get group bind group layout"),
                            &camera,
                            // TODO: restoring triangles or non rectangles?
                            vec![
                                Point { x: 0.0, y: 0.0 },
                                Point { x: 1.0, y: 0.0 },
                                Point { x: 1.0, y: 1.0 },
                                Point { x: 0.0, y: 1.0 },
                            ],
                            (p.dimensions.0 as f32, p.dimensions.1 as f32),
                            Point {
                                // x: random_number_800 as f32,
                                // y: random_number_450 as f32,
                                x: p.position.x as f32,
                                y: p.position.y as f32,
                            },
                            // TODO: restore rotation?
                            0.0,
                            p.border_radius as f32,
                            [
                                p.fill[0] as f32,
                                p.fill[1] as f32,
                                p.fill[2] as f32,
                                p.fill[3] as f32,
                            ],
                            Stroke {
                                thickness: p.stroke.thickness as f32,
                                fill: [
                                    p.stroke.fill[0] as f32,
                                    p.stroke.fill[1] as f32,
                                    p.stroke.fill[2] as f32,
                                    p.stroke.fill[3] as f32,
                                ],
                            },
                            -2.0,
                            p.layer.clone(),
                            p.name.clone(),
                            Uuid::from_str(&p.id).expect("Couldn't convert string to uuid"),
                            Uuid::from_str(&saved_sequence.id.clone())
                                .expect("Couldn't convert string to uuid"),
                        );

                        restored_polygon.hidden = hidden;

                        // editor.add_polygon(restored_polygon);
                        editor.polygons.push(restored_polygon);

                        println!("Polygon restored...");

                        
                    });
                    // drop(editor);

                
                    // let saved_sequence = saved_sequence.clone();
                    // let editor = editor.clone();
                    // let mut editor = editor_m.lock().unwrap();

                    saved_sequence.active_text_items.iter().for_each(|t| {

                        info!("Restoring text..."); 

                        let camera = editor.camera.as_ref().expect("Couldn't get camera");
                        let window_size = &camera.window_size;

                        let gpu_resources = editor
                            .gpu_resources
                            .as_ref()
                            .expect("Couldn't get gpu resources");

                        let device = &gpu_resources.device;
                        let queue = &gpu_resources.queue;

                        let position = Point {
                            x: 0.0 + t.position.x as f32,
                            y: 0.0 + t.position.y as f32,
                        };

                        let mut restored_text = TextRenderer::new(
                            &device,
                            &queue,
                            editor
                                .model_bind_group_layout
                                .as_ref()
                                .expect("Couldn't get model bind group layout"),
                            &editor
                                .group_bind_group_layout
                                .as_ref()
                                .expect("Couldn't get group bind group layout"),
                            editor
                                .font_manager
                                .get_font_by_name(&t.font_family)
                                .expect("Couldn't get font family"),
                            &window_size,
                            t.text.clone(),
                            TextRendererConfig {
                                id: Uuid::from_str(&t.id).expect("Couldn't convert uuid"),
                                name: t.name.clone(),
                                text: t.text.clone(),
                                font_family: t.font_family.clone(),
                                dimensions: (t.dimensions.0 as f32, t.dimensions.1 as f32),
                                position,
                                layer: t.layer.clone(),
                                color: t.color.clone(),
                                font_size: t.font_size.clone(),
                                background_fill: t.background_fill.unwrap_or([200, 200, 200, 255]),
                            },
                            Uuid::from_str(&t.id).expect("Couldn't convert string to uuid"),
                            Uuid::from_str(&saved_sequence.id.clone())
                                .expect("Couldn't convert string to uuid"),
                            camera,
                        );

                        restored_text.hidden = hidden;

                        restored_text.render_text(&device, &queue);

                        // editor.add_polygon(restored_polygon);
                        editor.text_items.push(restored_text);

                        println!("Text restored...");

                        
                    });
                    drop(editor);

                
                    // let saved_sequence = saved_sequence.clone();
                    // let editor = editor.clone();

                    // let mut editor = editor_m.lock().unwrap();
                    
                    
                        // let saved_sequence = saved_sequence.clone();
                        // let editor = editor.clone();
                    
                        info!("Restoring images...");
                        let image_futures: Vec<_> = saved_sequence.active_image_items
                            .iter()
                            .map(|i| {
                                let token = token.clone();
                                let image_config = StImageConfig { // Create config *outside* the async block
                                    id: i.id.clone(),
                                    name: i.name.clone(),
                                    dimensions: i.dimensions.clone(),
                                    url: i.url.clone(),
                                    position: Point {
                                        x: 0.0 + i.position.x as f32,
                                        y: 0.0 + i.position.y as f32,
                                    },
                                    layer: i.layer.clone(),
                                };

                                async move {  // async block *after* config creation
                                    info!("Fetching image...");
                                    let image_data = fetch_image(token.clone(), i.url.clone()).await.expect("Couldn't fetch image data");
                                    (image_data, image_config, i.id.clone(), i.url.clone())
                                }
                            })
                            .collect();

                        info!("Fetching all...");
                        let image_results = join_all(image_futures).await;

                        info!("Adding all...");
                        for (image_data, image_config, id, url) in image_results {
                            let mut editor = editor_m.lock().unwrap(); // Lock *once* *after* all fetches are done
                            let camera = editor.camera.as_ref().expect("Couldn't get camera");
                            let window_size = camera.window_size.clone(); // Clone these *outside* the loop
                            let gpu_resources = editor.gpu_resources.as_ref().expect("Couldn't get gpu resources");
                            let device = &gpu_resources.device;
                            let queue = &gpu_resources.queue;
                            let model_bind_group_layout = editor.model_bind_group_layout.as_ref().expect("Couldn't get model bind group layout").clone();
                            let group_bind_group_layout = editor.group_bind_group_layout.as_ref().expect("Couldn't get group bind group layout").clone();


                            let mut restored_image = StImage::new(
                                &device,
                                &queue,
                                url.clone(),
                                &image_data,
                                image_config,
                                &window_size,
                                &model_bind_group_layout,
                                &group_bind_group_layout,
                                -2.0,
                                id.clone(),
                                Uuid::from_str(&saved_sequence.id.clone()).expect("Couldn't convert string to uuid"),
                            );

                            restored_image.hidden = hidden;
                            editor.image_items.push(restored_image);
                            info!("Image restored...");
                            drop(editor); // Drop the lock after all images are created.

                        }
                    

                    
                
            // });
            }
            // drop(editor);
        }
    );

    // saved_sequence.active_video_items.iter().for_each(|i| {
    //     // let mut saved_mouse_path = None;
    //     let mut source_data_path = None;
    //     let mut stored_mouse_positions = None;
    //     if let Some(mouse_path) = &i.mouse_path {
    //         let mut mouse_pathbuf = Path::new(&mouse_path).to_path_buf();
    //         mouse_pathbuf.pop();
    //         source_data_path = Some(mouse_pathbuf.join("sourceData.json"));

    //         if let Ok(positions) = fs::read_to_string(mouse_path) {
    //             if let Ok(mouse_positions) = serde_json::from_str::<Vec<MousePosition>>(&positions)
    //             {
    //                 // saved_mouse_path = Some(mouse_path);
    //                 stored_mouse_positions = Some(mouse_positions);
    //             }
    //         }
    //     }

    //     let mut stored_source_data = None;
    //     if let Some(source_path) = &source_data_path {
    //         if let Ok(source_data) = fs::read_to_string(source_path) {
    //             if let Ok(data) = serde_json::from_str::<SourceData>(&source_data) {
    //                 stored_source_data = Some(data);
    //             }
    //         }
    //     }

    //     println!(
    //         "Restoring video source data... {:?} {:?}",
    //         source_data_path, stored_source_data
    //     );

    //     let position = Point {
    //         x: 0.0 + i.position.x as f32,
    //         y: 0.0 + i.position.y as f32,
    //     };

    //     let video_config = StVideoConfig {
    //         id: i.id.clone(),
    //         name: i.name.clone(),
    //         dimensions: i.dimensions.clone(),
    //         path: i.path.clone(),
    //         position,
    //         layer: i.layer.clone(),
    //         mouse_path: i.mouse_path.clone(),
    //     };

    //     let mut restored_video = StVideo::new(
    //         &device,
    //         &queue,
    //         // string to Path
    //         Path::new(&i.path),
    //         video_config,
    //         &window_size,
    //         editor
    //             .model_bind_group_layout
    //             .as_ref()
    //             .expect("Couldn't get model bind group layout"),
    //         &editor
    //             .group_bind_group_layout
    //             .as_ref()
    //             .expect("Couldn't get group bind group layout"),
    //         -2.0,
    //         i.id.clone(),
    //         Uuid::from_str(&saved_sequence.id.clone()).expect("Couldn't convert string to uuid"),
    //     );
    //     // .expect("Couldn't restore video");

    //     restored_video.hidden = hidden;

    //     // set window data from capture
    //     restored_video.source_data = stored_source_data;

    //     // set mouse positions
    //     restored_video.mouse_positions = stored_mouse_positions;

    //     // render 1 frame to provide preview image
    //     restored_video
    //         .draw_video_frame(device, queue)
    //         .expect("Couldn't draw video frame");

    //     // editor.add_polygon(restored_polygon);
    //     editor.video_items.push(restored_video);

    //     println!("Video restored...");
    // });
}
