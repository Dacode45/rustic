use ggez::graphics::*;
use specs::{Component, VecStorage};

use std::mem::drop;
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
    pub id: Option<EntityID>,
    pub anim: Option<Animation>,
    pub pos: Option<Position>,
    pub t_pos: Option<TilePosition>,
    pub render: Option<EntityRender>,
    pub map: Option<Arc<RwLock<Map>>>,
    pub input: Option<InputState>,
}

impl Default for Character {
    fn default() -> Self {
        Character::new(None, None, None, None, None, None, None)
    }
}

impl Character {
    pub fn new(
        id: Option<EntityID>,
        anim: Option<Animation>,
        pos: Option<Position>,
        t_pos: Option<TilePosition>,
        render: Option<EntityRender>,
        map: Option<Arc<RwLock<Map>>>,
        input: Option<InputState>,
    ) -> Self {
        Character {
            id: id,
            anim: anim,
            pos: pos,
            t_pos: t_pos,
            render: render,
            map: map,
            input: input,
        }
    }

    pub fn id(&mut self) -> &mut EntityID {
        match self.id {
            Some(ref mut id) => return id,
            None => panic!("no EntityID component"),
        }
    }

    pub fn anim(&mut self) -> &mut Animation {
        match self.anim {
            Some(ref mut anim) => return anim,
            None => panic!("no Animation component"),
        }
    }

    pub fn pos(&mut self) -> &mut Position {
        match self.pos {
            Some(ref mut pos) => return pos,
            None => panic!("no position component"),
        }
    }

    pub fn t_pos(&mut self) -> &mut TilePosition {
        match self.t_pos {
            Some(ref mut t_pos) => return t_pos,
            None => panic!("no TilePosition component"),
        }
    }

    pub fn render(&mut self) -> &mut EntityRender {
        match self.render {
            Some(ref mut render) => return render,
            None => panic!("no EntityRender component"),
        }
    }

    pub fn map(&mut self) -> &mut Arc<RwLock<Map>> {
        match self.map {
            Some(ref mut map) => return map,
            None => panic!("no Map component"),
        }
    }

    pub fn input(&mut self) -> &mut InputState {
        match self.input {
            Some(ref mut input) => return input,
            None => panic!("no input component"),
        }
    }

    pub fn get_tile_foot(&mut self, x: usize, y: usize) -> Point2 {
        let mut foot = self.map().read().unwrap().get_tile_foot(x, y);
        foot.x -= self.render().width as f32 / 2.0;
        foot.y -= self.render().height as f32;
        foot
    }

    pub fn read_tile(&mut self, x: usize, y: usize, layer: usize) -> u32 {
        let map = self.map().read().unwrap();
        return map.get_tile(x, y, layer);
    }

    pub fn can_move(&mut self, x: usize, y: usize, layer: usize) -> bool {
        self.map().read().unwrap().is_blocking(x, y, layer)
    }

