use specs::World;

use std::collections::HashMap;

use map::Map;

pub fn add_map_resource(world: &mut World) {
    world.add_resource::<CurrentMap>(CurrentMap::default());
    world.add_resource::<Maps>(Maps::default());
}

#[derive(Default)]
pub struct CurrentMap(pub String);
#[derive(Default)]
pub struct Maps(pub HashMap<String, Map>);
