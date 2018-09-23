use ggez::graphics::Rect;
use specs::World;

pub fn add_camera_resource(world: &mut World, camera: Rect) {
    world.add_resource::<Camera>(Camera(camera));
}

pub struct Camera(pub Rect);
