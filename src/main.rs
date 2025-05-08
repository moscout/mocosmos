use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::loader::{ load_resources, load_world };
use crate::creator::{ create_space };
use crate::systems::{ physics, draw };

pub mod loader;
pub mod creator;
pub mod components;
pub mod systems;

#[macroquad::main("mocosmos")]
async fn main() -> Result<(), macroquad::Error> {
    let mut world = World::new();
    let mut schedule = Schedule::default();

    load_resources(&mut world).await.expect("resources loading error");
    load_world(&mut world, "1");
    create_space(&mut world);

    schedule.add_systems((physics, draw));

    loop {
        clear_background(BLACK);
        schedule.run(&mut world);
        next_frame().await
    }
}
