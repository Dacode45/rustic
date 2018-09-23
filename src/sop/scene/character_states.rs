use entities;
use storyboard::*;

pub fn add_character() -> Story {
    return Story::Setup(Box::new(move |ctx| {
        let state = &mut *ctx.state.borrow_mut();
        let ctx = &mut *ctx.ctx.borrow_mut();
        entities::add_character(ctx, &mut state.world.specs_world);
        return Story::Done("add_character".to_owned());
    }));
}
