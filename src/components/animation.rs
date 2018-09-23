use specs::{Component, VecStorage, World};

use std::cmp;
use std::collections::HashMap;

pub enum Facing {
    Up,
    Right,
    Down,
    Left,
}

impl Facing {
    pub fn name(&self) -> &'static str {
        match self {
            Facing::Up => "up",
            Facing::Down => "down",
            Facing::Left => "left",
            Facing::Right => "right",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub map: HashMap<String, Vec<usize>>,
    pub frames: Vec<usize>,
    pub should_loop: bool,
    pub spf: f32,

    index: usize,
    time: f32,
}

impl Animation {
    pub fn new(
        map: HashMap<String, Vec<usize>>,
        start: &str,
        should_loop: bool,
        spf: Option<f32>,
    ) -> Self {
        let spf = if let Some(v) = spf { v } else { 0.12 };
        let mut anim = Animation {
            map,
            frames: Vec::new(),
            should_loop,
            spf,

            index: 0,
            time: 0.0,
        };
        let frames = anim.map.get(start).unwrap().clone();
        anim.set_frames(frames);
        anim
    }

    pub fn update(&mut self, dt: f32) {
        self.time = self.time + dt;

        if self.time >= self.spf {
            self.index = self.index + 1;
            self.time = 0.0;

            if self.index >= self.frames.len() {
                if self.should_loop {
                    self.index = 0;
                } else {
                    self.index = self.frames.len() - 1;
                }
            }
        }
    }

    pub fn set_frames(&mut self, frames: Vec<usize>) {
        self.frames = frames;
        self.index = cmp::min(self.index, self.frames.len() - 1);
    }

    pub fn frame(&self) -> usize {
        return self.frames[self.index];
    }

    pub fn is_finished(&self) -> bool {
        !self.should_loop && self.index == self.frames.len() - 1
    }
}

impl Component for Animation {
    type Storage = VecStorage<Self>;
}
