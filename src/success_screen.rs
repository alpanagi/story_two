use bevy::prelude::*;

use crate::game_state::GameState;

pub struct SuccessScreenPlugin;
impl Plugin for SuccessScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Won), show_screen);
    }
}

fn show_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "SUCCESS!",
                TextStyle {
                    font: asset_server.load("PixelifySans-Medium.ttf"),
                    font_size: 32.0,
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                "SUCCESS!",
                TextStyle {
                    font: asset_server.load("PixelifySans-Medium.ttf"),
                    font_size: 32.0,
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                "SUCCESS!",
                TextStyle {
                    font: asset_server.load("PixelifySans-Medium.ttf"),
                    font_size: 32.0,
                    ..default()
                },
            ));
        });
}
