use std::ops::DerefMut;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use crate::components::*;

pub fn physics(mut space: ResMut<Space>) {
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
}

pub fn draw(sprites: Res<SpriteKinds>, state: Res<GameState>,
    query: Query<(&Position, &Rotation, &Sprite, Option<&Player>, Option<&Asteroid>)>) {
    match state.as_ref() {
        GameState::Playing => {
        	for (position, rotation, sprite, player, asteroid) in query {
        		if let Some(player) = player {
        			if let Some(kind) = sprites.kinds.get(&sprite.key) {
        			    if let Some(texture) = &kind.texture {
            			    draw_texture_ex(texture, position.x, position.y, WHITE,
            			        DrawTextureParams { rotation: rotation.angle, ..Default::default() });
            		    }
        			}
        		} else if let Some(asteroid) = asteroid {
        		    if let Some(kind) = sprites.kinds.get(&sprite.key) {
        		        if let Some(texture) = &kind.texture {
            			    draw_texture_ex(texture, position.x, position.y, WHITE,
            			        DrawTextureParams { rotation: rotation.angle, ..Default::default() });
        		        }
        		    }
        		}
        	}
        },
        GameState::Menu => {},
        GameState::Paused => {},
        GameState::Over => {}
    }
}
