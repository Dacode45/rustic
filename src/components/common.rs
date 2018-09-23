use ggez::graphics::*;
use specs::{Component, VecStorage, World};

#[derive(Debug, Clone)]
pub struct Position(pub Point2);

impl Default for Position {
    fn default() -> Self {
        Position(Point2::new(0.0, 0.0))
    }
}

impl Position {
    pub fn set_pos(&mut self, pos: Point2) {
        self.0 = pos;
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Default, Debug, Clone)]
pub struct TilePosition {
    pub x: usize,
    pub y: usize,
    pub layer: usize,
    pub blocking: bool,
}

impl TilePosition {
    pub fn new(x: usize, y: usize, layer: usize) -> Self {
        TilePosition {
            x,
            y,
            layer,
            blocking: false,
        }
    }
    pub fn set_tile_pos(&mut self, tile_x: usize, tile_y: usize, layer: usize) {
        self.x = tile_x;
        self.y = tile_y;
        self.layer = layer;
    }
}

impl Component for TilePosition {
    type Storage = VecStorage<Self>;
}

#[derive(Default, Debug, Clone)]
pub struct EntityRender {
    pub sprite_id: String,

    pub width: u32,
    pub height: u32,

    pub start_frame: usize,
    pub frame: usize,
}

impl Component for EntityRender {
    type Storage = VecStorage<Self>;
}
