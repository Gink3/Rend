/*
 *  Taylor King
 *  Nov. 2, 2020
 *
 */
pub mod tile;
pub use tile::*;

type Map = Vec<Vec<Tile>>;

pub struct Level {
    map: Map,
}

impl Level {
    pub fn make_map(MAP_HEIGHT: i32, MAP_WIDTH: i32) -> Map {
        // fill map with "unblocked" tiles
        let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    
        // place two pillars to test the map
        map[30][22] = Tile::wall();
        map[50][22] = Tile::wall();
    
        map
    }
}