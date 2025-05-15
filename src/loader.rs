use std::env::current_exe;
use std::collections::HashMap;
use flecs_ecs::prelude::*;
use macroquad::prelude::*;
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

	world.set(sprites);
    world.set(GameState::Playing);

	Ok(())
}

pub fn load_world(world: &mut World, id: &str) {
	world.get::<&SpriteKinds>(|sprites| {
		if let Some(kind) = sprites.kinds.get("player-ship-a-blue") {
			world.entity()
				.set(Position { x: 300.0, y: 300.0 })
				.set(Rotation { angle: 0.0 })
				.set(Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 })
				.set(Size     { width: kind.width, height: kind.height })
				.set(Sprite   { key: String::from("player-ship-a-blue") })
				.set(Weapon   { kind: WeaponKind::OneBullet })
				.set(Ship     { speed: 50, tracing: false, trace: Trace { key: String::from("thin"), tint: 0 }})
				.set(Actions  { actions: Action::Nothing })
				.set(Handle   { handle: None })
				.add::<Player>();
		}
		
		if let Some(kind) = sprites.kinds.get("asteroid-a-grey-big") {
			world.entity()
				.set(Position { x: 50.0, y: 50.0 })
				.set(Rotation { angle: 0.0 })
				.set(Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 })
				.set(Size     { width: kind.width, height: kind.height })
				.set(Sprite   { key: String::from("asteroid-a-grey-big") })
				.set(Handle   { handle: None })
				.add::<Asteroid>();
		}

		if let Some(kind) = sprites.kinds.get("asteroid-a-brown-small") {
			world.entity()
				.set(Position { x: 500.0, y: 50.0 })
				.set(Rotation { angle: 0.0 })
				.set(Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 })
				.set(Size     { width: kind.width, height: kind.height })
				.set(Sprite   { key: String::from("asteroid-a-brown-small") })
				.set(Handle   { handle: None })
				.add::<Asteroid>();
		}

		if let Some(kind) = sprites.kinds.get("asteroid-c-brown-big") {
			world.entity()
				.set(Position { x: 50.0, y: 500.0 })
				.set(Rotation { angle: 0.0 })
				.set(Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 })
				.set(Size     { width: kind.width, height: kind.height })
				.set(Sprite   { key: String::from("asteroid-c-brown-big") })
				.set(Handle   { handle: None })
				.add::<Asteroid>();
		}

		if let Some(kind) = sprites.kinds.get("asteroid-b-brown-tiny") {
			world.entity()
				.set(Position { x: 500.0, y: 500.0 })
				.set(Rotation { angle: 0.0 })
				.set(Center   { cx: kind.width / 2.0, cy: kind.height / 2.0 })
				.set(Size     { width: kind.width, height: kind.height })
				.set(Sprite   { key: String::from("asteroid-b-brown-tiny") })
				.set(Handle   { handle: None })
				.add::<Asteroid>();
		}
	});
}
