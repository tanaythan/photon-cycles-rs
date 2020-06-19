use super::{Bike, Wall};

use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Bike>,
        ReadStorage<'s, Wall>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (bikes, walls, transforms): Self::SystemData) {
        for (bike, bike_transform) in (&bikes, &transforms).join() {
            let bike_coords = bike_transform.translation();
            let bike_x = bike_coords.x;
            let bike_y = bike_coords.y;
            for (wall, wall_transform) in (&walls, &transforms).join() {
                let wall_coords = wall_transform.translation();
                let wall_x = wall_coords.x - (wall.width * 0.5);
                let wall_y = wall_coords.y - (wall.height * 0.5);
                if bike_x >= wall_x
                    && bike_x <= wall_x + wall.width
                    && bike_y >= wall_y
                    && bike_y <= wall_y + wall.height
                {
                    println!(
                        "collision: bike {:?} colliding with wall {:?}",
                        bike.color, wall.color
                    );
                }
            }
        }
    }
}
