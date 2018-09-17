use state::*;
use storyboard::*;

pub fn quit_state() -> Story {
    return Story::Start(Box::new(QuitState));
}

struct QuitState;

impl State<StoryboardContext> for QuitState {
    fn state_name(&self) -> String {
        return "QuitState".to_owned();
    }
    fn on_start(&mut self, ctx: StateData<StoryboardContext>) -> StoryTrans {
        ctx.data.state.borrow_mut().should_quit = true;
        Trans::None
    }
}
