use ggez::*;

use sop;
use state;

type StoryConstructor<'a> = Fn(&mut StoryboardContext<'a>) -> Story<'a>;

pub type StoryState<'a> = state::State<StoryboardContext<'a>, ()>;
pub type StoryTrans<'a> = state::Trans<StoryboardContext<'a>, ()>;

pub enum Story<'a> {
    Setup(Box<StoryConstructor<'a>>),
    Start(Box<StoryState<'a>>),
}

pub struct PartialStoryboardContext {}
pub struct StoryboardContext<'a> {
    ctx: &'a mut Context,
}

pub struct Storyboard<'a> {
    stories: Vec<Story<'a>>,
    ctx: PartialStoryboardContext,
    storystack: state::StateMachine<StoryboardContext<'a>, ()>,
}

impl<'a> Storyboard<'a> {
    pub fn new(stories: Vec<Story<'a>>) -> Self {
        Storyboard {
            stories: stories,
            ctx: PartialStoryboardContext {},
            storystack: state::StateMachine::new(Box::new(sop::EmptyState)),
        }
    }
}
