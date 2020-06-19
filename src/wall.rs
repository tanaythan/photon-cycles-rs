#![deny(unused_imports)]
use super::bike::Color;

use amethyst::ecs::{Component, DenseVecStorage};

const WALL_HEIGHT: f32 = 3.0;
const WALL_WIDTH: f32 = 3.0;

pub struct Wall {
    pub height: f32,
    pub width: f32,
    pub color: Color,
}

impl Wall {
    pub fn new(color: Color) -> Self {
        Self {
            height: WALL_HEIGHT,
            width: WALL_WIDTH,
            color,
        }
    }
}

impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}
