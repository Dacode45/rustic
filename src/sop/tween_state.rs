use state;
use tween::*;

pub struct TweenState<F>
where
    F: FnMut(f32) + Send + Sync + 'static,
{
    pub tween: Tween,
    pub tween_fn: TweenFn,
    pub apply_fn: F,
}

impl<T, F> state::State<T> for TweenState<F>
where
    F: FnMut(f32) + Send + Sync + 'static,
{
    fn state_name(&self) -> String {
        return format!("TweenState: {}", self.tween.value());
    }

    fn update(&mut self, dt: f32, _ctx: state::StateData<T>) -> state::Trans<T> {
        self.tween.update(dt, &self.tween_fn);
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
