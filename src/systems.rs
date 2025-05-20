use std::ops::DerefMut;
use std::time::Instant;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use rapier2d::{
    na::Point2,
    prelude::*
};

use crate::{
    components::{ Rotation as Rot, * },
    helpers::*
};

pub fn control(mut query: Query<&mut Actions, With<Player>>, states: Res<GameStates>) {
    for mut actions in &mut query {
        match states.state {
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

pub fn actions(mut query: Query<(&Handle, &Actions, &mut Weapon, &mut Ship), With<Player>>, mut space: ResMut<Space>, states: Res<GameStates>) {
    for (handle, actions, mut weapon, mut ship) in &mut query {
        match states.state {
            GameState::Playing => {
                if let Some(handle) = handle.handle {
                    if let Some(body) = space.physics.bodies.get_mut(handle) {
                    	let impulse = 3.90625 * body.mass();
                    	let angular_impulse = 0.7510417 * body.mass();
                        
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

pub fn physics(mut space: ResMut<Space>, states: Res<GameStates>) {
    match states.state {
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

pub fn transformation(mut query: Query<(&Handle, &mut Position, &mut Rot, &Center)>, space: Res<Space>, states: Res<GameStates>) {
    for (handle, mut position, mut rotation, center) in &mut query {
        match states.state {
            GameState::Playing => {
                if let Some(handle) = handle.handle {
                    if let Some(body) = space.physics.bodies.get(handle) {
                        position.x = meters_to_pixels(body.translation().x) - center.cx;
                        position.y = meters_to_pixels(body.translation().y) - center.cy;
                        rotation.rotation = body.rotation().clone();
                        rotation.angle = body.rotation().angle();
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn draw(query: Query<(&Position, &Rot, &Sprite, &Center, Option<&Ship>)>, sprites: Res<SpriteKinds>, states: Res<GameStates>) {
    for (position, rotation, sprite, center, ship) in &query {
        match states.state {
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

pub fn shooting(
    mut query: Query<(&Position, &Center, &Rot, &Size, &mut Weapon, &Actions)>,
    mut commands: Commands,
    states: Res<GameStates>,
    sprites: Res<SpriteKinds>,
    mut space: ResMut<Space>) {
    for (position, center, rotation, size, mut weapon, actions) in &mut query {
        if states.state == GameState::Playing && actions.actions & Action::Shoot != Action::Nothing && weapon.shot.elapsed().as_millis() >= 100 {
            if let Some(kind) = sprites.kinds.get(&String::from("bullet-a")) {
                match weapon.kind {
                    WeaponKind::OneBullet => {
                        let physics = space.physics.deref_mut();
                        let impulse = rotation.rotation.transform_vector(&vector![0.0, -20.0]);

                        let pos = rotate_point(&Point2::new(position.x + center.cx - kind.width / 2.0, position.y - kind.height),
                            position.x + center.cx, position.y + center.cy, rotation.angle);

                		let body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                			.translation(vector![pixels_to_meters(pos.x + kind.width / 2.0), pixels_to_meters(pos.y + kind.height / 2.0)])
                			.rotation(rotation.angle)
                			.build();
                		let handle = physics.bodies.insert(body);
                		let collider = ColliderBuilder::cuboid(pixels_to_meters(kind.width) / 2.0, pixels_to_meters(kind.height) / 2.0)
                			.collision_groups(InteractionGroups::new(Group::GROUP_10, Group::GROUP_2 | Group::GROUP_10))
                			.density(1.0)
                			.friction(0.01)
                			.build();

                		physics.colliders.insert_with_parent(collider, handle, &mut physics.bodies);

                        commands.spawn((
                            Bullet,
                            Position { x: pos.x, y: pos.y },
                            Size     { width: kind.width, height: kind.height },
                            Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
                            Sprite   { key: String::from("bullet-a") },
                            Handle   { handle: Some(handle) },
                            rotation.clone()
                        ));

                        if let Some(body) = physics.bodies.get_mut(handle) {
                            body.apply_impulse(impulse, true);
                        }
                    },

                    WeaponKind::TwoBullets => {
                        let physics = space.physics.deref_mut();
                        let impulse = rotation.rotation.transform_vector(&vector![0.0, -20.0]);
                        
                        let dx = size.width / 3.0;
                        let pos1 = rotate_point(&Point2::new(position.x + dx - kind.width / 2.0, position.y - kind.height),
                            position.x + center.cx, position.y + center.cy, rotation.angle);
                        let pos2 = rotate_point(&Point2::new(position.x + 2.0 * dx - kind.width / 2.0, position.y - kind.height),
                            position.x + center.cx, position.y + center.cy, rotation.angle);

                		let body1 = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                			.translation(vector![pixels_to_meters(pos1.x + kind.width / 2.0), pixels_to_meters(pos1.y + kind.height / 2.0)])
                			.rotation(rotation.angle)
                			.build();
                		let handle1 = physics.bodies.insert(body1);
                		let collider1 = ColliderBuilder::cuboid(pixels_to_meters(kind.width) / 2.0, pixels_to_meters(kind.height) / 2.0)
                			.collision_groups(InteractionGroups::new(Group::GROUP_10, Group::GROUP_2 | Group::GROUP_10))
                			.density(1.0)
                			.friction(0.01)
                			.build();

                		physics.colliders.insert_with_parent(collider1, handle1, &mut physics.bodies);

                		let body2 = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                			.translation(vector![pixels_to_meters(pos2.x + kind.width / 2.0), pixels_to_meters(pos2.y + kind.height / 2.0)])
                			.rotation(rotation.angle)
                			.build();
                		let handle2 = physics.bodies.insert(body2);
                		let collider2 = ColliderBuilder::cuboid(pixels_to_meters(kind.width) / 2.0, pixels_to_meters(kind.height) / 2.0)
                			.collision_groups(InteractionGroups::new(Group::GROUP_10, Group::GROUP_2 | Group::GROUP_10))
                			.density(1.0)
                			.friction(0.01)
                			.build();

                		physics.colliders.insert_with_parent(collider2, handle2, &mut physics.bodies);

                        commands.spawn((
                            Bullet,
                            Position { x: pos1.x, y: pos1.y },
                            Size     { width: kind.width, height: kind.height },
                            Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
                            Sprite   { key: String::from("bullet-a") },
                            Handle   { handle: Some(handle1) },
                            rotation.clone()
                        ));

                        commands.spawn((
                            Bullet,
                            Position { x: pos2.x, y: pos2.y },
                            Size     { width: kind.width, height: kind.height },
                            Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
                            Sprite   { key: String::from("bullet-a") },
                            Handle   { handle: Some(handle2) },
                            rotation.clone()
                        ));

                        if let Some(body1) = physics.bodies.get_mut(handle1) {
                            body1.apply_impulse(impulse, true);
                        }

                        if let Some(body2) = physics.bodies.get_mut(handle2) {
                            body2.apply_impulse(impulse, true);
                        }
                    }
                }

                weapon.shot = Instant::now();
            }
        }
    }
}

pub fn cleaning(
    query: Query<(Entity, &Position, &Size, &Handle), With<Bullet>>,
    mut commands: Commands,
    states: Res<GameStates>,
    mut space: ResMut<Space>) {
    let rect = Rectangle {
        x: -(screen_width() / 2.0) * 2.0,
        y: -(screen_height() / 2.0) * 2.0,
        width: screen_width() * 2.0,
        height: screen_height() * 2.0 };

    for (entity, position, size, handle) in &query {
        if states.state == GameState::Playing && is_outside_of_rect(&position, &size, &rect) {
            if let Some(handle) = handle.handle {
                let physics = space.physics.deref_mut();

                physics.bodies.remove(
                    handle,
                    &mut physics.islands,
                    &mut physics.colliders,
                    &mut physics.impulse_joints,
                    &mut physics.multibody_joints,
                    true);

                commands.entity(entity).despawn();
            }
        }
    }
}
