use ggez::graphics;
use ggez::graphics::*;
use specs;

use std::path;
use std::sync::{Arc, Mutex, RwLock};

use components::*;
use input::Input;
use map::*;
use resources::*;
use sprite::*;
use state::*;
use storyboard::*;
use systems::UpdateCharacters;

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

        let mut maps = state.world.specs_world.write_resource::<Maps>();
        let mut current = state.world.specs_world.write_resource::<CurrentMap>();

        maps.0.insert(tilemap_src.to_owned(), map);
        current.0 = tilemap_src.to_owned();

        Story::Run(Box::new(SceneStory::new(tilemap_src.clone())))
    }));
}

pub struct SceneStory {
    tilemap_src: String,
    started: bool,
    done: Arc<RwLock<bool>>,
}

impl SceneStory {
    pub fn new(tilemap_src: String) -> Self {
        SceneStory {
            tilemap_src,
            started: false,
            done: Arc::new(RwLock::new(false)),
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
            return Trans::Push(Box::new(SceneState::new(self.done.clone())));
        }
        if *self.done.read().unwrap() {
            return Trans::Pop;
        }
        Trans::None
    }
    fn is_blocking(&self) -> bool {
        false
    }
}

struct SceneState {
    done: Arc<RwLock<bool>>,
}

impl SceneState {
    pub fn new(done: Arc<RwLock<bool>>) -> Self {
        SceneState { done }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .with(UpdateCharacters, "update_characters", &[])
            .build()
    }
}

impl State<StoryboardContext> for SceneState {
    fn state_name(&self) -> String {
        return "SceneState".to_owned();
    }
    fn update(&mut self, _dt: f32, ctx: StateData<StoryboardContext>) -> StoryTrans {
        let state = &mut *ctx.data.state.borrow_mut();
        let res = &state.world.specs_world.res;
        let mut disp = SceneState::register_systems();
        disp.dispatch(&state.world.specs_world.res);
        Trans::None
    }
    fn draw(&mut self, ctx: StateData<StoryboardContext>) {
        use specs::Join;

        let state = &mut *ctx.data.state.borrow_mut();
        let ctx = &mut *ctx.data.ctx.borrow_mut();

        // Drawing map
        let mut sprite_map = state.world.specs_world.write_resource::<SpriteMap>();
        let current_map = state.world.specs_world.read_resource::<CurrentMap>();
        let mut maps = state.world.specs_world.write_resource::<Maps>();
        {
            let map = maps.0.get_mut(&current_map.0).unwrap();
            let i_src = map.map_def.tilesets[0].images[0].source.clone();
            let map_sprite = sprite_map.0.get_mut(&i_src).unwrap();

            for i in 0..map.map_def.layers.len() / 3 {
                map.layer_index = i;
                let s = map_sprite.with_context(&*map);
                graphics::draw(ctx, &s, Point2::new(0.0, 0.0), 0.0).unwrap();
            }
        }

        // Drawing characters
        let mut render = state.world.specs_world.write_storage::<EntityRender>();
        let pos = state.world.specs_world.read_storage::<Position>();
        let anim = state.world.specs_world.read_storage::<Animation>();
        for (render, anim) in (&mut render, &anim).join() {
            render.frame = anim.frame();
        }

        for (pos, render) in (&pos, &render).join() {
            let renderable = Renderable::new(pos.clone(), render.clone());
            warn!("Attempting to draw character: {:?}", renderable);
            if let Some(sprite) = sprite_map.0.get_mut(&render.sprite_id) {
                let s = sprite.with_context(&renderable);
                graphics::draw(ctx, &s, Point2::new(0.0, 0.0), 0.0).unwrap();
            } else {
                panic!("Sprite Doesn't Exist");
            }
        }
    }
}
