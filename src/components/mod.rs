use ggez::graphics::*;

use specs::*;

#[derive(Debug)]
pub struct Camera {
    pub dimensions: Rect,
}

impl Camera {
    pub fn new(dimensions: Rect) -> Self {
        Camera { dimensions }
    }
}

impl Component for Camera {
    type Storage = VecStorage<Self>;
}
