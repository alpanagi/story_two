use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::hex("#e8c170").unwrap(),
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(0.95, 0.95)),
            material,
            transform: Transform::from_xyz(0.5 - 15., 1., 0.5 + 15.),
            ..Default::default()
        },
        Player,
    ));
}

fn movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    rapier_context: Res<RapierContext>,
) {
    if keys.get_pressed().count() > 0 {
        next_state.set(GameState::Playing);
    } else {
        return;
    }

    let mut player_transform = player_query.single_mut();

    let mut offset = Vec3::ZERO;
    if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp) {
        if player_transform.translation.z > -15. {
            offset = Vec3::new(0., 0., -1.);
        }
    } else if keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown) {
        if player_transform.translation.z < 15. {
            offset = Vec3::new(0., 0., 1.);
        }
    } else if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        if player_transform.translation.x > -15. {
            offset = Vec3::new(-1., 0., 0.);
        }
    } else if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        if player_transform.translation.x < 15. {
            offset = Vec3::new(1., 0., 0.);
        }
    }

    if let None = rapier_context.cast_ray(
        player_transform.translation + offset + Vec3::new(0., 1., 0.),
        Vec3::new(0., -1., 0.),
        2.,
        true,
        QueryFilter::default(),
    ) {
        player_transform.translation += offset;
        if player_transform.translation.z < -15. {
            next_state.set(GameState::Won);
        }
    }
}
