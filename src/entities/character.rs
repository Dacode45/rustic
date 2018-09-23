use ggez::graphics::*;
use ggez::Context;
use specs::{Builder, Component, ReadStorage, RunNow, System, VecStorage, World};

use std::collections::HashMap;

use components::*;
use resources::*;
use sprite::*;
use state::*;

pub struct CharacterBuilder {
    id: Option<EntityID>,
    tile_width: Option<usize>,
    pub render: Option<EntityRender>,
    t_pos: Option<TilePosition>,
    anim_map: HashMap<String, Vec<usize>>,
    start_state: Option<Box<State<Character> + Sync>>,
}

impl CharacterBuilder {
    const DEFAULT_TILE_WIDTH: usize = 16;
    pub fn new() -> Self {
        CharacterBuilder {
            id: None,
            tile_width: None,
            render: None,
            t_pos: None,
            anim_map: HashMap::new(),
            start_state: None,
        }
    }

    pub fn id(&mut self, id: &str) {
        self.id = Some(EntityID(id.to_owned()));
    }

    pub fn tile_width(&mut self, tw: usize) {
        self.tile_width = Some(tw);
    }

    pub fn render(&mut self, render: EntityRender) {
        self.render = Some(render);
    }
    pub fn t_pos(&mut self, t_pos: TilePosition) {
        self.t_pos = Some(t_pos);
    }
    pub fn anim(&mut self, name: &str, frames: Vec<usize>) {
        self.anim_map.insert(name.to_owned(), frames);
    }
    pub fn start_state(&mut self, state: Box<State<Character> + Sync>) {
        self.start_state = Some(state);
    }

    pub fn add(self, world: &mut World) {
        let tile_width = match self.tile_width {
            Some(tw) => tw,
            None => CharacterBuilder::DEFAULT_TILE_WIDTH,
        };
        let id = match self.id {
            Some(id) => id,
            None => panic!("No id found"),
        };
        let t_pos = match self.t_pos {
            Some(t_pos) => t_pos,
            None => panic!("No tile position"),
        };
        let render = match self.render {
            Some(render) => render,
            None => panic!("No renderer found"),
        };
        let state = match self.start_state {
            Some(state) => state,
            None => panic!("No start state found"),
        };
        let anim = Animation::new(self.anim_map, "idle", true, None);
        let pos = Position(Point2::new(
            (tile_width * t_pos.x) as f32 + (tile_width as f32 / 2.0) - (render.width as f32 / 2.0),
            (tile_width * (t_pos.y + 1)) as f32 - (render.height as f32),
        ));

        world
            .create_entity()
            .with(id)
            .with(pos)
            .with(t_pos)
            .with(render)
            .with(anim)
            .with(CharacterController::new(state))
            .build();
    }
}
