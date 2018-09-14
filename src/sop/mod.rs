use state;
use storyboard;

pub struct EmptyState;

impl<'a> state::State<storyboard::StoryboardContext<'a>> for EmptyState {
    fn update(
        &mut self,
        dt: f32,
        context: state::StateData<storyboard::StoryboardContext<'a>>,
    ) -> storyboard::StoryTrans<'a> {
        state::Trans::None
    }

    fn state_name(&self) -> &'static str {
        "EmptyState"
    }
}

pub struct WaitState {
    time_left: f32,
}

impl WaitState {
    pub fn new(seconds: f32) -> Self {
        WaitState { time_left: seconds }
    }
}

impl<'a> state::State<storyboard::StoryboardContext<'a>> for WaitState {
    fn update(
        &mut self,
        dt: f32,
        context: state::StateData<storyboard::StoryboardContext<'a>>,
    ) -> storyboard::StoryTrans<'a> {
        self.time_left -= dt;
        state::Trans::None
    }
}
