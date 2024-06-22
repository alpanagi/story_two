use bevy::prelude::*;

use crate::game_state::GameState;

#[derive(Component)]
struct InstructionsComponent;

#[derive(Component)]
struct PressAnyKeyText;

pub struct InstructionsScreenPlugin;
impl Plugin for InstructionsScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnExit(GameState::Instructions), despawn)
            .add_systems(Update, blink.run_if(in_state(GameState::Instructions)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(300.),
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            InstructionsComponent,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "MOVE WITH",
                TextStyle {
                    font: asset_server.load("PixelifySans-Medium.ttf"),
                    font_size: 32.0,
                    ..default()
                },
            ),));
        });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("instructions.png"),
            transform: Transform::from_scale(Vec3::new(4.0, 4.0, 4.0))
                .with_translation(Vec3::new(3.5, 0., 0.)),
            ..Default::default()
        },
        InstructionsComponent,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    bottom: Val::Px(300.),
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            InstructionsComponent,
            PressAnyKeyText,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "PRESS ANY KEY",
                TextStyle {
                    font: asset_server.load("PixelifySans-Medium.ttf"),
                    font_size: 32.0,
                    ..default()
                },
            ),));
        });
}

fn despawn(instructions_query: Query<Entity, With<InstructionsComponent>>, mut commands: Commands) {
    for entity in instructions_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn blink(
    mut text_query: Query<&mut Visibility, With<PressAnyKeyText>>,
    time: Res<Time>,
    mut elapsed_time: Local<f32>,
) {
    *elapsed_time += time.delta_seconds();

    let mut text_vis = text_query.single_mut();
    if *elapsed_time > 1.0 {
        if *text_vis == Visibility::Hidden {
            *text_vis = Visibility::Visible;
        } else if *text_vis == Visibility::Visible {
            *text_vis = Visibility::Hidden;
        }

        *elapsed_time = 0.;
    }
}
