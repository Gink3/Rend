/*
 *  Taylor King
 *  Nov. 2, 2020
 *
 */

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

extern crate tcod;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;
pub mod actor;
pub mod level;
pub mod point;
use actor::*;
use level::*;
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
    pub player: Actor,
}

impl Engine {
    //TODO Render All
    //TODO Handle Keys
    pub fn new() -> Engine {
        Engine {
            root: Root::initializer()
                .font("./bin/assets/arial10x10.png", FontLayout::Tcod)
                .font_type(FontType::Greyscale)
                .size(SCREEN_WIDTH, SCREEN_HEIGHT)
                .title("Rust/libtcod tutorial")
                .init(),
            con: Offscreen::new(80,45),
            player: Actor::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        }
    }
    pub fn handle_keys(&mut self) -> bool {
        let key = self.root.wait_for_keypress(true);
        match key {
            Key {
                code: Enter,
                alt: true,
                ..
            } => {
                // Alt+Enter: toggle fullscreen
                let fullscreen = self.root.is_fullscreen();
                self.root.set_fullscreen(!fullscreen);
            }
            Key { code: Escape, .. } => return true,

            // movement keys
            Key { code: Up, .. } => self.player.y -= 1,
            Key { code: Down, .. } => self.player.y += 1,
            Key { code: Left, .. } => self.player.x -= 1,
            Key { code: Right, .. } => self.player.x += 1,

            _ => {}
        }
        false
    }
}

