extern crate ggez;
extern crate rustic;
extern crate tiled;

use ggez::graphics::Rect;

use components::animation::Facing::*;
use rustic::application::*;
use rustic::components;
use rustic::entities::*;
use rustic::resources::*;
use rustic::sop::*;
use rustic::storyboard::*;

fn main() {
    let mut builder = ApplicationBuilder::new("map", "rustic");
    builder.stories(vec![
        Story::Setup(Box::new(|ctx| {
            let state = &mut *ctx.state.borrow_mut();
            let world = &mut state.world.specs_world;
            add_sprite_map_resource(world);
            add_camera_resource(world, Rect::new(0.0, 0.0, 800.0, 600.0));
            add_map_resource(world);
            components::register_components(world);
            Story::Done("Setup".to_owned())
        })),
        create_scene("/dungeon/map_player_house.tmx"),
        // move_camera_to_tile(43, 15, 3.0),
        // quit_state(),
    ]);
    let app = builder.build();
    app.run();
}
