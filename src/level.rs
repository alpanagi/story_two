use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_state::GameState;

#[derive(Resource)]
struct Map {
    image: Option<Handle<Image>>,
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map { image: None })
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Playing), spawn_map);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Map {
        image: Some(asset_server.load("map.png")),
    });
}

fn spawn_map(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut map: ResMut<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::hex("#ebede9").unwrap(),
        ..Default::default()
    });

    if map.image.is_some() {
        let image = images.get(map.image.clone().unwrap()).unwrap();
        for i in 0..image.width() {
            for j in 0..image.height() {
                let image = image.clone().try_into_dynamic().unwrap().to_rgba8();
                let pixel = image.get_pixel(i, j);
                if pixel.0[0] == 0 && pixel.0[1] == 0 && pixel.0[2] == 0 {
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Plane3d::default().mesh().size(0.95, 0.95)),
                            material: material.clone(),
                            transform: Transform::from_xyz(
                                0.5 + i as f32 - image.width() as f32 / 2.,
                                0.,
                                0.5 + j as f32 - image.height() as f32 / 2.,
                            ),
                            ..Default::default()
                        },
                        Collider::cuboid(0.95, 0.5, 0.95),
                    ));
                }
            }
        }

        map.image = None;
    }
}
