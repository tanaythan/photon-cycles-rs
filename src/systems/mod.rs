#![deny(dead_code)]
#![deny(unused_imports)]
mod bike;
mod collision;
mod wall;

pub use bike::BikeSystem;
pub use collision::CollisionSystem;
pub use wall::WallSystem;

use super::bike::Bike;
use super::bike::Color;
use super::resources;
use super::wall::Wall;
