use ggez::graphics::*;

use std::sync::{Arc, Mutex, RwLock};

use state::*;
use storyboard::*;

use super::tween_state::TweenState;
use tween::*;

/// goes from opague to transparent
pub fn fade_in(duration: f32, color: Color) -> Story {
    Story::Start(Box::new(FadeStory::new(duration, color, 1.0, 0.0)))
}

// goes from transparent to opague
pub fn fade_out(duration: f32, color: Color) -> Story {
    Story::Start(Box::new(FadeStory::new(duration, color, 0.0, 1.0)))
}

#[derive(Clone)]
pub struct FadeStory {
    duration: f32,
    color: Color,
    alpha_start: f32,
    alpha_end: f32,
    done: Arc<RwLock<bool>>,
}

impl FadeStory {
    pub fn new(seconds: f32, color: Color, start: f32, end: f32) -> Self {
        FadeStory {
            duration: seconds,
            color: color,
            alpha_start: start,
            alpha_end: end,
            done: Arc::new(RwLock::new(false)),
        }
    }
}

impl State<StoryboardContext> for FadeStory {
    fn on_start(&mut self, _ctx: StateData<StoryboardContext>) -> StoryTrans {
        return Trans::Push(Box::new(FadeState::new(
            self.duration,
            self.color,
            self.alpha_start,
            self.alpha_end,
            self.done.clone(),
        )));
    }
    fn update(&mut self, _dt: f32, _ctx: StateData<StoryboardContext>) -> StoryTrans {
        if *self.done.read().unwrap() {
            return Trans::Pop;
        }
        Trans::None
    }

    fn state_name(&self) -> String {
        return format!("FadeStory: {}", self.duration).to_owned();
    }
}

pub struct FadeState {
    duration: f32,
    color: Color,
    alpha_start: f32,
    alpha_end: f32,
    started: bool,
    alpha: Arc<RwLock<f32>>,
    done: Arc<RwLock<bool>>,
}

impl FadeState {
    pub fn new(seconds: f32, color: Color, start: f32, end: f32, done: Arc<RwLock<bool>>) -> Self {
        FadeState {
            duration: seconds,
            color: color,
            alpha_start: start,
            alpha_end: end,
            started: false,
            alpha: Arc::new(RwLock::new(1.0)),
            done: done,
        }
    }
}

impl State<StoryboardContext> for FadeState {
    fn state_name(&self) -> String {
        return format!("FadeState: {}", self.duration);
    }
    fn update(&mut self, _dt: f32, _ctx: StateData<StoryboardContext>) -> StoryTrans {
        if !self.started {
            self.started = true;

            let alpha = self.alpha.clone();
            let tween = Box::new(TweenState {
                tween: Tween::new(self.alpha_start, self.alpha_end, self.duration),
                tween_fn: TweenFn::EaseInQuad,
                apply_fn: move |value| {
                    *alpha.write().unwrap() = value;
                },
            });

            return Trans::Push(tween);
        }
        let alpha = *self.alpha.read().unwrap();
        if alpha <= 0.0 || alpha >= 1.0 {
            *self.done.write().unwrap() = true;
            return Trans::Pop;
        }
        Trans::None
    }
    fn draw(&mut self, ctx: StateData<StoryboardContext>) {
        let mut parts = self.color.to_rgba();
        let alpha = *self.alpha.read().unwrap();
        parts.3 = (255.0 * alpha) as u8;
        let color = Color::from(parts);

        let ctx = &mut *ctx.data.ctx.borrow_mut();
        let (width, height) = get_size(ctx);

        set_background_color(ctx, WHITE);

        set_color(ctx, color).unwrap();
        let rect = Rect::new(0.0, 0.0, width as f32, height as f32);
        rectangle(ctx, DrawMode::Fill, rect).unwrap();
    }
}
