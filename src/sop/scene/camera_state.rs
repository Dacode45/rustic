use std::cell::RefCell;
use std::rc::Rc;

use map::*;
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
    t: Rc<RefCell<f32>>,
}

impl MoveCameraToTile {
    pub fn new(tile_x: usize, tile_y: usize, duration: f32) -> Self {
        MoveCameraToTile {
            start_tile_x: 0,
            start_tile_y: 0,

            end_tile_x: tile_x,
            end_tile_y: tile_y,
            duration,
            t: Rc::new(RefCell::new(0.0)),
        }
    }
}

impl State<StoryboardContext> for MoveCameraToTile {
    fn state_name(&self) -> String {
        let t = *self.t.borrow();
        format!("MoveCameraToTile {}", t)
    }
    fn on_start(&mut self, ctx: StateData<StoryboardContext>) -> StoryTrans {
        let state = &mut *ctx.data.state.borrow_mut();
        let mut camera = state.world.specs_world.write_resource::<Camera>();
        let mut map = state.world.specs_world.write_resource::<Map>();

        let (x, y) = map.point_to_tile(camera.0.x, camera.0.y);
        self.start_tile_x = x;
        self.start_tile_y = y;

        let t = Rc::clone(&self.t);
        let tween = Box::new(TweenState {
            tween: Tween::new(0.0, 1.0, self.duration),
            tween_fn: Box::new(ease_in_quad),
            apply_fn: Box::new(move |value| {
                *t.borrow_mut() = value;
            }),
        });
        return Trans::Push(tween);
    }

    fn update(&mut self, dt: f32, ctx: StateData<StoryboardContext>) -> StoryTrans {
        let state = &mut *ctx.data.state.borrow_mut();
        let mut camera = state.world.specs_world.write_resource::<Camera>();
        let mut map = state.world.specs_world.write_resource::<Map>();

        let t = *self.t.borrow();

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
