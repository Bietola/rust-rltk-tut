use rltk::{RGB};
use specs::prelude::*;
use specs_derive::Component;
use rltk::Point;

#[derive(Component)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl From<Point> for Pos {
    fn from(p: Point) -> Self {
        Pos { x: p.x, y: p.y }
    }
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Viewshed {
            range,
            ..Default::default()
        }
    }
}
