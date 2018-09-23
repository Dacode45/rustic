use state::*;
use storyboard::*;

pub fn wait(duration: f32) -> Story {
    return Story::Run(Box::new(WaitState::new(duration)));
}

pub struct WaitState {
    time_left: f32,
}

impl WaitState {
    pub fn new(seconds: f32) -> Self {
        WaitState { time_left: seconds }
    }
}

impl State<StoryboardContext> for WaitState {
    fn update(&mut self, dt: f32, _context: StateData<StoryboardContext>) -> StoryTrans {
        self.time_left -= dt;
        // println!("Waiting -{} {}!", dt, self.time_left);
        if self.time_left < 0.0 {
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn state_name(&self) -> String {
        "WaitState".to_owned()
    }
}
