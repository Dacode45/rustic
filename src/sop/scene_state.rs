use std::cell::RefCell;
use std::rc::Rc;

use map::*;
use state::*;
use storyboard::*;

pub fn create_scene(tilemap_src: &'static str) -> Story {
    let tilemap_src = tilemap_src.to_owned();
    return Story::Setup(Box::new(move |ctx| {
        let state = &mut *ctx.state.borrow_mut();
        let ctx = &mut *ctx.ctx.borrow_mut();
        let map_def = load_tile_map(ctx, tilemap_src.as_ref()).unwrap();

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
    fn update(&mut self, dt: f32, ctx: StateData<StoryboardContext>) -> StoryTrans {
        if !self.started {
            self.started = true;
            return Trans::Push(Box::new(SceneState::new(Rc::clone(&self.done))));
        }
        if *self.done.borrow() {
            return Trans::Pop;
        }
        Trans::None
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
    fn update(&mut self, dt: f32, ctx: StateData<StoryboardContext>) -> StoryTrans {
        Trans::None
    }
}
