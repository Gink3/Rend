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


const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let symbol: char = '@';
    let mut engine = Engine::new();

    while !engine.root.window_closed() {
        // clear the screen of the previous frame
        engine.con.clear();

        // render the screen
        //engine.render_all();

        engine.root.flush();
        engine.root.wait_for_keypress(true);
        // handle keys and exit game if needed
        //let exit = engine.handle_keys(&mut engine);
        let exit = engine.handle_keys();
        if exit {
            break;
        }
    }
}