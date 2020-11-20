
pub struct Actor {
    pub x: i32,
    pub y: i32,
    pub sym: char,
    pub health: i32, 
}

impl Actor {
    pub fn new(actor_x: i32, actor_y: i32) -> Actor {
        Actor {
            x: actor_x,
            y: actor_y,
            sym: '@',
            health: 10
        }
    }
}
