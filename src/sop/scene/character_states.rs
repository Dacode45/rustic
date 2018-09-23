use std::sync::Arc;
use std::sync::RwLock;

use components::*;
use entities;
use resources::*;
use state::*;
use storyboard::*;

pub fn add_character(gen: fn() -> entities::CharacterBuilder) -> Story {
    return Story::Setup(Box::new(move |ctx| {
        let state = &mut *ctx.state.borrow_mut();
        let ctx = &mut *ctx.ctx.borrow_mut();
        let builder = gen();
        {
            // add the sprite
            let render = match builder.render {
                Some(ref render) => render.clone(),
                None => panic!("No renderer"),
            };
            add_sprite_resource(
                &render.sprite_id,
                render.width,
                render.height,
                ctx,
                &mut state.world.specs_world,
            );
        }
        builder.add(&mut state.world.specs_world);
        return Story::Done("add_character".to_owned());
    }));
}

pub fn move_character(_name: &str, path: Vec<Facing>) -> Story {
    use specs::Join;

    let eid = _name.to_owned();
    return Story::Setup(Box::new(move |ctx| {
        return Story::Start(Box::new(MoveCharacterState::new(eid.clone(), path.clone())));
    }));
}

pub struct MoveCharacterState {
    path: Vec<Facing>,
    eid: String,
    done: Arc<RwLock<bool>>,
}

impl MoveCharacterState {
    pub fn new(eid: String, path: Vec<Facing>) -> Self {
        return MoveCharacterState {
            path,
            eid,
            done: Arc::new(RwLock::new(false)),
        };
    }
}

impl State<StoryboardContext> for MoveCharacterState {
    fn on_start(&mut self, ctx: StateData<StoryboardContext>) -> StoryTrans {
        use specs::Join;

        let ctx = ctx.data;
        let state = &mut *ctx.state.borrow_mut();
        let ctx = &mut *ctx.ctx.borrow_mut();
        let world = &mut state.world.specs_world;

        let id = world.read_storage::<EntityID>();
        let mut cc = world.write_storage::<CharacterController>();

        for (id, cc) in (&id, &mut cc).join() {
            if id.0 == self.eid {
                let mut c = Character::default();
                cc.states.push(
                    Box::new(character::FollowPathState::new(
                        self.path.clone(),
                        self.done.clone(),
                    )),
                    StateData::new(&mut c),
                );
            }
        }
        Trans::None
    }
    fn update(&mut self, _dt: f32, _context: StateData<StoryboardContext>) -> StoryTrans {
        if *self.done.read().unwrap() {
            return Trans::Pop;
        }
        Trans::None
    }

    fn state_name(&self) -> String {
        "EmptyState".to_owned()
    }
}
