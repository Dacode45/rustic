use ggez::graphics::*;

use state::*;
use storyboard::*;

pub fn screen_state(color: Color) -> Story {
    Story::Start(Box::new(ScreenStory::new(color)))
}
#[derive(Clone)]
pub struct ScreenStory {
    color: Color,
}

impl ScreenStory {
    pub fn new(color: Color) -> Self {
        ScreenStory { color }
    }
}

impl State<StoryboardContext> for ScreenStory {
    fn state_name(&self) -> String {
        return format!("ScreenState: {:?}", self.color.to_rgba());
    }
    fn on_start(&mut self, _ctx: StateData<StoryboardContext>) -> StoryTrans {
        Trans::Push(Box::new(self.clone()))
    }
    fn draw(&mut self, ctx: StateData<StoryboardContext>) {
        let ctx = &mut *ctx.data.ctx.borrow_mut();
        let (width, height) = get_size(ctx);

        set_color(ctx, self.color).unwrap();
        let rect = Rect::new(0.0, 0.0, width as f32, height as f32);
        rectangle(ctx, DrawMode::Fill, rect).unwrap();
    }
    fn is_blocking(&self) -> bool {
        false
    }
}
