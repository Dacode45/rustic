use ggez::graphics::*;
use ggez::Context;
use specs::{Builder, Component, ReadStorage, RunNow, System, VecStorage, World};

use std::collections::HashMap;

use components::*;
use resources::*;
use sprite::*;

pub fn add_character(ctx: &mut Context, world: &mut World) {
    let render = EntityRender {
        sprite_id: "/dungeon/walk_cycle.png".to_owned(),

        width: 16,
        height: 24,

        start_frame: 89,
        frame: 89,
    };
    let mut anim_map: HashMap<String, Vec<usize>> = HashMap::new();
    anim_map.insert("idle".to_owned(), vec![render.start_frame]);
    anim_map.insert(Facing::Up.name().to_owned(), vec![81, 82, 83, 84]);
    anim_map.insert(Facing::Right.name().to_owned(), vec![85, 86, 87, 88]);
    anim_map.insert(Facing::Down.name().to_owned(), vec![89, 90, 91, 92]);
    anim_map.insert(Facing::Left.name().to_owned(), vec![93, 94, 95, 96]);

    let anim = Animation::new(anim_map, Facing::Right.name(), true, None);

    let position = Position(Point2::new(20.0, 30.0));
    {
        let mut s_map = world.write_resource::<SpriteMap>();
        match s_map.0.get(&render.sprite_id) {
            None => {
                let mut image = Image::new(ctx, &render.sprite_id).unwrap();
                s_map.0.insert(
                    render.sprite_id.clone(),
                    Sprite::new(image, render.width, render.height),
                );
            }
            _ => {}
        };
    }

    world
        .create_entity()
        .with(position)
        .with(TilePosition::new(10, 10, 0))
        .with(render)
        .with(anim)
        .with(CharacterController::new(Box::new(character::WaitState)))
        .build();
}
