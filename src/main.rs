extern crate amethyst;
use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::input::{is_close_requested, is_key_down}; 
use amethyst::renderer::{Texture, Camera, PngFormat, DisplayConfig, DrawFlat, Event, Pipeline, PosTex,
                         RenderBundle, Stage, VirtualKeyCode, SpriteSheetHandle, MaterialTextureSet, SpriteSheet};

mod frog;


struct GameState;

impl<'a, 'b> State<GameData<'a, 'b>> for GameState {
    
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        
        let sprite_sheet_handle = load_sprite_sheet(world);

        println!("\n\nokherewego");
        frog::initialise_frog(world, sprite_sheet_handle);
        
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }


}

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());
    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    
    let dir = env!("CARGO_MANIFEST_DIR");
    let config = DisplayConfig::load(&path);

    /*
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );
    */
    let game_data = GameDataBuilder::default()
        .with_basic_renderer(path, DrawFlat::<PosTex>::new(), true)?;
    let mut game = Application::build(dir, GameState)?
        .build(game_data)?;
    game.run();
    Ok(())
}


fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {

    // Reference to texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            PngFormat,
            Default::default(),
            (),
            &texture_storage,
        )
    };

    let texture_id = 0;
    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    material_texture_set.insert(texture_id, texture_handle);

    let sprite_sheet = SpriteSheet{
        texture_id,
        sprites: vec![frog::SPRITE]
    };

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
    };

    sprite_sheet_handle
}