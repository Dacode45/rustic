use ggez::graphics::*;
use ggez::*;
use tiled;

use sprite::*;
use storyboard::*;
use util;

#[derive(Debug)]
pub struct Map {
    // pixel location of top left of map
    pub(crate) pos: Point2,
    pub(crate) camera: Rect,

    // layer index to use
    pub(crate) layer_index: usize,
    // tileset to use
    pub(crate) tile_set: usize,

    // gid of tileset with blocking layer
    pub(crate) blocking_tile: Option<u32>,

    pub(crate) map_def: tiled::Map,
}

impl Map {
    pub fn new(map_def: tiled::Map) -> Self {
        let mut blocking_tile = None;
        for tileset in map_def.tilesets.iter() {
            if tileset.name.contains("collision") {
                blocking_tile = Some(tileset.first_gid);
            }
        }
        Map {
            pos: Point2::new(0.0, 0.0),
            camera: Rect::new(
                0.0,
                0.0,
                // 43.0 * 16.0,
                // 15.0 * 16.0,
                (map_def.width * map_def.tile_width) as f32,
                (map_def.width * map_def.tile_height) as f32,
            ),
            layer_index: 0,
            tile_set: 0,
            blocking_tile,
            map_def,
        }
    }

    /// Getters

    pub fn dimensions(&self) -> (u32, u32) {
        let m = &self.map_def;
        return (m.width, m.height);
    }

    pub fn tile_dimensions(&self) -> (u32, u32) {
        let m = &self.map_def;
        return (m.tile_width, m.tile_height);
    }

    pub fn pixel_dimensions(&self) -> (u32, u32) {
        let m = &self.map_def;
        return (m.tile_width * m.width, m.height * m.tile_height);
    }

    /// Advance Getters
    pub fn get_tile(&self, x: usize, y: usize, layer: usize) -> u32 {
        self.map_def.layers[layer].tiles[y][x]
    }

    /// converts world pixel coordinates to tile in map
    pub fn point_to_tile(&self, x: f32, y: f32) -> (usize, usize) {
        let (w, h) = self.pixel_dimensions();
        let (tw, th) = self.tile_dimensions();

        let x = util::clamp(x, self.pos.x, self.pos.y + w as f32 - 1.0);
        let y = util::clamp(y, self.pos.y, self.pos.y + h as f32 - 1.0);

        let tile_x = ((x - self.pos.x) / tw as f32).floor();
        let tile_y = ((y - self.pos.y) / th as f32).floor();

        (tile_x as usize, tile_y as usize)
    }

    pub fn get_tile_foot(&self, x: usize, y: usize) -> graphics::Point2 {
        let tile_dimensions = self.tile_dimensions();
        let x = self.pos.x + (tile_dimensions.0 as f32 * x as f32) + tile_dimensions.0 as f32 / 2.0;
        let y = self.pos.y + (tile_dimensions.1 as f32 * y as f32) + tile_dimensions.1 as f32;
        Point2::new(x, y)
    }

    pub fn get_tile_top(&self, x: usize, y: usize) -> graphics::Point2 {
        let tile_dimensions = self.tile_dimensions();
        let x = self.pos.x + (tile_dimensions.0 as f32 * x as f32) + tile_dimensions.0 as f32 / 2.0;
        let y = self.pos.y + (tile_dimensions.1 as f32 * y as f32);
        Point2::new(x, y)
    }

    /// redner helpers

    pub fn tile_draw_params(
        &self,
        uvs: &Vec<Rect>,
        tile_x: usize,
        tile_y: usize,
        tile: u32,
    ) -> graphics::DrawParam {
        let (tw, th) = self.tile_dimensions();
        let x: f32 = self.pos.x + tw as f32 * tile_x as f32;
        let y: f32 = self.pos.y + th as f32 * tile_y as f32;

        // subtract 1 because tiled indexes by 1
        let uv = uvs[(tile - 1) as usize];
        // println!("wh: {} {}", uv.left() * self.map_pixel_width, uv.right() * self.map_pixel_width);

        let mut params = graphics::DrawParam::default();
        params.src = uv;
        params.dest = Point2::new(-self.camera.left() + x, -self.camera.top() + y);
        // TODO: Figure out reason for this hack
        // have to scale otherwise it looks like tearing
        params.scale = Point2::new(1.1, 1.1);
        params
    }
}

impl SpriteComponent for Map {
    fn setup_sprite(&self, sprite: &mut Sprite) {
        // layers are made of 3 sections
        // want the index to point to a given section
        let layer_index = self.layer_index * 3;

        let (tile_left, tile_top) = self.point_to_tile(self.camera.left(), self.camera.top());
        let (tile_right, tile_bottom) =
            self.point_to_tile(self.camera.right(), self.camera.bottom());
        sprite.sprite_batch.clear();
        let mut count = 0;
        for j in tile_top..=(tile_bottom) {
            for i in tile_left..=(tile_right) {
                count += 1;
                // Get actual tile layer
                let tile = self.get_tile(i, j, layer_index);
                if tile > 0 {
                    sprite
                        .sprite_batch
                        .add(self.tile_draw_params(&sprite.uvs, i, j, tile));
                }

                // Get decoration layer tiles
                let tile = self.get_tile(i, j, layer_index + 1);
                if tile > 0 {
                    sprite
                        .sprite_batch
                        .add(self.tile_draw_params(&sprite.uvs, i, j, tile));
                }
            }
        }
    }

    /// Where to draw the sprite map at
    fn draw_sprite_at(&self) -> graphics::Point2 {
        self.pos.clone()
    }
}

pub fn load_tile_map(ctx: &mut Context, tilemap_src: &str) -> GameResult<tiled::Map> {
    let tilemap_file = ctx.filesystem.open(tilemap_src)?;
    match tiled::parse(tilemap_file) {
        Ok(map) => Ok(map),
        Err(_) => Err(GameError::from(String::from("tiled error"))),
    }
}
