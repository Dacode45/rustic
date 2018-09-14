use ggez::event::*;
use ggez::*;

use std::cell::RefCell;
use std::path;
use std::rc::Rc;

use input;
use storyboard::*;

pub struct Game {
    pub ctx: Rc<RefCell<Context>>,
    pub storyboard: Storyboard,
    events: event::Events,
    pub should_exit: bool,
}
/// use https://github.com/ggez/ggez/issues/295
impl Game {
    pub fn new(_resource_dir: Option<path::PathBuf>, ctx: Context, stories: Vec<Story>) -> Self {
        let events = event::Events::new(&ctx).unwrap();
        Game {
            ctx: Rc::new(RefCell::new(ctx)),
            storyboard: Storyboard::new(stories),
            events: events,
            should_exit: false,
        }
    }

    pub fn update(&mut self) {
        const DESIRED_FPS: u32 = 15;
        while { timer::check_update_time(&mut self.ctx.borrow_mut(), DESIRED_FPS) } {
            self.storyboard
                .update(1.0 / DESIRED_FPS as f32, Rc::clone(&self.ctx));
        }

        // self.scenes.world.assets.sync(ctx);
    }

    pub fn draw(&mut self) {
        graphics::clear(&mut *self.ctx.borrow_mut());
        self.storyboard.draw(Rc::clone(&self.ctx));
        graphics::present(&mut *self.ctx.borrow_mut());
    }

    pub fn handle_events(&mut self) {
        if let Ok(ctx) = self.ctx.try_borrow_mut().as_mut() {
            // Tell the timer stuff a frame has happened.
            // Without this the FPS timer functions and such won't work.
            ctx.timer_context.tick();
            // Handle events
            for event in self.events.poll() {
                ctx.process_event(&event);
                match event {
                    event::Event::Quit { .. }
                    | event::Event::KeyDown {
                        keycode: Some(event::Keycode::Escape),
                        ..
                    } => {
                        println!("Quitting");
                        self.should_exit = true;
                    }
                    x => println!("Event fired: {:?}", x),
                }
            }
        } else {
            panic!("Aieee, something else is holding a reference to the context -- events!!")
        }
    }
}
