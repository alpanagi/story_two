use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if map.image.is_some() {
        let image = images.get(map.image.clone().unwrap()).unwrap();
        for i in 0..image.width() {
            for j in 0..image.height() {
                let image = image.clone().try_into_dynamic().unwrap().to_rgba8();
                let pixel = image.get_pixel(i, j);
                if pixel.0[0] == 0 && pixel.0[1] == 0 && pixel.0[2] == 0 {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(30., 30.))),
                        material: materials.add(Color::hex("#ffffff").unwrap()),
                        transform: Transform::from_xyz(
                            16. + i as f32 * 32. - 32. * image.width() as f32 / 2.,
                            16. + j as f32 * 32. - 32. * image.height() as f32 / 2.,
                            0.,
                        ),
                        ..Default::default()
                    });
                }
            }
        }

        map.image = None;
    }
}
