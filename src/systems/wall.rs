use super::{resources::SpriteResource, Bike, Wall};

use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, SystemData,
};
use amethyst::prelude::*;

const FRAME_GAP: f32 = 0.8f32;

#[derive(SystemDesc, Default)]
pub struct WallSystem;

impl<'s> System<'s> for WallSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Bike>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
    );
    fn run(
        &mut self,
        (entities, time, transforms, bikes, resource_handle, lazy_update): Self::SystemData,
    ) {
        if time.absolute_real_time().as_secs_f32() % FRAME_GAP < FRAME_GAP / 2f32 {
            return;
        }
        for (bike, transform) in (&bikes, &transforms).join() {
            let (_rot_x, _rot_y, _rot_z) = transform.euler_angles();

            let mut new_transform = transform.clone();
            new_transform.append_translation_xyz(0.0, -bike.height * 0.5, 0.0);
            let sprite_render = resource_handle.get_wall_sprite_render(bike.color);

            lazy_update
                .create_entity(&entities)
                .with(new_transform)
                .with(Wall::new(bike.color))
                .with(sprite_render)
                .build();
        }
    }
}
