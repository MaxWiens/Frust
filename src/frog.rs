use amethyst::prelude::*;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::renderer::{Sprite, SpriteSheetHandle, WithSpriteRender, TextureHandle};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};


pub struct Frog {

}

const SPRITE_SIZE: (f32, f32) = (16.0, 16.0);
const SPRITESHEET_SIZE: (f32, f32) = (128.0, 16.0);

pub const SPRITE : Sprite = Sprite {
		left: 	0.0,
		right: 	16.0,
		top: 	0.0,
		bottom:	16.0
};

impl Frog {
	fn new() -> Frog {
		Frog {

		}
	}
}


/*
let spritesheet : SpriteHandle = {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        "texture/spritesheet.png",
        PngFormat,
        Default::default(),
        (),
        &texture_storage
    )
};
*/

impl Component for Frog {
	type Storage = DenseVecStorage<Self>;
}
// Sprite().From( (16,16), [16,32,16,0])

pub fn initialise_frog(world: &mut World, spritesheet : SpriteSheetHandle ) {

	

	world.create_entity()
		.with(Frog::new())
		//.with_sprite(&sprite, spritesheet, SPRITESHEET_SIZE).expect("Error creating SpriteRender for frog")
		.with(GlobalTransform::default())
		.with(Transform::default())
		.build();
	println!("\n\nnothek");
}

