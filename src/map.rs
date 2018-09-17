use ggez::*;
use tiled;

use storyboard::*;

pub struct Map {
    map_def: tiled::Map,
}

impl Map {
    pub fn new(map_def: tiled::Map) -> Self {
        println!("{:?}", map_def);
        Map { map_def }
    }
}

pub fn load_tile_map(ctx: &mut Context, tilemap_src: &str) -> GameResult<tiled::Map> {
    let tilemap_file = ctx.filesystem.open(tilemap_src)?;
    match tiled::parse(tilemap_file) {
        Ok(map) => Ok(map),
        Err(_) => Err(GameError::from(String::from("tiled error"))),
    }
}
