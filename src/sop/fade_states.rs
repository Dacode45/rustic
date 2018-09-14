use ggez::graphics::*;

use std::cell::RefCell;
use std::rc::Rc;

use state::*;
use storyboard::*;

use super::tween_state::TweenState;
use tween::*;

/// goes from opague to transparent
pub fn fade_in(duration: f32, color: Color) -> Story {
    Story::Run(Box::new(FadeStory::new(duration, color, 1.0, 0.0)))
}

// goes from transparent to opague
pub fn fade_out(duration: f32, color: Color) -> Story {
    Story::Run(Box::new(FadeStory::new(duration, color, 0.0, 1.0)))
}

#[derive(Clone)]
pub struct FadeStory {
    duration: f32,
    color: Color,
    alpha_start: f32,
    alpha_end: f32,
    started: bool,
    done: Rc<RefCell<bool>>,
}

impl FadeStory {
    pub fn new(seconds: f32, color: Color, start: f32, end: f32) -> Self {
        FadeStory {
            duration: seconds,
            color: color,
            alpha_start: start,
            alpha_end: end,
            started: false,
            done: Rc::new(RefCell::new(false)),
        }
    }
}

impl State<StoryboardContext> for FadeStory {
    fn update(&mut self, dt: f32, ctx: StateData<StoryboardContext>) -> StoryTrans {
        if !self.started {
            self.started = true;
            return Trans::Push(Box::new(FadeState::new(
                self.duration,
                self.color,
                self.alpha_start,
                self.alpha_end,
                Rc::clone(&self.done),
            )));
        }
        if *self.done.borrow() {
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
    alpha: Rc<RefCell<f32>>,
    done: Rc<RefCell<bool>>,
}

impl FadeState {
    pub fn new(seconds: f32, color: Color, start: f32, end: f32, done: Rc<RefCell<bool>>) -> Self {
        FadeState {
            duration: seconds,
            color: color,
            alpha_start: start,
            alpha_end: end,
            started: false,
            alpha: Rc::new(RefCell::new(1.0)),
            done: done,
        }
    }
}

impl State<StoryboardContext> for FadeState {
    fn state_name(&self) -> String {
        return format!("FadeState: {}", *self.alpha.borrow()).to_owned();
    }
    fn update(&mut self, dt: f32, ctx: StateData<StoryboardContext>) -> StoryTrans {
        if !self.started {
            self.started = true;

            let alpha = Rc::clone(&self.alpha);
            let tween = Box::new(TweenState {
                tween: Tween::new(self.alpha_start, self.alpha_end, self.duration),
                tween_fn: Box::new(ease_in_quad),
                apply_fn: Box::new(move |value| {
                    *alpha.borrow_mut() = value;
                }),
            });

            return Trans::Push(tween);
        }
        let alpha = *self.alpha.borrow();
        if alpha <= 0.0 || alpha >= 1.0 {
            *self.done.borrow_mut() = true;
            return Trans::Pop;
        }
        Trans::None
    }
    fn draw(&mut self, ctx: StateData<StoryboardContext>) {
        let mut parts = self.color.to_rgba();
        let alpha = *self.alpha.borrow();
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
