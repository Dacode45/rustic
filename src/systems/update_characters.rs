use specs;
use specs::prelude::Resources;
use specs::{Read, System, SystemData, Write, WriteStorage};

use std::sync::{Arc, RwLock};

use components::*;
use input::Input;
use resources::*;
use state::*;

pub struct UpdateCharacters;

impl<'a> System<'a> for UpdateCharacters {
    type SystemData = (
        Read<'a, DeltaTime>,
        Read<'a, Input>,
        Read<'a, CurrentMap>,
        Write<'a, Maps>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, TilePosition>,
        WriteStorage<'a, EntityRender>,
        WriteStorage<'a, CharacterController>,
    );

    fn run(
        &mut self,
        (dt, input, c_map, mut maps, mut anim, mut pos, mut t_pos, mut render, mut cc): Self::SystemData,
){
        use specs::Join;
        let mut map = maps.0.remove(&c_map.0).unwrap();
        let sync_map = Arc::new(RwLock::new(map));

        for anim in (&mut anim).join() {
            anim.update(dt.0);
        }

        for (anim, pos, t_pos, render, cc) in
            (&mut anim, &mut pos, &mut t_pos, &mut render, &mut cc).join()
        {
            let mut character = Character::new(
                anim.clone(),
                pos.clone(),
                t_pos.clone(),
                render.clone(),
                sync_map.clone(),
                input.0.clone(),
            );
            if !cc.states.is_running() {
                cc.states.start(StateData::new(&mut character));
            }
            cc.states.update(dt.0, StateData::new(&mut character));
            *anim = character.anim;
            *pos = character.pos;
            *t_pos = character.t_pos;
            *render = character.render;
        }
        let map = Arc::try_unwrap(sync_map).unwrap().into_inner().unwrap();
        maps.0.insert(c_map.0.clone(), map);
    }
    fn setup(&mut self, res: &mut Resources) {
        use specs::prelude::SystemData;
        Self::SystemData::setup(res);
        // self.reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}
