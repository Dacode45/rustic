use ggez;
use specs::World;

use super::character::*;
use components::*;
use resources::*;

pub fn new_guard() -> CharacterBuilder {
    // add the resource
    let render = EntityRender {
        sprite_id: "/dungeon/walk_cycle.png".to_owned(),

        width: 16,
        height: 24,

        start_frame: 88,
        frame: 88,
    };

    let mut builder = CharacterBuilder::new();
    builder.id("guard1");
    builder.render(render);
    builder.t_pos(TilePosition::new(35, 20, 0));
    builder.anim("idle", vec![88]);
    builder.start_state(Box::new(character::WaitState));
    builder.anim(Facing::Up.name(), vec![80, 81, 82, 83]);
    builder.anim(Facing::Right.name(), vec![84, 85, 86, 87]);
    builder.anim(Facing::Down.name(), vec![88, 89, 90, 91]);
    builder.anim(Facing::Left.name(), vec![92, 93, 94, 95]);
    builder
}
