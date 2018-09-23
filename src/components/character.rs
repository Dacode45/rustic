use ggez::graphics::*;
use specs::{Component, VecStorage};

use std::sync::{Arc, RwLock};

use components::*;
use input::{Axis, InputState};
use map::Map;
use sop::TweenState;
use state::*;
use storyboard::*;
use tween::*;

#[derive(Debug)]
pub struct Character {
    pub anim: Animation,
    pub pos: Position,
    pub t_pos: TilePosition,
    pub render: EntityRender,
    pub map: Arc<RwLock<Map>>,
    pub input: InputState,
}

impl Character {
    pub fn new(
        anim: Animation,
        pos: Position,
        t_pos: TilePosition,
        render: EntityRender,
        map: Arc<RwLock<Map>>,
        input: InputState,
    ) -> Self {
        Character {
            anim,
            pos,
            t_pos,
            render,
            map,
            input,
        }
    }

    pub fn get_tile_foot(&self, x: usize, y: usize) -> Point2 {
        let mut foot = self.map.read().unwrap().get_tile_foot(x, y);
        foot.x -= self.render.width as f32 / 2.0;
        foot.y -= self.render.height as f32;
        foot
    }
}

pub type CharacterState = State<Character>;
pub type CharacterTrans = Trans<Character>;

pub struct CharacterController {
    pub states: StateMachine<Character>,
}

impl CharacterController {
    pub fn new(state: Box<State<Character> + Sync>) -> Self {
        CharacterController {
            states: StateMachine::new(state),
        }
    }
}

// sue me
unsafe impl Send for CharacterController {}

impl Component for CharacterController {
    type Storage = VecStorage<Self>;
}

pub struct WaitState;

impl State<Character> for WaitState {
    fn state_name(&self) -> String {
        "WaitState".to_owned()
    }
    fn update(&mut self, dt: f32, ctx: StateData<Character>) -> Trans<Character> {
        let c = ctx.data;
        let current_pos = c.pos.0.clone();
        let tx = c.t_pos.x;
        let ty = c.t_pos.y;
        let l = c.t_pos.layer;

        info!(
            "Character: {} {:?} {:?}",
            self.state_name(),
            c.input.get_axis(Axis::Horz),
            c.input
        );

        if c.input.get_axis(Axis::Horz) > 0.1 {
            let next_pos = c.get_tile_foot(c.t_pos.x + 1, c.t_pos.y);
            c.t_pos.set_tile_pos(tx + 1, ty, l);
            return Trans::Push(Box::new(MoveState::new(
                Facing::Right,
                current_pos,
                next_pos,
            )));
        } else if c.input.get_axis(Axis::Horz) < -0.1 {
            let next_pos = c.get_tile_foot(c.t_pos.x - 1, c.t_pos.y);
            c.t_pos.set_tile_pos(tx - 1, ty, l);
            return Trans::Push(Box::new(MoveState::new(
                Facing::Left,
                current_pos,
                next_pos,
            )));
        } else if c.input.get_axis(Axis::Vert) > 0.1 {
            let next_pos = c.get_tile_foot(c.t_pos.x, c.t_pos.y - 1);
            c.t_pos.set_tile_pos(tx, ty - 1, l);
            return Trans::Push(Box::new(MoveState::new(Facing::Up, current_pos, next_pos)));
        } else if c.input.get_axis(Axis::Vert) < -0.1 {
            let next_pos = c.get_tile_foot(c.t_pos.x, c.t_pos.y + 1);
            c.t_pos.set_tile_pos(tx, ty + 1, l);
            return Trans::Push(Box::new(MoveState::new(
                Facing::Down,
                current_pos,
                next_pos,
            )));
        }
        let frames = c.anim.map.get("idle").unwrap().clone();
        c.anim.set_frames(frames);
        c.anim.should_loop = true;
        Trans::None
    }
}

pub struct MoveState {
    facing: Facing,
    started: bool,
    start: Point2,
    end: Point2,
    local_t: f32,
    t: Arc<RwLock<f32>>,
}

impl MoveState {
    pub fn new(facing: Facing, start: Point2, end: Point2) -> Self {
        MoveState {
            facing,
            started: false,
            start: start,
            end: end,
            local_t: 0.0,
            t: Arc::new(RwLock::new(0.0)),
        }
    }
}

impl State<Character> for MoveState {
    fn state_name(&self) -> String {
        format!("MoveState: {}", self.local_t).to_owned()
    }
    fn on_start(&mut self, ctx: StateData<Character>) -> Trans<Character> {
        let c = ctx.data;
        let frames = c.anim.map.get(self.facing.name()).unwrap().clone();
        c.anim.spf = 1.0 / frames.len() as f32;
        c.anim.set_frames(frames);
        c.anim.should_loop = true;
        Trans::None
    }
    fn update(&mut self, dt: f32, ctx: StateData<Character>) -> Trans<Character> {
        info!("Character: {}", self.state_name());

        if !self.started {
            self.started = true;
            let c = ctx.data;

            let t = self.t.clone();
            let tween = Box::new(TweenState {
                tween: Tween::new(0.0, 1.0, 1.0),
                tween_fn: TweenFn::EaseInQuad,
                apply_fn: move |value| {
                    *t.write().unwrap() = value;
                },
            });
            return Trans::Push(tween);
        }
        debug!("MoveState");
        let t = *self.t.read().unwrap();
        self.local_t = t;
        if t >= 1.0 {
            return Trans::Pop;
        }
        let pos = Point2::new(
            self.start.x + t * (self.end.x - self.start.x),
            self.start.y + t * (self.end.y - self.start.y),
        );
        ctx.data.pos = Position(pos);
        Trans::None
    }
}
