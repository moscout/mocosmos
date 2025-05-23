use macroquad::prelude::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::loader::*;
use crate::creator::*;
use crate::systems::*;

pub mod loader;
pub mod creator;
pub mod components;
pub mod systems;
pub mod helpers;

#[macroquad::main("mocosmos")]
async fn main() -> Result<(), macroquad::Error> {
    let mut app = App::new();

    app.add_systems(Startup, (
            load_world,
            create_space.after(load_world)
        )).add_systems(Update, (
            control,
            actions.after(control),
            physics.after(actions),
            transformation.after(physics),
            draw.after(transformation),
            shooting,
            collisions,
            effects,
            cleaning
        ));

    load_resources(app.world_mut()).await.expect("resources loading error");
    
    set_camera(&Camera2D {
        zoom: vec2(1.0 / screen_width(), 1.0 / screen_height()) * 2.0,
        ..Camera2D::default() });

    loop {
        clear_background(BLACK);
        
        app.update();

        if let Some(exit) = app.should_exit() {
            if exit.is_error() { std::process::exit(1); }
            else { break Result::Ok(()); }
        }
        
        next_frame().await
    }
}
