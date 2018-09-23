pub mod animation;
pub mod character;
pub mod common;
pub mod render;

use specs::{Component, VecStorage, World};

pub use self::animation::*;
pub use self::character::{Character, CharacterController};
pub use self::common::*;
pub use self::render::*;

pub fn register_components(world: &mut World) {
    world.register::<Animation>();
    world.register::<Position>();
    world.register::<TilePosition>();
    world.register::<EntityRender>();
    world.register::<CharacterController>();
}
