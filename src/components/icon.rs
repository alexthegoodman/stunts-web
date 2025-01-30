use leptos::prelude::*;
use phosphor_leptos::{
    Icon, IconWeight, IconWeightData, ARROWS_CLOCKWISE, ARROWS_OUT_CARDINAL, ARROW_FAT_LINES_RIGHT,
    ARROW_LEFT, ATOM, BONE, BOOK_OPEN, BRAIN, BROADCAST, CARET_DOWN, CARET_RIGHT, CIRCLES_THREE,
    COPY, CUBE_FOCUS, DOTS_THREE_OUTLINE_VERTICAL, DOT_OUTLINE, FADERS, FAST_FORWARD, FOLDER_PLUS,
    GEAR, HORSE, IMAGE, MAP_TRIFOLD, MINUS, OCTAGON, PAINT_BRUSH, PANORAMA, PLUS, POLYGON, RESIZE,
    SHAPES, SPEEDOMETER, SPHERE, SQUARE, TEXT_T, TRASH, TRIANGLE, VECTOR_THREE, VIDEO, WINDMILL, X,
};

use crate::helpers::projects::ProjectInfo;

#[component]
pub fn CreateIcon(icon: String, size: String) -> impl IntoView {
    match icon.as_str() {
        "plus" => {
            view! { <Icon icon=PLUS weight=IconWeight::Light size=size /> }
        }
        "minus" => {
            view! { <Icon icon=MINUS weight=IconWeight::Light size=size /> }
        }
        "windmill" => {
            view! { <Icon icon=WINDMILL weight=IconWeight::Light size=size /> }
        }
        "gear" => {
            view! { <Icon icon=GEAR weight=IconWeight::Light size=size /> }
        }
        "brush" => {
            view! { <Icon icon=PAINT_BRUSH weight=IconWeight::Light size=size /> }
        }
        "shapes" => {
            view! { <Icon icon=SHAPES weight=IconWeight::Light size=size /> }
        }
        "arrow-left" => {
            view! { <Icon icon=ARROW_LEFT weight=IconWeight::Light size=size /> }
        }
        "polygon" => {
            view! { <Icon icon=POLYGON weight=IconWeight::Light size=size /> }
        }
        "octagon" => {
            view! { <Icon icon=OCTAGON weight=IconWeight::Light size=size /> }
        }
        "square" => {
            view! { <Icon icon=SQUARE weight=IconWeight::Light size=size /> }
        }
        "triangle" => {
            view! { <Icon icon=TRIANGLE weight=IconWeight::Light size=size /> }
        }
        "dot" => {
            view! { <Icon icon=DOT_OUTLINE weight=IconWeight::Light size=size /> }
        }
        "dots-vertical" => {
            view! { <Icon icon=DOTS_THREE_OUTLINE_VERTICAL weight=IconWeight::Light size=size /> }
        }
        "sphere" => {
            view! { <Icon icon=SPHERE weight=IconWeight::Light size=size /> }
        }
        "gizmo" => {
            view! { <Icon icon=VECTOR_THREE weight=IconWeight::Light size=size /> }
        }
        "book" => {
            view! { <Icon icon=BOOK_OPEN weight=IconWeight::Light size=size /> }
        }
        "cube" => {
            view! { <Icon icon=CUBE_FOCUS weight=IconWeight::Light size=size /> }
        }
        "faders" => {
            view! { <Icon icon=FADERS weight=IconWeight::Light size=size /> }
        }
        "map" => {
            view! { <Icon icon=MAP_TRIFOLD weight=IconWeight::Light size=size /> }
        }
        "panorama" => {
            view! { <Icon icon=PANORAMA weight=IconWeight::Light size=size /> }
        }
        "speedometer" => {
            view! { <Icon icon=SPEEDOMETER weight=IconWeight::Light size=size /> }
        }
        "motion-arrow" => {
            view! { <Icon icon=ARROW_FAT_LINES_RIGHT weight=IconWeight::Light size=size /> }
        }
        "atom" => {
            view! { <Icon icon=ATOM weight=IconWeight::Light size=size /> }
        }
        "brain" => {
            view! { <Icon icon=BRAIN weight=IconWeight::Light size=size /> }
        }
        "broadcast" => {
            view! { <Icon icon=BROADCAST weight=IconWeight::Light size=size /> }
        }
        "circles" => {
            view! { <Icon icon=CIRCLES_THREE weight=IconWeight::Light size=size /> }
        }
        "fast-forward" => {
            view! { <Icon icon=FAST_FORWARD weight=IconWeight::Light size=size /> }
        }
        "folder-plus" => {
            view! { <Icon icon=FOLDER_PLUS weight=IconWeight::Light size=size /> }
        }
        "bone" => {
            view! { <Icon icon=BONE weight=IconWeight::Light size=size /> }
        }
        "caret-down" => {
            view! { <Icon icon=CARET_DOWN weight=IconWeight::Light size=size /> }
        }
        "caret-right" => {
            view! { <Icon icon=CARET_RIGHT weight=IconWeight::Light size=size /> }
        }
        "translate" => {
            view! { <Icon icon=ARROWS_OUT_CARDINAL weight=IconWeight::Light size=size /> }
        }
        "rotate" => {
            view! { <Icon icon=ARROWS_CLOCKWISE weight=IconWeight::Light size=size /> }
        }
        "scale" => {
            view! { <Icon icon=RESIZE weight=IconWeight::Light size=size /> }
        }
        "image" => {
            view! { <Icon icon=IMAGE weight=IconWeight::Light size=size /> }
        }
        "text" => {
            view! { <Icon icon=TEXT_T weight=IconWeight::Light size=size /> }
        }
        "video" => {
            view! { <Icon icon=VIDEO weight=IconWeight::Light size=size /> }
        }
        "copy" => {
            view! { <Icon icon=COPY weight=IconWeight::Light size=size /> }
        }
        "trash" => {
            view! { <Icon icon=TRASH weight=IconWeight::Light size=size /> }
        }
        "x" => {
            view! { <Icon icon=X weight=IconWeight::Light size=size /> }
        }
        _ => {
            view! { <Icon icon=HORSE weight=IconWeight::Light size=size /> }
        }
    }
}
