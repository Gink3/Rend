/*
 *  Taylor King
 *  Nov. 2, 2020
 *
 */

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    blocked: bool,
    blocked_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            blocked_sight: false,
        }
    }
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            blocked_sight: true,
        }
    }
}