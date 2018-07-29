extern crate amethyst;
extern crate amethyst_extra;
#[macro_use]
extern crate log;
extern crate rand;

use amethyst::animation::*;
use amethyst::assets::*;
use amethyst::audio::*;
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::*;
use amethyst::ecs::*;
use amethyst::input::*;
use amethyst::prelude::*;
use amethyst::renderer::mouse::set_mouse_cursor_none;
use amethyst::renderer::*;
use amethyst::ui::*;
use amethyst::Result;
use amethyst_extra::*;
use amethyst::utils::scene::BasicScenePrefab;

use std::env;

mod data;
mod states;
mod systems;
mod utils;

pub use data::*;
pub use states::*;
pub use systems::*;
pub use utils::*;

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());
    // run_dir() -> String
    let bin_path = env::args().next().expect("Failed to get binary executable path");
    let last_slash_index = bin_path.rfind("/").expect("Failed to get last slash in binary path.");
    let mut base_path = bin_path[..last_slash_index].to_string();
    
    if base_path.contains("target/"){
        base_path = String::from(".");
    }
    
    let asset_loader = AssetLoader::new(
        &format!("{}/assets", base_path).to_string(),
        "base",
    );
    let display_config_path = asset_loader.resolve_path("config/display.ron").unwrap();
    let key_bindings_path = asset_loader.resolve_path("config/input.ron").unwrap();

    let game_data_builder = GameDataBuilder::default()
        .with(BulletMoverSystem, "bullet_mover", &[])
        .with(BulletEmitterSystem::default(), "bullet_emitter", &[])
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?
        )?
        .with(
            FollowMouseSystem::<String, String>::default(),
            "follow_mouse",
            &[],
        )
        .with_bundle(TransformBundle::new().with_dep(&[
            "bullet_mover",
            "bullet_emitter",
            "follow_mouse",
        ]))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(AnimationBundle::<u32, Material>::new(
            "animation_control_system",
            "sampler_interpolation_system",
        ))?
        .with_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))?
        .with(PrefabLoaderSystem::<BasicScenePrefab<Vec<PosTex>>>::default(), "", &[])
        .with(TimedDestroySystem, "timed_destroy", &[])
        .with(NormalOrthoCameraSystem::default(), "aspect_ratio", &[])
        .with(VisibilitySortingSystem::new(), "visibility", &["transform_system"])
        .with_basic_renderer(display_config_path, DrawFlat::<PosTex>::new().with_transparency(ColorMask::all(), ALPHA, None), true)?;
    let resources_directory = format!("");
    Application::build(resources_directory, TestState)?
        .with_resource(asset_loader)
        .build(game_data_builder)?
        .run();
    Ok(())
}
