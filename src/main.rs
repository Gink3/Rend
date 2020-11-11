/*
 *  Taylor King
 *  Nov. 2, 2020
 *
 *  Rend is a mix of Escape from Tarkov and a traditional Roguelike
 * 
*/

// This file is generated automatically. Do not edit it directly.
// See the Contributing section in README on how to make changes to it.
use tcod::colors::*;
use tcod::console::*;
pub mod engine;
pub use engine::*;
pub use engine::object::*;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// size of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("./bin/assets/arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let symbol: char = '@';
    let player = Object::new(MAP_WIDTH / 2, MAP_HEIGHT / 2, symbol);

    let mut engine = Engine { root, con, MAP_WIDTH, MAP_HEIGHT };


    // // create object representing the player
    // let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    // // create an NPC
    // let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

    // the list of objects with those two
    //let mut objects = [player, npc];

    while !engine.root.window_closed() {
        // clear the screen of the previous frame
        engine.con.clear();

        // render the screen
        //engine.render_all();

        engine.root.flush();

        // handle keys and exit game if needed
        //let exit = engine.handle_keys(&mut engine);
        let exit = true;
        if exit {
            break;
        }
    }
}