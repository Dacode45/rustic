extern crate ggez;
extern crate rustic;
extern crate tiled;

use rustic::application::*;
use rustic::sop::*;

fn main() {
    let mut builder = ApplicationBuilder::new("empty", "rustic");
    builder.stories(vec![create_scene("/dungeon/jail.tmx")]);
    let app = builder.build();
    app.run();
}
