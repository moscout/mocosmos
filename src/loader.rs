use std::env::current_exe;
use std::collections::HashMap;
use std::time::Instant;
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use rapier2d::na::UnitComplex;
use rusqlite::{ Connection, Result };

use crate::components::*;

pub async fn load_resources(world: &mut World) -> Result<()> {
    set_pc_assets_folder("assets");

	let path = match current_exe() {
		Ok(path) => path.with_file_name("space.db").into_os_string().into_string(),
		Err(err) => panic!("Can't get executable path: {}", err)
	};
	
	let conn = Connection::open(path.unwrap())?;
	let mut stmt = conn.prepare("select key, label, file_name from sprite")?;

	let iter = stmt.query_map([], |row| {
		Ok(SpriteKind {
			key: row.get(0)?,
			label: row.get(1)?,
			file_name: row.get(2)?,
			texture: None,
			width: 0.0,
			height: 0.0
		})
	})?;

	let mut sprites = SpriteKinds { kinds: HashMap::new() };

	for kind in iter {
		let mut k = kind.unwrap().clone();
		let texture = load_texture(&k.file_name).await.expect("can't load texture");
		
		texture.set_filter(FilterMode::Linear);
		k.width = texture.width();
		k.height = texture.height();
		k.texture = Some(texture);

		sprites.kinds.insert(k.key.clone(), k);
	}

	build_textures_atlas();

	world.insert_resource(sprites);
    world.insert_resource(GameStates {
    	state: GameState::Playing,
    	zoom: 1.0,
    	scaled: Instant::now(),
    	position: Position { x: 500.0, y: 500.0 },
    	fullscreen: false
    });

	Ok(())
}

pub fn load_world(mut commands: Commands, sprites: Res<SpriteKinds>) {
	if let Some(kind) = sprites.kinds.get("player-ship-a-blue") {
		commands.spawn((
			Player,
			Position { x: 500.0, y: 500.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("player-ship-a-blue") },
			Weapon   { kind: WeaponKind::OneBullet, shot: Instant::now() },
			Ship     { speed: 50, tracing: false, trace: Trace { key: String::from("trace-b-wide") }},
			Actions  { actions: Action::Nothing },
			Handle   { handle: None }
		));
	}
	
	if let Some(kind) = sprites.kinds.get("asteroid-a-grey-big") {
		commands.spawn((
			Asteroid,
			Position { x: 0.0, y: 0.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("asteroid-a-grey-big") },
			Handle   { handle: None }
		));
	}

	if let Some(kind) = sprites.kinds.get("asteroid-a-brown-small") {
		commands.spawn((
			Asteroid,
			Position { x: 1000.0, y: 0.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("asteroid-a-brown-small") },
			Handle   { handle: None }
		));
	}

	if let Some(kind) = sprites.kinds.get("asteroid-c-brown-big") {
		commands.spawn((
			Asteroid,
			Position { x: 0.0, y: 1000.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("asteroid-c-brown-big") },
			Handle   { handle: None }
		));
	}
		
	if let Some(kind) = sprites.kinds.get("asteroid-b-brown-tiny") {
		commands.spawn((
			Asteroid,
			Position { x: 1000.0, y: 1000.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("asteroid-b-brown-tiny") },
			Handle   { handle: None }
		));
	}

	if let Some(kind) = sprites.kinds.get("asteroid-a-grey-big") {
		commands.spawn((
			Asteroid,
			Position { x: 300.0, y: 300.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("asteroid-a-grey-big") },
			Handle   { handle: None }
		));
	}

	if let Some(kind) = sprites.kinds.get("asteroid-a-brown-small") {
		commands.spawn((
			Asteroid,
			Position { x: 700.0, y: 300.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("asteroid-a-brown-small") },
			Handle   { handle: None }
		));
	}

	if let Some(kind) = sprites.kinds.get("asteroid-c-brown-big") {
		commands.spawn((
			Asteroid,
			Position { x: 300.0, y: 700.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("asteroid-c-brown-big") },
			Handle   { handle: None }
		));
	}
		
	if let Some(kind) = sprites.kinds.get("asteroid-b-brown-tiny") {
		commands.spawn((
			Asteroid,
			Position { x: 700.0, y: 700.0 },
			Rotation { angle: 0.0, rotation: UnitComplex::new(0.0) },
			Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size     { width: kind.width, height: kind.height },
			Sprite   { key: String::from("asteroid-b-brown-tiny") },
			Handle   { handle: None }
		));
	}
}
