use amethyst::prelude::*;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::renderer::{Sprite, SpriteRender};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};


pub struct Frog {
	renderer = SpriteRender {
		pub sprite_sheet : SpriteSheetHandle = ,

	}

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

fn initialise_frog(world: &mut World) {
	world.create_entity()
		.with(Frog::new())
		.with(SpriteRender)
		.with(GlobalTransform::default())
		.with(Transform::default())
		.build();
}