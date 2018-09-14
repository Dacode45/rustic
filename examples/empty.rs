extern crate ggez;
extern crate rustic;

use ggez::*;

use std::path;

use rustic::sop::*;
use rustic::storyboard::*;
use rustic::*;

fn main() {
    util::setup_logger().expect("Could not set up logging");
    let mut cb = ContextBuilder::new("nothing", "ggez")
        .window_setup(conf::WindowSetup::default().title("nothing"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600));

    let cargo_path = util::cargo_path();

    if let Some(ref s) = cargo_path {
        cb = cb.add_resource_path(s);
    }

    let ctx = cb.build().unwrap();

    let state = &mut game::Game::new(cargo_path, ctx, vec![fade_in(3.0, ggez::graphics::BLACK)]);
    while !state.should_exit {
        state.update();
        state.draw();
        state.handle_events();

        ggez::timer::yield_now();
    }
}
