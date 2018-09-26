use ggez::*;

use std;
use std::cell::RefCell;
use std::rc::Rc;

use sop;
use state;
use world::*;

type StoryConstructor = Fn(&mut StoryboardContext) -> Story;

pub type StoryState = state::State<StoryboardContext>;
pub type StoryTrans = state::Trans<StoryboardContext>;

pub enum Story {
    Setup(Box<StoryConstructor>),
    Start(Box<StoryState>),
    Run(Box<StoryState>),
    Done(String),
}

pub struct PartialStoryboardContext {
    // pub input_binding: input::InputBinding,
    pub world: World,
    pub should_quit: bool,
}

pub struct StoryboardContext {
    pub state: Rc<RefCell<PartialStoryboardContext>>,
    pub ctx: Rc<RefCell<Context>>,
}

// impl StoryboardContext {
//     pub fn new(data: & mut PartialStoryboardContext, ctx: & mut Context) -> Self {
//         StoryboardContext { data, ctx }
//     }
// }

pub struct Storyboard {
    pub stories: Vec<Story>,
    pub ctx: Rc<RefCell<PartialStoryboardContext>>,
    pub storystack: state::StateMachine<StoryboardContext>,
}

impl Storyboard {
    pub fn new(world: World, stories: Vec<Story>) -> Self {
        Storyboard {
            stories: stories,
            ctx: Rc::new(RefCell::new(PartialStoryboardContext {
                world: world,
                should_quit: false,
            })),
            storystack: state::StateMachine::new(Box::new(sop::EmptyState)),
        }
    }

    pub fn update_stories(
        dt: f32,
        ctx: &mut StoryboardContext,
        storystack: &mut state::StateMachine<StoryboardContext>,
        stories: Vec<Story>,
    ) -> Vec<Story> {
        let mut done = false;
        let mut iter = 0;
        stories
            .into_iter()
            .map(|story| {
                if done {
                    return story;
                }
                iter += 1;
                let next = match story {
                    Story::Setup(setup) => {
                        done = true;
                        setup(ctx)
                    }
                    Story::Start(mut start) => {
                        let trans = start.on_start(state::StateData::new(ctx));
                        if let state::Trans::Pop = trans {
                            return Story::Done(start.state_name().to_owned());
                        } else {
                            storystack.transition(trans, state::StateData::new(ctx));
                        }
                        if start.is_blocking() {
                            done = true;
                        }
                        Story::Run(start)
                    }
                    Story::Run(mut s) => {
                        let trans = s.update(dt, state::StateData::new(ctx));
                        if let state::Trans::Pop = trans {
                            return Story::Done(s.state_name().to_owned());
                        } else {
                            storystack.transition(trans, state::StateData::new(ctx));
                        }
                        if s.is_blocking() {
                            done = true;
                        }
                        Story::Run(s)
                    }
                    Story::Done(name) => Story::Done(name),
                };
                next
            }).collect()
    }

    pub fn get_context(&mut self, ctx: Rc<RefCell<Context>>) -> StoryboardContext {
        StoryboardContext {
            state: Rc::clone(&self.ctx),
            ctx: ctx,
        }
    }

    pub fn draw_storyboard(&mut self, ctx: Rc<RefCell<Context>>) {
        let mut s_ctx = self.get_context(ctx);
        self.storystack.draw(state::StateData::new(&mut s_ctx));
    }

    pub fn update_storyboard(&mut self, dt: f32, ctx: Rc<RefCell<Context>>) -> bool {
        // setup the storyboard context
        let mut s_ctx = self.get_context(ctx);

        // run storystack
        if !self.storystack.is_running() {
            info!("Starting Storystack");
            self.storystack.start(state::StateData::new(&mut s_ctx));
        } else {
            self.storystack
                .update(dt, state::StateData::new(&mut s_ctx));
        }

        // capture the current stories
        let mut stories = Vec::new();
        std::mem::swap(&mut self.stories, &mut stories);

        let story_names: Vec<String> = stories
            .iter()
            .map(|story| match story {
                Story::Setup(_) => "setup state".to_owned(),
                Story::Start(s) => format!("TO START: {}", s.state_name()).to_owned(),
                Story::Run(s) => s.state_name().to_owned(),
                Story::Done(name) => format!("DONE: {}", name),
            }).collect();

        let state_names: Vec<String> = self
            .storystack
            .state_stack
            .iter()
            .map(|state| state.state_name().to_owned())
            .collect();

        info!("story_names {:?}", story_names);
        info!("state_names {:?}", state_names);
        // update the stories
        self.stories = Storyboard::update_stories(dt, &mut s_ctx, &mut self.storystack, stories);
        if s_ctx.state.borrow().should_quit {
            return true;
        }
        false
    }
}
