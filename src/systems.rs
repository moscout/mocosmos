use std::ops::DerefMut;

use flecs_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::{
    components::{ Rotation as Rot, * },
    helpers::*
};

pub fn create_systems(world: &mut World) {
    control(world);
    actions(world);
    physics(world);
    transformation(world);
    draw(world);
}

fn control(world: &mut World) {
    world.system_named::<(&mut Actions, &GameState)>("control")
        .term_at(1)
        .singleton()
        .with::<&Player>()
        .each(|(actions, state)| {
            match state {
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
        });
}

fn actions(world: &mut World) {
    world.system_named::<(&Handle, &Actions, &mut Weapon, &mut Ship, &mut Space, &mut GameState)>("actions")
        .term_at(4)
        .singleton()
        .term_at(5)
        .singleton()
        .with::<&Player>()
        .each(|(handle, actions, weapon, ship, space, state)| {
            match state {
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

    world.system_named::<(&Position, &Rot, &Sprite, &Center, &Ship, &SpriteKinds, &GameState)>("draw-traces")
        .term_at(5)
        .singleton()
        .term_at(6)
        .singleton()
        .each(|(position, rotation, sprite, center, ship, sprites, state)| {
            match state {
                GameState::Playing => {
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
                },
                GameState::Menu => {},
                GameState::Paused => {},
                GameState::Over => {}
            }
        });
}
