/*
 *  Taylor King
 *  Nov. 2, 2020
 *
 *  Rend is a mix of Escape from Tarkov and a traditional Roguelike
 * 
*/

use tcod::colors::*;
use tcod::console::*;
pub mod libtcod;
pub use libtcod::Tcod;
// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

fn main() {

    let root = Root::initializer()
        .font("assets/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let mut tcod = Tcod { root };

    println!("Hello, world!");

}
