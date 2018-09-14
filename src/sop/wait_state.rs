use state;
use storyboard;

pub struct WaitState {
    time_left: f32,
}

impl WaitState {
    pub fn new(seconds: f32) -> Self {
        WaitState { time_left: seconds }
    }
}

impl state::State<storyboard::StoryboardContext> for WaitState {
    fn update(
        &mut self,
        dt: f32,
        _context: state::StateData<storyboard::StoryboardContext>,
    ) -> storyboard::StoryTrans {
        self.time_left -= dt;
        // println!("Waiting -{} {}!", dt, self.time_left);
        if self.time_left < 0.0 {
            state::Trans::Pop
        } else {
            state::Trans::None
        }
    }

    fn state_name(&self) -> String {
        "WaitState".to_owned()
    }
}
