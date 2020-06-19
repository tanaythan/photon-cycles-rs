use amethyst::ecs::{Component, DenseVecStorage};

// TODO: Add support for two other players
#[derive(Debug, Copy, Clone)]
pub enum Color {
    Blue,
    Red,
    #[allow(dead_code)]
    Yellow,
    #[allow(dead_code)]
    Green,
}

#[derive(Debug)]
pub struct Bike {
    pub color: Color,
    pub width: f32,
    pub height: f32,
}

const BIKE_HEIGHT: f32 = 20.0;
const BIKE_WIDTH: f32 = 8.0;

impl Bike {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            width: BIKE_WIDTH,
            height: BIKE_HEIGHT,
        }
    }
}

impl Component for Bike {
    type Storage = DenseVecStorage<Self>;
}
