use std::ops::DerefMut;

use flecs_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{
    components::*,
    components::Rotation as Rot,
    helpers::meters_to_pixels
};

pub fn create_systems(world: &mut World) {
    control(world);
    physics(world);
    transformation(world);
    draw(world);
}

fn control(world: &mut World) {
    world.system_named::<(&Handle, &mut Weapon, &mut Ship, &mut Space, &mut GameState)>("control")
        .term_at(3)
        .singleton()
        .term_at(4)
        .singleton()
        .with::<&Player>()
        .each(|(handle, weapon, ship, space, state)| {
            match state {
                GameState::Playing => {
                    if let Some(handle) = handle.handle {
                        if let Some(mut body) = space.physics.bodies.get_mut(handle) {
                        }
                    }
                },
                GameState::Menu => {},
                GameState::Paused => {},
                GameState::Over => {}
            }
        });
}

fn physics(world: &mut World) {
     world.system_named::<(&mut Space, &GameState)>("physics")
        .term_at(0)
        .singleton()
        .term_at(1)
        .singleton()
        .each(|(space, state)| {
            match state {
                GameState::Playing => {
                    let physics = space.physics.deref_mut();

                    physics.pipeline.step(
                        &physics.gravity,
                        &physics.parameters,
                        &mut physics.islands,
                        physics.broad_phase.deref_mut(),
                        &mut physics.narrow_phase,
                        &mut physics.bodies,
                        &mut physics.colliders,
                        &mut physics.impulse_joints,
                        &mut physics.multibody_joints,
                        &mut physics.solver,
                        Some(&mut physics.query_pipeline),
                        physics.hooks.deref_mut(),
                        physics.events.deref_mut()
                    );
                },
                _ => {}
            }
        });
}

fn transformation(world: &mut World) {
    world.system_named::<(&Handle, &mut Position, &mut Rot, &Center, &Space, &GameState)>("transformation")
        .term_at(4)
        .singleton()
        .term_at(5)
        .singleton()
        .each(|(handle, position, rotation, center, space, state)| {
            match state {
                GameState::Playing => {
                    if let Some(handle) = handle.handle {
                        if let Some(body) = space.physics.bodies.get(handle) {
                            position.x = meters_to_pixels(body.translation().x) - center.cx;
                            position.y = meters_to_pixels(body.translation().y) - center.cy;
                            rotation.angle = body.rotation().angle();
                        }
                    }
                },
                _ => {}
            }
        });
}

fn draw(world: &mut World) {
    world.system_named::<(&Position, &Rot, &Sprite, &SpriteKinds, &GameState)>("draw")
        .term_at(3)
        .singleton()
        .term_at(4)
        .singleton()
        .each(|(position, rotation, sprite, sprites, state)| {
            match state {
                GameState::Playing => {
        			if let Some(kind) = sprites.kinds.get(&sprite.key) {
        			    if let Some(texture) = &kind.texture {
            			    draw_texture_ex(texture, position.x, position.y, WHITE,
            			        DrawTextureParams { rotation: rotation.angle, ..Default::default() });
            		    }
        			}
                },
                GameState::Menu => {},
                GameState::Paused => {},
                GameState::Over => {}
            }
        });
}
