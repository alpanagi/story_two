mod game_camera;
mod game_state;
mod instructions_screen;
mod level;
mod player;
mod success_screen;

use bevy::{asset::AssetMetaCheck, prelude::*};
use game_camera::GameCameraPlugin;
use instructions_screen::InstructionsScreenPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;
use success_screen::SuccessScreenPlugin;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            GameCameraPlugin,
            LevelPlugin,
            PlayerPlugin,
            InstructionsScreenPlugin,
            SuccessScreenPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();

    window.resolution.set(1024., 1024.);
}
