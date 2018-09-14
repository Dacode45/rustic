use ggez::*;

use std;
use std::cell::RefCell;
use std::rc::Rc;

use input;
use sop;
use state;

type StoryConstructor = Fn(&mut StoryboardContext) -> Story;

pub type StoryState = state::State<StoryboardContext>;
pub type StoryTrans = state::Trans<StoryboardContext>;

pub enum Story {
    Setup(Box<StoryConstructor>),
    Run(Box<StoryState>),
    Done(String),
}

pub struct PartialStoryboardContext {
    pub input_binding: input::InputBinding,
}

pub struct StoryboardContext {
    pub data: Rc<RefCell<PartialStoryboardContext>>,
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
    pub fn new(stories: Vec<Story>) -> Self {
        Storyboard {
            stories: stories,
            ctx: Rc::new(RefCell::new(PartialStoryboardContext {
                input_binding: input::create_input_binding(),
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

        stories
            .into_iter()
            .map(|story| {
                if done {
                    return story;
                }
                let next = match story {
                    Story::Setup(setup) => setup(ctx),
                    Story::Run(mut s) => {
                        let trans = s.update(dt, state::StateData::new(ctx));
                        if let state::Trans::Pop = trans {
                            return Story::Done(s.state_name().to_owned());
                        }
                        storystack.transition(trans, state::StateData::new(ctx));
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

    pub fn do_nothing(&mut self) {}

    pub fn update(&mut self, dt: f32, ctx: Rc<RefCell<Context>>) {
        // setup the storyboard context

        let mut s_ctx = StoryboardContext {
            data: Rc::clone(&self.ctx),
            ctx: ctx,
        };

        // capture the current stories
        let mut stories = Vec::new();
        std::mem::swap(&mut self.stories, &mut stories);

        let story_names: Vec<String> = stories
            .iter()
            .map(|story| match story {
                Story::Setup(_) => "setup state".to_owned(),
                Story::Run(s) => s.state_name().to_owned(),
                Story::Done(name) => format!("Done: {}", name),
            }).collect();

        let state_names: Vec<String> = self
            .storystack
            .state_stack
            .iter()
            .map(|state| state.state_name().to_owned())
            .collect();

        println!("story_names {:?}", story_names);
        println!("state_names {:?}", state_names);
        // update the stories
        self.stories = Storyboard::update_stories(dt, &mut s_ctx, &mut self.storystack, stories);

        // run storystack
        if self.storystack.is_running() {
            self.storystack.start(state::StateData::new(&mut s_ctx));
        } else {
            self.storystack
                .update(dt, state::StateData::new(&mut s_ctx));
        }
    }
}
