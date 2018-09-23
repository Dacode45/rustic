use ggez;
use ggez::graphics::{Point2, Rect, Vector2};
use specs;

use warmy;

use std::collections::HashMap;
use std::path;

use components::*;
use input;
use resources::*;

pub struct World {
    pub assets: warmy::Store<ggez::Context>,
    pub specs_world: specs::World,
}

impl World {
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

        let mut w = specs::World::new();
        add_basic_resources(&mut w);

        let mut the_world = Self {
            assets: store,
            specs_world: w,
        };

        the_world
    }
}
