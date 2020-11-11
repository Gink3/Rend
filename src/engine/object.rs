/*
 *  Taylor King
 *  Nov. 2, 2020
 *
 */

use tcod::colors::*;
use tcod::console::*;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    symbol: char,
}

impl Object {
    pub fn new(x: i32, y: i32, symbol: char) -> Self {
        Object { x, y, symbol}
    }
    //TODO Draw objects
    
}

