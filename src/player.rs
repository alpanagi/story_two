use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::game_state::GameState;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Instructions)
            .add_systems(Startup, setup)
            .add_systems(Update, movement.run_if(in_state(GameState::Instructions)))
            .add_systems(Update, movement.run_if(in_state(GameState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(30., 30.))),
            material: materials.add(Color::hex("#e8c170").unwrap()),
            transform: Transform::from_xyz(480. - 16., -512. + 16., 0.),
            ..Default::default()
        },
        Player,
    ));
}

fn movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.get_pressed().count() > 0 {
        next_state.set(GameState::Playing);
    }

    let mut player_transform = player_query.single_mut();

    if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp) {
        if player_transform.translation.y < 480. {
            player_transform.translation += Vec3::new(0., 32., 0.);

            if player_transform.translation.y > 480. {
                next_state.set(GameState::Won);
            }
        }
    } else if keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown) {
        if player_transform.translation.y > -480. {
            player_transform.translation += Vec3::new(0., -32., 0.);
        }
    } else if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        if player_transform.translation.x > -480. {
            player_transform.translation += Vec3::new(-32., 0., 0.);
        }
    } else if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        if player_transform.translation.x < 480. {
            player_transform.translation += Vec3::new(32., 0., 0.);
        }
    }
}
