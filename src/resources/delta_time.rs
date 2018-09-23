use ggez::graphics::Rect;
use specs::World;

#[derive(Default)]
pub struct DeltaTime(pub f32);

pub fn add_delta_time_resource(world: &mut World) {
    world.add_resource::<DeltaTime>(DeltaTime(0.05));
}
