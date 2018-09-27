extern crate amethyst;
use amethyst::prelude::*;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::input::{is_close_requested, is_key_down}; 
use amethyst::renderer::{Texture, Camera, PngFormat, DisplayConfig, DrawFlat, Event, Pipeline, PosNormTex,
                         RenderBundle, Stage, VirtualKeyCode};

mod frog;


struct GameState;

impl<'a, 'b> State<GameData<'a, 'b>> for GameState {
    
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, ..} = data;
        
        let spritesheet = {
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
        println!("\n\nokherewego");
        frog::initialise_frog(world, spritesheet);
        
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
    
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::build("./", GameState)?
        .build(game_data)?;
    game.run();
    Ok(())
}
