use ggez::event::*;
use ggez::*;
use std::path;

use input;
use storyboard::*;

pub struct Game<'a> {
    pub storyboard: Storyboard<'a>,
}

impl<'a> Game<'a> {
    pub fn new(
        resource_dir: Option<path::PathBuf>,
        ctx: &mut Context,
        stories: Vec<Story<'a>>,
    ) -> Self {
        Game {
            storyboard: Storyboard::new(stories),
        }
    }
}

impl<'a> EventHandler for Game<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        // self.scenes.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if let Some(ev) = self.storyboard.ctx.input_binding.resolve(keycode) {
            // self.scenes.input(ev, true);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(ev) = self.storyboard.ctx.input_binding.resolve(keycode) {
            // self.scenes.input(ev, false);
        }
    }
}
