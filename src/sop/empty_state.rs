use state::*;
use storyboard::*;
pub struct EmptyState;

impl State<StoryboardContext> for EmptyState {
    fn update(&mut self, _dt: f32, _context: StateData<StoryboardContext>) -> StoryTrans {
        Trans::None
    }

    fn state_name(&self) -> String {
        "EmptyState".to_owned()
    }
}
