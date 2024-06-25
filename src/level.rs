use bevy::prelude::*;

#[derive(Resource)]
struct Map {
    image: Option<Handle<Image>>,
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map { image: None })
            .add_systems(Startup, setup)
            .add_systems(Update, spawn_map);
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
        base_color: Color::WHITE,
        ..Default::default()
    });

    if map.image.is_some() {
        let image = images.get(map.image.clone().unwrap()).unwrap();
        for i in 0..image.width() {
            for j in 0..image.height() {
                let image = image.clone().try_into_dynamic().unwrap().to_rgba8();
                let pixel = image.get_pixel(i, j);
                if pixel.0[0] == 0 && pixel.0[1] == 0 && pixel.0[2] == 0 {
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Plane3d::default().mesh().size(1., 1.)),
                        material: material.clone(),
                        transform: Transform::from_xyz(
                            0.5 + i as f32 - image.width() as f32 / 2.,
                            0.,
                            0.5 + j as f32 - image.height() as f32 / 2.,
                        ),
                        ..Default::default()
                    });
                }
            }
        }

        map.image = None;
    }
}
