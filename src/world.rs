use ggez;
use ggez::graphics::{Point2, Rect, Vector2};
use ggez_goodies::input as ginput;
use specs;

use warmy;

use std::collections::HashMap;
use std::path;

use components::*;
use input;
use resources::*;

pub struct World {
    pub assets: warmy::Store<ggez::Context>,
    pub input: input::InputState,
    pub specs_world: specs::World,
}

impl World {
    pub fn setup(&mut self) {
        // setup spritemap
        self.specs_world
            .add_resource::<SpriteMap>(SpriteMap(HashMap::new()));
        self.specs_world
            .add_resource::<Camera>(Camera(Rect::new(0.0, 0.0, 800.0, 800.0)))
        // setup camera
    }
    pub fn new(ctx: &mut ggez::Context, resource_dir: Option<path::PathBuf>) -> Self {
        let resource_pathbuf: path::PathBuf = match resource_dir {
            Some(s) => s,
            None => ctx.filesystem.get_resources_dir().to_owned(),
        };
        info!("Setting up resource path: {:?}", resource_pathbuf);
        ctx.filesystem.log_all();

        let opt = warmy::StoreOpt::default().set_root(resource_pathbuf);
        let store = warmy::Store::new(opt)
            .expect("Could not create asset store? Does the directory exist?");

        let w = specs::World::new();

        let mut the_world = Self {
            assets: store,
            input: ginput::InputState::new(),
            specs_world: w,
        };

        the_world.setup();

        the_world
    }
}
