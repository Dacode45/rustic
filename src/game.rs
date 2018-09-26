use ggez::*;

use std::cell::RefCell;
use std::path;
use std::rc::Rc;

use input;
use resources::*;
use storyboard::*;
use world::*;

pub struct Game {
    pub ctx: Rc<RefCell<Context>>,
    pub storyboard: Storyboard,
    pub input_binding: input::InputBinding,
    events: event::Events,
    pub should_exit: bool,
}
/// use https://github.com/ggez/ggez/issues/295
impl Game {
    pub fn new(
        _resource_dir: Option<path::PathBuf>,
        mut ctx: Context,
        stories: Vec<Story>,
    ) -> Self {
        let events = event::Events::new(&ctx).unwrap();
        let world = World::new(&mut ctx, None);

        Game {
            ctx: Rc::new(RefCell::new(ctx)),
            storyboard: Storyboard::new(world, stories),
            input_binding: input::create_input_binding(),
            events: events,
            should_exit: false,
        }
    }

    pub fn update(&mut self) {
        const DESIRED_FPS: u32 = 60;
        const DESIRED_SPF: f32 = 1.0 / DESIRED_FPS as f32;
        {
            let state = self.storyboard.ctx.borrow_mut();
            let mut dt = state.world.specs_world.write_resource::<DeltaTime>();
            dt.0 = DESIRED_SPF;
        }
        while { timer::check_update_time(&mut self.ctx.borrow_mut(), DESIRED_FPS) } {
            {
                let state = self.storyboard.ctx.borrow_mut();
                let mut input = state.world.specs_world.write_resource::<input::Input>();
                input.0.update(DESIRED_SPF);
            }

            self.should_exit = self
                .storyboard
                .update_storyboard(1.0 / DESIRED_FPS as f32, Rc::clone(&self.ctx));
        }

        // self.scenes.world.assets.sync(ctx);
    }

    pub fn draw(&mut self) {
        graphics::clear(&mut *self.ctx.borrow_mut());
        self.storyboard.draw_storyboard(Rc::clone(&self.ctx));
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
                    event::Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => if let Some(ev) = self.input_binding.resolve(keycode) {
                        let state = self.storyboard.ctx.borrow_mut();
                        let mut input = state.world.specs_world.write_resource::<input::Input>();
                        input.0.update_effect(ev, true);
                        debug!("KeyDown {:?} {:?}", keycode, ev);
                    },
                    event::Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => if let Some(ev) = self.input_binding.resolve(keycode) {
                        let state = self.storyboard.ctx.borrow_mut();
                        let mut input = state.world.specs_world.write_resource::<input::Input>();
                        input.0.update_effect(ev, false);
                    },
                    _ => {}
                }
                match event {
                    event::Event::Quit { .. }
                    | event::Event::KeyDown {
                        keycode: Some(event::Keycode::Escape),
                        ..
                    } => {
                        println!("Quitting");
                        self.should_exit = true;
                    }
                    _ => ()
                    // x => println!("Event fired: {:?}", x),
                }
            }
        } else {
            panic!("Aieee, something else is holding a reference to the context -- events!!")
        }
    }
}
