use ggez::graphics;

use components::*;
use sprite::*;

#[derive(Debug)]
pub struct Renderable {
    pub pos: Position,
    pub render: EntityRender,
}

impl Renderable {
    pub fn new(pos: Position, render: EntityRender) -> Self {
        Renderable { pos, render }
    }
}

impl SpriteComponent for Renderable {
    fn setup_sprite(&self, sprite: &mut Sprite) {
        if self.render.frame != sprite.frame() {
            sprite.set_frame(self.render.frame);
        }
        sprite.sprite_batch.clear();
        let mut params = graphics::DrawParam::default();
        params.src = sprite.uvs[sprite.frame()];
        sprite.sprite_batch.add(params);
    }
    fn draw_sprite_at(&self) -> graphics::Point2 {
        self.pos.0.clone()
    }
}
