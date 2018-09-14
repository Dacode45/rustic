use ggez::*;

use std;

use input;
use sop;
use state;

type StoryConstructor<'a> = Fn(&mut StoryboardContext<'a>) -> Story<'a>;

pub type StoryState<'a> = state::State<StoryboardContext<'a>>;
pub type StoryTrans<'a> = state::Trans<StoryboardContext<'a>>;

pub enum Story<'a> {
    Setup(Box<StoryConstructor<'a>>),
    Start(Box<StoryState<'a>>),
}

pub struct PartialStoryboardContext {
    pub input_binding: input::InputBinding,
}

pub struct StoryboardContext<'a> {
    pub data: &'a mut PartialStoryboardContext,
    pub ctx: &'a mut Context,
}

pub struct Storyboard<'a> {
    pub stories: Vec<Story<'a>>,
    pub ctx: PartialStoryboardContext,
    pub storystack: state::StateMachine<StoryboardContext<'a>>,
}

impl<'a> Storyboard<'a> {
    pub fn new(stories: Vec<Story<'a>>) -> Self {
        Storyboard {
            stories: stories,
            ctx: PartialStoryboardContext {
                input_binding: input::create_input_binding(),
            },
            storystack: state::StateMachine::new(Box::new(sop::EmptyState)),
        }
    }

    pub fn update(&'a mut self, dt: f32, ctx: &'a mut Context) {
        let m_ctx = &mut self.ctx;
        let mut s_ctx = StoryboardContext {
            data: m_ctx,
            ctx: ctx,
        };
        if self.storystack.is_running() {
            self.storystack.start(state::StateData::new(&mut s_ctx));
        } else {
            self.storystack
                .update(dt, state::StateData::new(&mut s_ctx));
        }
    }
}
