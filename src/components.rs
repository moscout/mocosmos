use std::collections::HashMap;
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::prelude::*;

#[derive(Clone, Debug)]
pub struct SpriteKind {
	pub key: String,
	pub label: String,
	pub file_name: String,
	pub texture: Option<Texture2D>,
	pub width: f32,
	pub height: f32
}

pub struct Physics {
	pub bodies: RigidBodySet,
	pub colliders: ColliderSet,
	pub pipeline: PhysicsPipeline,

	pub gravity: Vector<f32>,
	pub parameters: IntegrationParameters,
	pub islands: IslandManager,
	pub broad_phase: Box<dyn BroadPhase>,
	pub narrow_phase: NarrowPhase,
	pub impulse_joints: ImpulseJointSet,
	pub multibody_joints: MultibodyJointSet,
	pub solver: CCDSolver,
	pub query_pipeline: QueryPipeline,
	pub hooks: Box<dyn PhysicsHooks>,
	pub events: Box<dyn EventHandler>
}

#[derive(Resource)]
pub struct SpriteKinds {
	pub kinds: HashMap<String, SpriteKind>
}

#[derive(Resource)]
pub enum GameState {
	Menu,
	Playing,
	Paused,
	Over,
}

#[derive(Resource)]
pub struct Space {
	pub physics: Box<Physics> 
}

pub enum WeaponKind {
	OneBullet = 1,
	TwoBullets = 2
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component, Debug)]
pub struct Position {
	pub x: f32,
	pub y: f32
}

#[derive(Component)]
pub struct Center {
	pub cx: f32,
	pub cy: f32
}

#[derive(Component)]
pub struct Size {
	pub width: f32,
	pub height: f32
}

#[derive(Component)]
pub struct Rotation {
	pub angle: f32
}

#[derive(Component)]
pub struct Weapon {
	pub kind: WeaponKind
}

#[derive(Component)]
pub struct Sprite {
	pub key: String
}

#[derive(Component)]
pub struct Handle {
	pub handle: RigidBodyHandle
}
