use ggez::*;

use std::path;

use game::Game;
use storyboard::*;
use util;

pub struct ApplicationBuilder {
    author: &'static str,
    game_id: &'static str,
    dimensions: Option<(u32, u32)>,
    stories: Option<Vec<Story>>,
}

impl ApplicationBuilder {
    pub fn new(game_id: &'static str, author: &'static str) -> Self {
        util::setup_logger().expect("Could not set up logging");
        ApplicationBuilder {
            author: author,
            game_id: game_id,
            dimensions: None,
            stories: None,
        }
    }
    pub fn dimensions(&mut self, dimensions: (u32, u32)) {
        self.dimensions = Some(dimensions);
    }
    pub fn stories(&mut self, stories: Vec<Story>) {
        self.stories = Some(stories);
    }
    pub fn build(self) -> Application {
        let (width, height) = match self.dimensions {
            Some(dimensions) => dimensions,
            None => (800, 600),
        };
        let stories = match self.stories {
            Some(stories) => stories,
            None => panic!("No stories added"),
        };

        let mut cb = ContextBuilder::new(self.game_id, self.author)
            .window_setup(conf::WindowSetup::default().title(self.game_id))
            .window_mode(conf::WindowMode::default().dimensions(width, height));

        let cargo_path = util::cargo_path();

        if let Some(ref s) = cargo_path {
            cb = cb.add_resource_path(s);
        }

        let ctx = cb.build().unwrap();

        let game = Game::new(cargo_path, ctx, stories);
        Application { game }
    }
}

pub struct Application {
    pub game: Game,
}

impl Application {
    pub fn run(mut self) {
        while !self.game.should_exit {
            self.game.update();
            self.game.draw();
            self.game.handle_events();

            timer::yield_now();
        }
    }
}
