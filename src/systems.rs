use std::ops::DerefMut;
use std::time::Instant;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use crossbeam::channel::unbounded;

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

                if is_key_down(KeyCode::A)     { actions.actions |= Action::MoveLeft; }
                if is_key_down(KeyCode::D)     { actions.actions |= Action::MoveRight; }
                if is_key_down(KeyCode::Up)    { actions.actions |= Action::MoveForward; }
                if is_key_down(KeyCode::Down)  { actions.actions |= Action::MoveBackward; }
                if is_key_down(KeyCode::Left)  { actions.actions |= Action::TurnLeft; }
                if is_key_down(KeyCode::Right) { actions.actions |= Action::TurnRight; }

                if is_key_pressed(KeyCode::Minus)      { actions.actions |= Action::MinimizeSpeed; }
                else if is_key_pressed(KeyCode::Equal) { actions.actions |= Action::MaximizeSpeed; }

                if is_key_down(KeyCode::Q)      { actions.actions |= Action::DecreaseSpeed; }
                else if is_key_down(KeyCode::E) { actions.actions |= Action::IncreaseSpeed; }

                if is_key_down(KeyCode::Space) { actions.actions |= Action::Brake; }
                if is_key_down(KeyCode::W)     { actions.actions |= Action::Shoot; }

                if is_key_down(KeyCode::LeftControl) {
                    if is_key_down(KeyCode::I) && states.scaled.elapsed().as_millis() >= 100 { actions.actions |= Action::ZoomIn; }
                    if is_key_down(KeyCode::O) && states.scaled.elapsed().as_millis() >= 100 { actions.actions |= Action::ZoomOut; }

                    if is_key_pressed(KeyCode::F) {
                        if states.fullscreen { actions.actions |= Action::FullscreenOff; }
                        else { actions.actions |= Action::FullscreenOn; }
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn actions(
    mut query: Query<(&Handle, &Actions, &mut Weapon, &mut Ship), With<Player>>,
    mut space: ResMut<Space>,
    mut states: ResMut<GameStates>) {
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
                                Action::ZoomIn => if states.zoom < 2.0 { states.zoom += 0.1; states.scaled = Instant::now(); },
                                Action::ZoomOut => if states.zoom > 1.0 { states.zoom -= 0.1; states.scaled = Instant::now(); },
                                Action::FullscreenOn => { set_fullscreen(true); states.fullscreen = true; },
                                Action::FullscreenOff => { set_fullscreen(false); states.fullscreen = false; },
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

pub fn physics(mut commands: Commands, mut space: ResMut<Space>, states: Res<GameStates>) {
    match states.state {
        GameState::Playing => {
            let physics = space.physics.deref_mut();
            let (collision_send, collision_recv) = unbounded();
            let (contact_force_send, contact_force_recv) = unbounded();
            let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
 
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
                &event_handler
            );

            while let Ok(event) = contact_force_recv.try_recv() {
                if let Some(collider1) = physics.colliders.get(event.collider1) {
                    if let Some(collider2) = physics.colliders.get(event.collider2) {
                        if let Some(handle1) = collider1.parent() {
                            if let Some(handle2) = collider2.parent() {
                                let id1 = if let Some(body1) = physics.bodies.get(handle1) { body1.user_data } else { 0 as u128 };
                                let id2 = if let Some(body2) = physics.bodies.get(handle2) { body2.user_data } else { 0 as u128 };
                                let entity1 = Entity::from_bits(id1 as u64);
                                let entity2 = Entity::from_bits(id2 as u64);

                                commands.entity(entity1).insert(Collision { entity: entity2 });
                                commands.entity(entity2).insert(Collision { entity: entity1 });
                            }
                        }
                    }
                }
            }
        },
        _ => {}
    }
}

pub fn transformation(
    mut query: Query<(&Handle, &mut Position, &mut Rot, &Center, Option<&Player>)>,
    space: Res<Space>,
    mut states: ResMut<GameStates>) {
    for (handle, mut position, mut rotation, center, player) in &mut query {
        match states.state {
            GameState::Playing => {
                if let Some(handle) = handle.handle {
                    if let Some(body) = space.physics.bodies.get(handle) {
                        position.x = meters_to_pixels(body.translation().x) - center.cx;
                        position.y = meters_to_pixels(body.translation().y) - center.cy;
                        rotation.rotation = body.rotation().clone();
                        rotation.angle = body.rotation().angle();

                        if let Some(player) = player {
                            states.position = Position { x: position.x + center.cx, y: position.y + center.cy };
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn draw(
    query: Query<(&Position, &Rot, &Sprite, &Center, Option<&Ship>)>,
    sprites: Res<SpriteKinds>,
    states: Res<GameStates>) {
    for (position, rotation, sprite, center, ship) in &query {
        match states.state {
            GameState::Playing => {
                // We need to set camera with current zoom and in current position.
                set_camera(&Camera2D {
                    zoom: vec2(1.0 / screen_width(), 1.0 / screen_height()) * states.zoom,
                    target: vec2(states.position.x, states.position.y),
                    ..Camera2D::default() });
                
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
    if states.state == GameState::Playing {
        for (position, center, rotation, size, mut weapon, actions) in &mut query {
            if actions.actions & Action::Shoot != Action::Nothing && weapon.shot.elapsed().as_millis() >= 100 {
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
                    			.ccd_enabled(true)
                    			.build();
                    		let handle = physics.bodies.insert(body);
                    		let collider = ColliderBuilder::cuboid(pixels_to_meters(kind.width) / 2.0, pixels_to_meters(kind.height) / 2.0)
                    			.collision_groups(InteractionGroups::new(Group::GROUP_10, Group::GROUP_2 | Group::GROUP_10))
                    			.active_events(ActiveEvents::CONTACT_FORCE_EVENTS)
                    			.density(1.0)
                    			.friction(0.01)
                    			.build();

                    		physics.colliders.insert_with_parent(collider, handle, &mut physics.bodies);

                            let entity = commands.spawn((
                                Bullet,
                                Position { x: pos.x, y: pos.y },
                                Size     { width: kind.width, height: kind.height },
                                Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
                                Sprite   { key: String::from("bullet-a") },
                                Handle   { handle: Some(handle) },
                                rotation.clone()
                            )).id();

                            if let Some(body) = physics.bodies.get_mut(handle) {
                                body.user_data = entity.to_bits() as u128;
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
                    			.ccd_enabled(true)
                    			.build();
                    		let handle1 = physics.bodies.insert(body1);
                    		let collider1 = ColliderBuilder::cuboid(pixels_to_meters(kind.width) / 2.0, pixels_to_meters(kind.height) / 2.0)
                    			.collision_groups(InteractionGroups::new(Group::GROUP_10, Group::GROUP_2 | Group::GROUP_10))
                    			.active_events(ActiveEvents::CONTACT_FORCE_EVENTS)
                    			.density(1.0)
                    			.friction(0.01)
                    			.build();

                    		physics.colliders.insert_with_parent(collider1, handle1, &mut physics.bodies);

                    		let body2 = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                    			.translation(vector![pixels_to_meters(pos2.x + kind.width / 2.0), pixels_to_meters(pos2.y + kind.height / 2.0)])
                    			.rotation(rotation.angle)
                    			.ccd_enabled(true)
                    			.build();
                    		let handle2 = physics.bodies.insert(body2);
                    		let collider2 = ColliderBuilder::cuboid(pixels_to_meters(kind.width) / 2.0, pixels_to_meters(kind.height) / 2.0)
                    			.collision_groups(InteractionGroups::new(Group::GROUP_10, Group::GROUP_2 | Group::GROUP_10))
                    			.active_events(ActiveEvents::CONTACT_FORCE_EVENTS)
                    			.density(1.0)
                    			.friction(0.01)
                    			.build();

                    		physics.colliders.insert_with_parent(collider2, handle2, &mut physics.bodies);

                            let entity1 = commands.spawn((
                                Bullet,
                                Position { x: pos1.x, y: pos1.y },
                                Size     { width: kind.width, height: kind.height },
                                Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
                                Sprite   { key: String::from("bullet-a") },
                                Handle   { handle: Some(handle1) },
                                rotation.clone()
                            )).id();

                            let entity2 = commands.spawn((
                                Bullet,
                                Position { x: pos2.x, y: pos2.y },
                                Size     { width: kind.width, height: kind.height },
                                Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
                                Sprite   { key: String::from("bullet-a") },
                                Handle   { handle: Some(handle2) },
                                rotation.clone()
                            )).id();

                            if let Some(body1) = physics.bodies.get_mut(handle1) {
                                body1.user_data = entity1.to_bits() as u128;
                                body1.apply_impulse(impulse, true);
                            }

                            if let Some(body2) = physics.bodies.get_mut(handle2) {
                                body2.user_data = entity2.to_bits() as u128;
                                body2.apply_impulse(impulse, true);
                            }
                        }
                    }

                    weapon.shot = Instant::now();
                }
            }
        }
    }
}

pub fn cleaning(
    query: Query<(Entity, &Position, &Size, &Handle), With<Bullet>>,
    mut commands: Commands,
    states: Res<GameStates>,
    mut space: ResMut<Space>) {
    if states.state == GameState::Playing {
        let factor = 3.0 - states.zoom;
        let rect = Rectangle {
            x: states.position.x - screen_width() / 2.0 * factor,
            y: states.position.y - screen_height() / 2.0 * factor,
            width: screen_width() * factor,
            height: screen_height() * factor };
// draw_rectangle(rect.x + 5.0, rect.y + 5.0, rect.width - 10.0, rect.height - 10.0, WHITE);
        for (entity, position, size, handle) in &query {
            if is_outside_of_rect(&position, &size, &rect) {
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
}

pub fn collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collision, &Handle, &Position, &Center, Option<&Bullet>)>,
    states: Res<GameStates>,
    mut space: ResMut<Space>) {
    if states.state == GameState::Playing {
        for (entity, collision, handle, position, center, bullet) in &query {
            if let Some(bullet) = bullet {
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

                    commands.spawn((
                        Spark,
                        Position  { x: position.x + center.cx, y: position.y + center.cy },
                        Animation { frame: 0, speed: 3, count: 3, keys: vec![String::from("spark-a-0"), String::from("spark-a-1"), String::from("spark-a-2")] }
                    ));
                }
            }
        }
    }
}

pub fn effects(
    mut commands: Commands,
    states: Res<GameStates>,
    sprites: Res<SpriteKinds>,
    mut query: Query<(Entity, &Position, &mut Animation), With<Spark>>) {
    if states.state == GameState::Playing {
        for (entity, position, mut animation) in &mut query {
            animation.frame += 1;

            if animation.frame == animation.count * animation.speed {
                commands.entity(entity).despawn();
            } else {
                let key = &animation.keys[(animation.frame / animation.speed) as usize];
                
    			if let Some(kind) = sprites.kinds.get(key) {
    			    if let Some(texture) = &kind.texture {
        			    draw_texture(
        			        texture,
        			        position.x - kind.width / 2.0,
        			        position.y - kind.height / 2.0,
        			        WHITE);
        		    }
    			}
            }
        }
    }
}
