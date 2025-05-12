use std::ops::DerefMut;

use flecs_ecs::prelude::*;
use macroquad::prelude::*;

use crate::components::*;

pub fn create_systems(world: &mut World) {
    world.system_named::<&mut Space>("physics")
        .each(|space| {
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
        });

    world.system_named::<(&Position, &Rotation, &Sprite, &SpriteKinds, &GameState)>("draw")
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
