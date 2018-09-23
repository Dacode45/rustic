use specs::World;

pub mod camera;
pub mod delta_time;
pub mod input_state;
pub mod maps;
pub mod sprite_map;

pub use self::camera::*;
pub use self::delta_time::*;
pub use self::input_state::*;
pub use self::maps::*;
pub use self::sprite_map::*;

pub fn add_basic_resources(world: &mut World) {
    add_delta_time_resource(world);
    add_input_resource(world);
}