    pub fn move_to(&mut self, x: usize, y: usize, layer: usize) -> bool {
        let t_pos = self.t_pos().clone();
        {
            let mut map = self.map().write().unwrap();
            if !map.in_bounds(x, y, layer) || map.is_blocking(x, y, layer) {
                return false;
            }
            map.set_blocking(x, y, layer, true);
            map.set_blocking(t_pos.x, t_pos.y, t_pos.layer, false);
        }
        self.t_pos().set_tile_pos(x, y, layer);
        true
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

    pub fn update(&mut self, dt: f32, c: &mut Character) {
        if !self.states.is_running() {
            self.states.start(StateData::new(c));
        } else {
            self.states.update(dt, StateData::new(c));
        }
        let mut state_desc = Vec::new();
        for s in self.states.state_stack.iter() {
            state_desc.push(format!(
                "[{}]{}",
                if s.is_blocking() { "B" } else { " " },
                s.state_name()
            ))
        }
        info!("Character: {:?}", state_desc);
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

    fn update(&mut self, _dt: f32, ctx: StateData<Character>) -> Trans<Character> {
        let c = ctx.data;
        let current_pos = c.pos().0.clone();
        let tx = c.t_pos().x;
        let ty = c.t_pos().y;
        let l = c.t_pos().layer;

        let mut ntx = tx;
        let mut nty = ty;
        let mut facing = Facing::Down;

        if c.input().get_axis(Axis::Horz) > 0.1 {
            ntx += 1;
            facing = Facing::Right;
        } else if c.input().get_axis(Axis::Horz) < -0.1 {
            ntx -= 1;
            facing = Facing::Left;
        } else if c.input().get_axis(Axis::Vert) > 0.1 {
            nty -= 1;
            facing = Facing::Up;
        } else if c.input().get_axis(Axis::Vert) < -0.1 {
            nty += 1;
            facing = Facing::Down;
        }
        if ntx != tx || nty != ty {
            // attempt to move
            if c.move_to(ntx, nty, l) {
                let next_pos = c.get_tile_foot(ntx, nty);
                return Trans::Push(Box::new(MoveState::new(facing, current_pos, next_pos)));
            }
        }
        let frames = c.anim().map.get("idle").unwrap().clone();
        c.anim().set_frames(frames);
        c.anim().should_loop = true;
        Trans::None
    }
}

pub struct MoveState {
    facing: Facing,
    start: Point2,
    end: Point2,
    local_t: f32,
    t: Arc<RwLock<f32>>,
}

impl MoveState {
    pub fn new(facing: Facing, start: Point2, end: Point2) -> Self {
        MoveState {
            facing,
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
        let frames = c.anim().map.get(self.facing.name()).unwrap().clone();
        c.anim().spf = 1.0 / (frames.len()) as f32;
        c.anim().set_frames(frames);
        c.anim().should_loop = true;

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
    fn update(&mut self, _dt: f32, ctx: StateData<Character>) -> Trans<Character> {
        let t = *self.t.read().unwrap();
        self.local_t = t;
        if t >= 1.0 {
            return Trans::Pop;
        }
        let pos = Point2::new(
            self.start.x + t * (self.end.x - self.start.x),
            self.start.y + t * (self.end.y - self.start.y),
        );
        ctx.data.pos = Some(Position(pos));
        Trans::None
    }
}

pub struct FollowPathState {
    index: usize,
    path: Vec<Facing>,
    done: Arc<RwLock<bool>>,
}

impl FollowPathState {
    pub fn new(path: Vec<Facing>, done: Arc<RwLock<bool>>) -> Self {
        FollowPathState {
            path,
            index: 0,
            done,
        }
    }
}

impl State<Character> for FollowPathState {
    fn state_name(&self) -> String {
        format!(
            "FollowPathState: {}",
            if self.index >= self.path.len() {
                "Done"
            } else {
                self.path[self.index].name()
            }
        ).to_owned()
    }
    fn update(&mut self, _dt: f32, ctx: StateData<Character>) -> Trans<Character> {
        if self.index >= self.path.len() {
            return Trans::Pop;
        }
        println!("Continue");
        let c = ctx.data;
        let current_pos = c.pos().0.clone();
        let tx = c.t_pos().x;
        let ty = c.t_pos().y;
        let l = c.t_pos().layer;

        let mut ntx = tx;
        let mut nty = ty;
        match self.path[self.index] {
            Facing::Up => {
                nty -= 1;
            }
            Facing::Down => {
                nty += 1;
            }
            Facing::Left => {
                ntx -= 1;
            }
            Facing::Right => {
                ntx += 1;
            }
        }
        if c.move_to(ntx, nty, l) {
            let next_pos = c.get_tile_foot(ntx, nty);
            let trans = Trans::Push(Box::new(MoveState::new(
                self.path[self.index].clone(),
                current_pos,
                next_pos,
            )));
            self.index += 1;
            return trans;
        }

        Trans::None
    }
    fn on_stop(&mut self, _ctx: StateData<Character>) {
        *self.done.write().unwrap() = true;
    }
}
