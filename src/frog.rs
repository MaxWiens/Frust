use amethyst::prelude::*;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::renderer::{Sprite, SpriteRender, SpriteSheetSet, SpriteSheetHandle};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};


pub struct Frog {

}

impl Frog {
	fn new() -> Frog {
		Frog {

		}
	}
}

impl Component for Frog {
	type Storage = DenseVecStorage<Self>;
}
// Sprite().From( (16,16), [16,32,16,0])

fn initialise_frog(world: &mut World, spritesheet : TextureHandle ) {
	let set = SpriteSheetSet::new();
	set.handle(1);


	let renderer = SpriteRender {
		sprite_sheet = SpriteSheetHandl
	}

	world.create_entity()
		.with(Frog::new())
		.with()
		.with(GlobalTransform::default())
		.with(Transform::default())
		.build();
}