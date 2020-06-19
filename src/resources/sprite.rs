use amethyst::{
    assets::Handle,
    ecs::prelude::World,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::bike::Color;

#[derive(Clone)]
pub struct SpriteResource {
    pub bike_resource: Handle<SpriteSheet>,
}

pub fn init_sprite_resource(
    world: &mut World,
    bike_resource: Handle<SpriteSheet>,
) -> SpriteResource {
    let resource = SpriteResource { bike_resource };
    world.insert(resource.clone());
    resource
}

impl SpriteResource {
    pub fn get_wall_sprite_render(&self, color: Color) -> SpriteRender {
        let sprite_number = match color {
            Color::Red => 3,
            Color::Blue => 2,
            _ => unimplemented!(),
        };
        SpriteRender {
            sprite_sheet: self.bike_resource.clone(),
            sprite_number,
        }
    }
}
