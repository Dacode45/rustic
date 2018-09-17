extern crate ggez;
extern crate rustic;
extern crate tiled;

use rustic::application::*;
use rustic::sop::*;

fn main() {
    let mut builder = ApplicationBuilder::new("empty", "rustic");
    builder.stories(vec![
        create_scene("/dungeon/map_jail.tmx"),
        move_camera_to_tile(43, 15, 3.0),
    ]);
    let app = builder.build();
    app.run();
}
