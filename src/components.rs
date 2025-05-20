use std::collections::HashMap;
use std::time::Instant;
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use bitflags::bitflags;

use rapier2d::{
	math::Rotation as Rot,
	prelude::*
};

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct Action: u64 {
		const Nothing =			0b0000000000000000000000000000000000000000000000000000000000000000;
		const OneBullet =		0b0000000000000000000000000000000000000000000000000000000000000001;
		const TwoBullets =		0b0000000000000000000000000000000000000000000000000000000000000010;
		const Weapon3 =			0b0000000000000000000000000000000000000000000000000000000000000100;
		const Weapon4 =			0b0000000000000000000000000000000000000000000000000000000000001000;
		const Weapon5 =			0b0000000000000000000000000000000000000000000000000000000000010000;
		const Weapon6 =			0b0000000000000000000000000000000000000000000000000000000000100000;
		const Weapon7 =			0b0000000000000000000000000000000000000000000000000000000001000000;
		const Weapon8 =			0b0000000000000000000000000000000000000000000000000000000010000000;
		const Weapon9 =			0b0000000000000000000000000000000000000000000000000000000100000000;
		const MoveForward =		0b0000000000000000000000000000000000000000000000000000001000000000;
		const MoveBackward =	0b0000000000000000000000000000000000000000000000000000010000000000;
		const MoveLeft =		0b0000000000000000000000000000000000000000000000000000100000000000;
		const MoveRight =		0b0000000000000000000000000000000000000000000000000001000000000000;
		const TurnLeft =		0b0000000000000000000000000000000000000000000000000010000000000000;
		const TurnRight =		0b0000000000000000000000000000000000000000000000000100000000000000;
		const IncreaseSpeed =	0b0000000000000000000000000000000000000000000000001000000000000000;
		const DecreaseSpeed =	0b0000000000000000000000000000000000000000000000010000000000000000;
		const MaximizeSpeed =	0b0000000000000000000000000000000000000000000000100000000000000000;
		const MinimizeSpeed =	0b0000000000000000000000000000000000000000000001000000000000000000;
		const Brake =			0b0000000000000000000000000000000000000000000010000000000000000000;
		const Shoot =			0b0000000000000000000000000000000000000000000100000000000000000000;
	}
}

pub struct Rectangle {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32
}

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

#[derive(PartialEq, Eq)]
pub enum GameState {
	Menu,
	Playing,
	Paused,
	Over,
}

#[derive(Resource)]
pub struct GameStates {
	pub state: GameState
}

#[derive(Resource)]
pub struct SpriteKinds {
	pub kinds: HashMap<String, SpriteKind>
}

#[derive(Resource)]
pub struct Space {
	pub physics: Box<Physics> 
}

pub enum WeaponKind {
	OneBullet = 1,
	TwoBullets = 2
}

pub struct Trace {
	pub key: String
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Debug)]
pub struct Position {
	pub x: f32,
	pub y: f32
}

#[derive(Component, Debug)]
pub struct Center {
	pub cx: f32,
	pub cy: f32
}

#[derive(Component, Debug)]
pub struct Size {
	pub width: f32,
	pub height: f32
}

#[derive(Component, Clone)]
pub struct Rotation {
	pub angle: f32,
	pub rotation: Rot<f32>
}

#[derive(Component)]
pub struct Weapon {
	pub kind: WeaponKind,
	pub shot: Instant
}

#[derive(Component)]
pub struct Sprite {
	pub key: String
}

#[derive(Component)]
pub struct Handle {
	pub handle: Option<RigidBodyHandle>
}

#[derive(Component)]
pub struct Ship {
	pub speed: i32,
	pub tracing: bool,
	pub trace: Trace
}

#[derive(Component)]
pub struct Actions {
	pub actions: Action
}
