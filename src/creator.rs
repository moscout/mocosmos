use bevy_ecs::prelude::*;
use rapier2d::prelude::*;

use crate::components::{ Rotation as Rot, * };
use crate::helpers::pixels_to_meters;

pub fn create_space(
	mut query: Query<(&Position, &Rot, &Size, &Center, &mut Handle, Option<&Player>, Option<&Asteroid>)>,
	mut commands: Commands) {
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
	let pipeline = PhysicsPipeline::new();

	let gravity = vector![0.0, 0.0];
	let parameters = IntegrationParameters::default();
	let islands = IslandManager::new();
	let broad_phase = Box::new(DefaultBroadPhase::new());
	let narrow_phase = NarrowPhase::new();
	let impulse_joints = ImpulseJointSet::new();
	let multibody_joints = MultibodyJointSet::new();
	let solver = CCDSolver::new();
	let query_pipeline = QueryPipeline::new();
	let hooks = Box::new(());
	let events = Box::new(());

	for (position, rotation, size, center, mut handle, player, asteroid) in &mut query {
		let groups = if let Some(player) = player {
			InteractionGroups::new(Group::GROUP_1, Group::GROUP_1 | Group::GROUP_2)
		} else if let Some(asteroid) = asteroid {
			InteractionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_10)
		} else {
			InteractionGroups::new(Group::GROUP_32, Group::GROUP_32)
		};
			
		let body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
			.translation(vector![pixels_to_meters(position.x + center.cx), pixels_to_meters(position.y + center.cy)])
			.rotation(rotation.angle)
			.build();
		let _handle = bodies.insert(body);
		let collider = ColliderBuilder::cuboid(pixels_to_meters(size.width) / 2.0, pixels_to_meters(size.height) / 2.0)
			.collision_groups(groups)
			.density(1.0)
			.friction(0.1)
			.build();

		colliders.insert_with_parent(collider, _handle, &mut bodies);
		handle.handle = Some(_handle);
	}

	commands.insert_resource(Space {
		physics: Box::new(Physics {
			bodies,
			colliders,
			pipeline,
			gravity,
			parameters,
			islands,
			broad_phase,
			narrow_phase,
			impulse_joints,
			multibody_joints,
			solver,
			query_pipeline,
			hooks,
			events
		})
	});
}
