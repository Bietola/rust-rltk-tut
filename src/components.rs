use rltk::{RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

