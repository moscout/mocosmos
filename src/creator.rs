use bevy_ecs::prelude::*;
use rapier2d::prelude::*;

use crate::components::{ Space, Physics };

pub fn create_space(world: &mut World) {
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

	world.insert_resource(Space {
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
