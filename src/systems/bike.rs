use super::Bike;

use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct BikeSystem;

const BIKE_VELOCITY: f32 = 70f32;
const BIKE_BOOST_VELOCITY: f32 = 140f32;
const TURN_VELOCITY: f32 = 2.3;

impl<'s> System<'s> for BikeSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Bike>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut bikes, input, time): Self::SystemData) {
        let ds = time.delta_seconds();
        for (bike, transform) in (&mut bikes, &mut transforms).join() {
            use super::Color;
            let movement = match bike.color {
                Color::Blue => input.axis_value("bike_blue"),
                Color::Red => input.axis_value("bike_red"),
                Color::Green | Color::Yellow => unimplemented!(),
            };

            if let Some(ref mut bt) = bike.boost.time_left {
                *bt -= ds;
                if *bt < 0f32 {
                    bike.reset_boost();
                }
            }

            let boost = match bike.color {
                Color::Blue => input.action_is_down("bike_blue_boost"),
                Color::Red => input.action_is_down("bike_red_boost"),
                Color::Green | Color::Yellow => unimplemented!(),
            }
            .unwrap_or(false);

            if boost {
                bike.boost_action();
            }

            let velocity = if bike.boost.is_boosted {
                BIKE_BOOST_VELOCITY
            } else {
                BIKE_VELOCITY
            } * ds;
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    transform.prepend_rotation_z_axis(mv_amount * TURN_VELOCITY * ds);
                }
            }
            let (_rot_x, _rot_y, rot_z) = transform.euler_angles();
            let rot_z = rot_z + std::f32::consts::PI / 2f32;
            let y_vel = rot_z.sin() * velocity;
            let x_vel = rot_z.cos() * velocity;
            transform.prepend_translation_x(x_vel);
            transform.prepend_translation_y(y_vel);
        }
    }
}
