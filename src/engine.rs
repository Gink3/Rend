/*
 *  Taylor King
 *  Nov. 2, 2020
 *
 */

extern crate tcod;
use tcod::colors::*;
use tcod::console::*;
pub mod level;
pub mod object;
pub mod point;
use level::*;
use object::*;
use point::*;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

pub struct Engine {
    pub root: Root,
    pub con: Offscreen,
    pub MAP_WIDTH: i32,
    pub MAP_HEIGHT: i32,
}

impl Engine {
    //TODO Render All
    //TODO Handle Keys

}

