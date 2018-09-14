use state;
use storyboard;
pub struct EmptyState;

impl state::State<storyboard::StoryboardContext> for EmptyState {
    fn update(
        &mut self,
        _dt: f32,
        _context: state::StateData<storyboard::StoryboardContext>,
    ) -> storyboard::StoryTrans {
        state::Trans::None
    }

    fn state_name(&self) -> &'static str {
        "EmptyState"
    }
}
