//! Provides components and systems to create an in game user interface.

#![doc(html_logo_url = "https://www.amethyst.rs/assets/amethyst.svg")]
#![warn(missing_docs, rust_2018_idioms, rust_2018_compatibility)]

use amethyst_assets;
use amethyst_audio;
use amethyst_core;

use amethyst_renderer;
use clipboard;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
use fnv;
use gfx;
use gfx_glyph;
use glsl_layout;
use hibitset;
#[macro_use]
extern crate log;
use ron;
#[macro_use]
extern crate serde;
use shred;
#[macro_use]
extern crate shred_derive;
use unicode_normalization;
use unicode_segmentation;
use winit;

mod action_components;
mod bundle;
mod button;
mod event;
mod font;
mod format;
mod layout;
mod pass;
mod prefab;
mod resize;
mod selection;
mod selection_order_cache;
mod text;
mod text_editing;
mod transform;

pub use self::{
    action_components::{OnUiActionImage, OnUiActionSound},
    bundle::UiBundle,
    button::{UiButton, UiButtonBuilder, UiButtonBuilderResources, UiButtonSystem},
    event::{targeted, Interactable, UiEvent, UiEventType, UiMouseSystem},
    font::{
        default::get_default_font,
        systemfont::{default_system_font, get_all_font_handles, list_system_font_families},
    },
    format::{FontAsset, FontFormat, FontHandle, OtfFormat, TtfFormat},
    layout::{Anchor, ScaleMode, Stretch, UiTransformSystem},
    pass::DrawUi,
    prefab::{
        NoCustomUi, ToNativeWidget, UiCreator, UiFormat, UiImagePrefab, UiLoader, UiLoaderSystem,
        UiPrefab, UiTextBuilder, UiTransformBuilder, UiWidget,
    },
    resize::{ResizeSystem, UiResize},
    selection::{Selectable, Selected, SelectionKeyboardSystem, SelectionMouseSystem},
    selection_order_cache::{CacheSelectionOrderSystem, CachedSelectionOrder},
    text::{LineMode, TextEditing, TextEditingMouseSystem, UiText},
    text_editing::TextEditingInputSystem,
    transform::{UiFinder, UiTransform},
};
