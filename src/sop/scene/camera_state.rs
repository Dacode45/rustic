use std::sync::{Arc, RwLock};

use resources::*;
use sop::TweenState;
use state::*;
use storyboard::*;
use tween::*;

pub fn move_camera_to_tile(tile_x: usize, tile_y: usize, duration: f32) -> Story {
    Story::Start(Box::new(MoveCameraToTile::new(tile_x, tile_y, duration)))
}

pub struct MoveCameraToTile {
    start_tile_x: usize,
    start_tile_y: usize,

    end_tile_x: usize,
    end_tile_y: usize,

    duration: f32,
    t: Arc<RwLock<f32>>,
}

impl MoveCameraToTile {
    pub fn new(tile_x: usize, tile_y: usize, duration: f32) -> Self {
        MoveCameraToTile {
            start_tile_x: 0,
            start_tile_y: 0,

            end_tile_x: tile_x,
            end_tile_y: tile_y,
            duration,
            t: Arc::new(RwLock::new(0.0)),
        }
    }
}

impl State<StoryboardContext> for MoveCameraToTile {
    fn state_name(&self) -> String {
        format!("MoveCameraToTile {},{}", self.end_tile_x, self.end_tile_y)
    }
    fn on_start(&mut self, ctx: StateData<StoryboardContext>) -> StoryTrans {
        let state = &mut *ctx.data.state.borrow_mut();
        let camera = state.world.specs_world.write_resource::<Camera>();
        let current_map = state.world.specs_world.read_resource::<CurrentMap>();
        let mut maps = state.world.specs_world.write_resource::<Maps>();
        let map = maps.0.get_mut(&current_map.0).unwrap();

        let (x, y) = map.point_to_tile(camera.0.x, camera.0.y);
        self.start_tile_x = x;
        self.start_tile_y = y;

        let t = self.t.clone();
        let tween = Box::new(TweenState {
            tween: Tween::new(0.0, 1.0, self.duration),
            tween_fn: TweenFn::EaseInQuad,
            apply_fn: move |value| {
                *t.write().unwrap() = value;
            },
        });
        return Trans::Push(tween);
    }

    fn update(&mut self, _dt: f32, ctx: StateData<StoryboardContext>) -> StoryTrans {
        let state = &mut *ctx.data.state.borrow_mut();
        let mut camera = state.world.specs_world.write_resource::<Camera>();
        let current_map = state.world.specs_world.read_resource::<CurrentMap>();
        let mut maps = state.world.specs_world.write_resource::<Maps>();
        let map = maps.0.get_mut(&current_map.0).unwrap();

        let t = *self.t.read().unwrap();

        let (tx, ty) = map.tile_dimensions();
        let x =
            (self.start_tile_x * tx as usize) as f32 + t * (self.end_tile_x * tx as usize) as f32;
        let y =
            (self.start_tile_y * ty as usize) as f32 + t * (self.end_tile_y * ty as usize) as f32;

        camera.0.x = x;
        camera.0.y = y;

        map.camera = camera.0.clone();

        if t >= 1.0 {
            return Trans::Pop;
        }

        Trans::None
    }
}
