use std::ops::DerefMut;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{
    components::{ Rotation as Rot, * },
    helpers::*
};

pub fn control(mut query: Query<&mut Actions, With<Player>>, mut state: ResMut<GameState>) {
    for mut actions in &mut query {
        match state.deref_mut() {
            GameState::Playing => {
                actions.actions = Action::Nothing;
        
                if is_key_pressed(KeyCode::Key1)      { actions.actions |= Action::OneBullet; }
                else if is_key_pressed(KeyCode::Key2) { actions.actions |= Action::TwoBullets; }

                if is_key_down(KeyCode::A)     { actions.actions |= Action::MoveLeft }
                if is_key_down(KeyCode::D)     { actions.actions |= Action::MoveRight }
                if is_key_down(KeyCode::Up)    { actions.actions |= Action::MoveForward }
                if is_key_down(KeyCode::Down)  { actions.actions |= Action::MoveBackward }
                if is_key_down(KeyCode::Left)  { actions.actions |= Action::TurnLeft }
                if is_key_down(KeyCode::Right) { actions.actions |= Action::TurnRight }

                if is_key_pressed(KeyCode::Minus)      { actions.actions |= Action::MinimizeSpeed }
                else if is_key_pressed(KeyCode::Equal) { actions.actions |= Action::MaximizeSpeed }

                if is_key_down(KeyCode::Q)      { actions.actions |= Action::DecreaseSpeed }
                else if is_key_down(KeyCode::E) { actions.actions |= Action::IncreaseSpeed }

                if is_key_down(KeyCode::Space) { actions.actions |= Action::Brake }
                if is_key_down(KeyCode::W)     { actions.actions |= Action::Shoot }
            },
            _ => {}
        }
    }
}

pub fn actions(mut query: Query<(&Handle, &Actions, &mut Weapon, &mut Ship), With<Player>>, mut space: ResMut<Space>, mut state: ResMut<GameState>) {
    for (handle, actions, mut weapon, mut ship) in &mut query {
        match state.deref_mut() {
            GameState::Playing => {
                if let Some(handle) = handle.handle {
                    if let Some(body) = space.physics.bodies.get_mut(handle) {
                    	let impulse = 3.90625 * body.mass();
                    	let angular_impulse = 0.6510417 * body.mass();
                        
                        let rotation = body.rotation();
                        let vec_left = rotation.transform_vector(&vector![-impulse, 0.0]);
                        let vec_right = rotation.transform_vector(&vector![impulse, 0.0]);
                    	let vec_forward = rotation.transform_vector(&vector![0.0, -impulse]);
                    	let vec_backward = rotation.transform_vector(&vector![0.0, impulse]);
                    	
                        for action in actions.actions {
                            match action {
                                Action::OneBullet => weapon.kind = WeaponKind::OneBullet,
                                Action::TwoBullets => weapon.kind = WeaponKind::TwoBullets,
                                Action::MoveLeft => body.apply_impulse(vec_left, true),
                                Action::MoveRight => body.apply_impulse(vec_right, true),
                                Action::MoveForward => body.apply_impulse(vec_forward, true),
                                Action::MoveBackward => body.apply_impulse(vec_backward, true),
                                Action::TurnLeft => body.apply_torque_impulse(-angular_impulse, true),
                                Action::TurnRight => body.apply_torque_impulse(angular_impulse, true),
                                Action::DecreaseSpeed if ship.speed > 0 => ship.speed = ship.speed - 1,
                                Action::IncreaseSpeed if ship.speed < 50 => ship.speed = ship.speed + 1,
                                Action::MinimizeSpeed => ship.speed = 0,
                                Action::MaximizeSpeed => ship.speed = 50,
                                Action::Brake => {
                                    let linear_damping = body.linear_damping();
                                    let angular_damping = body.angular_damping();

                                    if linear_damping < 100.0 { body.set_linear_damping(linear_damping * 1.2 + 0.5); }
                                    if angular_damping < 100.0 { body.set_angular_damping(angular_damping * 1.2 + 0.5); }
                                },
                                Action::Shoot => {},
                                _ => ()
                            }
                        }

                        if actions.actions & Action::Brake == Action::Nothing {
                            body.set_linear_damping((50.0 - ship.speed as f32) / 10.0);
                            body.set_angular_damping((50.0 - ship.speed as f32) / 10.0);
                        }

                        ship.tracing = actions.actions & (
                            Action::MoveForward  |
                            Action::MoveBackward |
                            Action::MoveLeft     |
                            Action::MoveRight)   != Action::Nothing;
                    }
                }
            },
            GameState::Menu => {},
            GameState::Paused => {},
            GameState::Over => {}
        }
    }
}

pub fn physics(mut space: ResMut<Space>, mut state: ResMut<GameState>) {
    match state.deref_mut() {
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
}

pub fn transformation(mut query: Query<(&Handle, &mut Position, &mut Rot, &Center)>, space: Res<Space>, mut state: ResMut<GameState>) {
    for (handle, mut position, mut rotation, center) in &mut query {
        match state.deref_mut() {
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
    }
}

pub fn draw(mut query: Query<(&Position, &Rot, &Sprite, &Center, Option<&Ship>)>, sprites: Res<SpriteKinds>, mut state: ResMut<GameState>) {
    for (position, rotation, sprite, center, ship) in &mut query {
        match state.deref_mut() {
            GameState::Playing => {
    			if let Some(kind) = sprites.kinds.get(&sprite.key) {
    			    if let Some(texture) = &kind.texture {
        			    draw_texture_ex(texture, position.x, position.y, WHITE,
        			        DrawTextureParams { rotation: rotation.angle, ..Default::default() });
        		    }
    			}

                if let Some(ship) = ship {
        			if let Some(kind) = sprites.kinds.get(&sprite.key) {
            			if let Some(trace) = sprites.kinds.get(&ship.trace.key) {
            			    if let Some(texture) = &trace.texture {
            			        if ship.tracing {
                    			    draw_texture_ex(
                    			        texture,
                    			        position.x + center.cx - trace.width / 2.0,
                    			        position.y + kind.height,
                    			        WHITE,
                    			        DrawTextureParams {
                    			            rotation: rotation.angle,
                    			            pivot: Some(vec2(position.x + center.cx, position.y + center.cy)),
                    			            ..Default::default() });
            			        }
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
}
