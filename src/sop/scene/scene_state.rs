use ggez::graphics;
use ggez::graphics::*;

use std::cell::RefCell;
use std::path;
use std::rc::Rc;

use map::*;
use resources::*;
use sprite::*;
use state::*;
use storyboard::*;

pub fn create_scene(tilemap_src: &'static str) -> Story {
    let tilemap_src = tilemap_src.to_owned();
    return Story::Setup(Box::new(move |ctx| {
        let state = &mut *ctx.state.borrow_mut();
        let ctx = &mut *ctx.ctx.borrow_mut();
        let map_def = load_tile_map(ctx, tilemap_src.as_ref()).unwrap();

        {
            let src = path::Path::new(&tilemap_src);
            let dir = src.parent().unwrap();
            let mut s_map = state.world.specs_world.write_resource::<SpriteMap>();

            for tileset in map_def.tilesets.iter() {
                for image in tileset.images.iter() {
                    let i_src = dir.join(&image.source);
                    let i = Image::new(ctx, i_src.to_str().unwrap()).unwrap();
                    let sprite = Sprite::new(i, tileset.tile_width, tileset.tile_height);
                    s_map.0.insert(image.source.clone(), sprite);
                }
            }
        }

        let map = Map::new(map_def);

        state.world.specs_world.add_resource(map);

        Story::Run(Box::new(SceneStory::new(tilemap_src.clone())))
    }));
}

pub struct SceneStory {
    tilemap_src: String,
    started: bool,
    done: Rc<RefCell<bool>>,
}

impl SceneStory {
    pub fn new(tilemap_src: String) -> Self {
        SceneStory {
            tilemap_src,
            started: false,
            done: Rc::new(RefCell::new(false)),
        }
    }
}

impl State<StoryboardContext> for SceneStory {
    fn state_name(&self) -> String {
        format!("SceneStory: {}", self.tilemap_src)
    }
    fn update(&mut self, _dt: f32, _ctx: StateData<StoryboardContext>) -> StoryTrans {
        if !self.started {
            self.started = true;
            return Trans::Push(Box::new(SceneState::new(Rc::clone(&self.done))));
        }
        if *self.done.borrow() {
            return Trans::Pop;
        }
        Trans::None
    }
    fn is_blocking(&self) -> bool {
        false
    }
}

struct SceneState {
    done: Rc<RefCell<bool>>,
}

impl SceneState {
    pub fn new(done: Rc<RefCell<bool>>) -> Self {
        SceneState { done }
    }
}

impl State<StoryboardContext> for SceneState {
    fn state_name(&self) -> String {
        return "SceneState".to_owned();
    }
    fn update(&mut self, _dt: f32, _ctx: StateData<StoryboardContext>) -> StoryTrans {
        Trans::None
    }
    fn draw(&mut self, ctx: StateData<StoryboardContext>) {
        let state = &mut *ctx.data.state.borrow_mut();
        let ctx = &mut *ctx.data.ctx.borrow_mut();

        let mut sprite_map = state.world.specs_world.write_resource::<SpriteMap>();
        let map = state.world.specs_world.read_resource::<Map>();
        let i_src = &map.map_def.tilesets[0].images[0].source;
        let map_sprite = sprite_map.0.get_mut(i_src).unwrap();

        let s = map_sprite.with_context(&*map);
        graphics::draw(ctx, &s, Point2::new(0.0, 0.0), 0.0).unwrap();
    }
}
