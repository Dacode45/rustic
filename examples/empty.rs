extern crate ggez;
extern crate rustic;

use rustic::application::*;
use rustic::sop::*;

fn main() {
    let mut builder = ApplicationBuilder::new("empty", "rustic");
    builder.stories(vec![
        fade_out(3.0, ggez::graphics::BLACK),
        fade_in(3.0, ggez::graphics::BLACK),
        quit_state(),
    ]);
    let app = builder.build();
    app.run();
}
