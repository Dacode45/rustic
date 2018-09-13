use ggez;
use ggez::graphics::{Point2, Vector2};
use ggez_goodies::input as ginput;
use specs;

use warmy;

use std::path;

use input;

pub struct World {
    pub assets: warmy::Store<ggez::Context>,
    pub input: input::InputState,
    pub specs_world: specs::World,
}

impl World {
    pub fn new(ctx: &mut ggez::Context, resource_dir: Option<path::PathBuf>) -> Self {
        let resource_pathbuf: path::PathBuf = match resource_dir {
            Some(s) => s,
            None => ctx.filesystem.get_resources_dir().to_owned(),
        };
        info!("Setting up resource path: {:?}", resource_pathbuf);

        let opt = warmy::StoreOpt::default().set_root(resource_pathbuf);
        let store = warmy::Store::new(opt)
            .expect("Could not create asset store? Does the directory exist?");

        let w = specs::World::new();

        let mut the_world = Self {
            assets: store,
            input: ginput::InputState::new(),
            specs_world: w,
        };

        the_world
    }
}