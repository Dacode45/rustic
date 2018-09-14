extern crate ggez;
extern crate rustic;

use ggez::*;

use std::path;

use rustic::application::*;
use rustic::sop::*;
use rustic::storyboard::*;
use rustic::*;

fn main() {
    let mut builder = ApplicationBuilder::new("empty", "rustic");
    builder.stories(vec![fade_in(3.0, ggez::graphics::BLACK)]);
    let app = builder.build();
    app.run();
}
