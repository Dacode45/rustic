use specs::World;

use std::collections::HashMap;

use sprite::Sprite;

pub fn add_sprite_map_resource(world: &mut World) {
    world.add_resource::<SpriteMap>(SpriteMap(HashMap::new()));
}

pub struct SpriteMap(pub HashMap<String, Sprite>);
