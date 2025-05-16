use flecs_ecs::prelude::*;
use macroquad::prelude::*;

use crate::loader::{ load_resources, load_world };
use crate::creator::create_space;
use crate::systems::create_systems;

pub mod loader;
pub mod creator;
pub mod components;
pub mod systems;
pub mod helpers;

#[macroquad::main("mocosmos")]
async fn main() -> Result<(), macroquad::Error> {
    let mut world = World::new();

    load_resources(&mut world).await.expect("resources loading error");
    load_world(&mut world, "1");
    create_space(&mut world);
    create_systems(&mut world);

    set_camera(&Camera2D { zoom: vec2(1.0 / screen_width(), 1.0 / screen_height()), ..Camera2D::default() });

    loop {
        clear_background(BLACK);
        world.progress();
        next_frame().await
    }
}
