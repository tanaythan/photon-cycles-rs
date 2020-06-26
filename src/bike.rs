use amethyst::ecs::{Component, DenseVecStorage};

const BOOST_DURATION: f32 = 2f32;
const BOOST_COOLDOWN: f32 = 10f32;
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
    pub boost: Boost,
}

#[derive(Debug, Default)]
pub struct Boost {
    pub is_boosted: bool,
    pub time_left: Option<f32>,
}

const BIKE_HEIGHT: f32 = 20.0;
const BIKE_WIDTH: f32 = 8.0;

impl Bike {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            width: BIKE_WIDTH,
            height: BIKE_HEIGHT,
            boost: Boost::default(),
        }
    }

    pub fn boost_action(&mut self) {
        if !self.boost.is_boosted && self.boost.time_left.is_none() {
            self.boost.time_left = Some(BOOST_DURATION);
            self.boost.is_boosted = true;
        }
    }

    pub fn reset_boost(&mut self) {
        if self.boost.is_boosted {
            self.boost.is_boosted = false;
            self.boost.time_left = Some(BOOST_COOLDOWN);
        } else {
            self.boost.time_left = None;
        }
    }
}

impl Component for Bike {
    type Storage = DenseVecStorage<Self>;
}
