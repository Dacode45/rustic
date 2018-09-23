use ggez;
use ggez::graphics::*;
use specs::World;

use std::collections::HashMap;

use sprite::Sprite;

pub fn add_sprite_map_resource(world: &mut World) {
    world.add_resource::<SpriteMap>(SpriteMap(HashMap::new()));
}

pub fn add_sprite_resource(
    sprite_id: &str,
    width: u32,
    height: u32,
    ctx: &mut ggez::Context,
    world: &mut World,
) {
    let mut s_map = world.write_resource::<SpriteMap>();
    match s_map.0.get(sprite_id) {
        None => {
            let mut image = Image::new(ctx, sprite_id).unwrap();
            s_map
                .0
                .insert(sprite_id.to_owned(), Sprite::new(image, width, height));
        }
        _ => {}
    };
}

pub struct SpriteMap(pub HashMap<String, Sprite>);
