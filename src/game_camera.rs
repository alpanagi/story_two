use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*, render::camera::ScalingMode};

pub struct GameCameraPlugin;
impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 10., 0.)
            .with_rotation(Quat::from_rotation_x(-90_f32.to_radians())),
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::hex("#172038").unwrap()),
            ..Default::default()
        },
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 32.,
                height: 32.,
            },
            ..Default::default()
        }),
        tonemapping: Tonemapping::None,
        ..Default::default()
    });
}
