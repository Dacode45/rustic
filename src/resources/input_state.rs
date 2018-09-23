use ggez::graphics::Rect;
use specs::World;

use input::Input;

pub fn add_input_resource(world: &mut World) {
    world.add_resource::<Input>(Input::default());
}
