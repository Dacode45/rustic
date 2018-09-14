use state;
use storyboard;
use tween::*;

pub struct TweenState {
    pub tween: Tween,
    pub tween_fn: Box<TweenFn>,
    pub apply_fn: Box<FnMut(f32)>,
}

impl state::State<storyboard::StoryboardContext> for TweenState {
    fn state_name(&self) -> String {
        return format!("TweenState: {}", self.tween.value());
    }

    fn update(
        &mut self,
        dt: f32,
        ctx: state::StateData<storyboard::StoryboardContext>,
    ) -> storyboard::StoryTrans {
        self.tween.update(dt, &*self.tween_fn);
        (self.apply_fn)(self.tween.value());
        if self.tween.is_finished() {
            return state::Trans::Pop;
        }
        state::Trans::None
    }

    fn is_blocking(&self) -> bool {
        return false;
    }
}
