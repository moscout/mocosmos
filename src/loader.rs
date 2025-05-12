use std::env::current_exe;
use std::collections::HashMap;
use bevy_ecs::prelude::*;
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

	world.insert_resource(sprites);
    world.insert_resource(GameState::Playing);

	Ok(())
}

pub fn load_world(world: &mut World, id: &str) {
	let mut kinds = HashMap::new();

	{
		let sprites = world.get_resource::<SpriteKinds>().unwrap();

		if let Some(kind) = sprites.kinds.get("player-ship-a-blue") {
			kinds.insert(String::from("player-ship-a-blue"), Size { width: kind.width, height: kind.height });
		}
		if let Some(kind) = sprites.kinds.get("asteroid-a-grey-big") {
			kinds.insert(String::from("asteroid-a-grey-big"), Size { width: kind.width, height: kind.height });
		}
		if let Some(kind) = sprites.kinds.get("asteroid-a-brown-small") {
			kinds.insert(String::from("asteroid-a-brown-small"), Size { width: kind.width, height: kind.height });
		}
		if let Some(kind) = sprites.kinds.get("asteroid-c-brown-big") {
			kinds.insert(String::from("asteroid-c-brown-big"), Size { width: kind.width, height: kind.height });
		}
		if let Some(kind) = sprites.kinds.get("asteroid-b-brown-tiny") {
			kinds.insert(String::from("asteroid-b-brown-tiny"), Size { width: kind.width, height: kind.height });
		}
	}

	if let Some(kind) = kinds.get("player-ship-a-blue") {
		world.spawn((
			Player,
			Position { x: 300.0, y: 300.0 },
			Rotation { angle: 0.0 },
			Center { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size { width: kind.width, height: kind.height },
			Sprite { key: String::from("player-ship-a-blue") },
			Weapon { kind: WeaponKind::OneBullet }
		));
	}

	if let Some(kind) = kinds.get("asteroid-a-grey-big") {
		world.spawn((
			Asteroid,
			Position { x: 50.0, y: 50.0 },
			Rotation { angle: 0.0 },
			Center { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size { width: kind.width, height: kind.height },
			Sprite { key: String::from("asteroid-a-grey-big") },
		));
	}
	
	if let Some(kind) = kinds.get("asteroid-a-brown-small") {
		world.spawn((
			Asteroid,
			Position { x: 500.0, y: 50.0 },
			Rotation { angle: 0.0 },
			Center { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size { width: kind.width, height: kind.height },
			Sprite { key: String::from("asteroid-a-brown-small") },
		));
	}
	
	if let Some(kind) = kinds.get("asteroid-c-brown-big") {
		world.spawn((
			Asteroid,
			Position { x: 50.0, y: 500.0 },
			Rotation { angle: 0.0 },
			Center { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size { width: kind.width, height: kind.height },
			Sprite { key: String::from("asteroid-c-brown-big") },
		));
	}
	
	if let Some(kind) = kinds.get("asteroid-b-brown-tiny") {
		world.spawn((
			Asteroid,
			Position { x: 500.0, y: 500.0 },
			Rotation { angle: 0.0 },
			Center { cx: kind.width / 2.0, cy: kind.height / 2.0 },
			Size { width: kind.width, height: kind.height },
			Sprite { key: String::from("asteroid-b-brown-tiny") },
		));
	}
}
