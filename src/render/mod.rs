use bevy::prelude::*;

pub mod completed;
pub mod floor;
pub mod objects;
pub mod player;
pub mod puzzle;
pub mod score;
pub mod smooth;
pub mod walls;

pub fn setup_camera_and_light( mut commands: Commands,) {
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            range: 1060.0,
            radius: 30.0,
            intensity: 50_000_000.0,
            ..default()
        },
        Transform::from_xyz(-20.0, 20.0, 32.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.5, -43.5, 45.0).looking_at(Vec3::new(0.0, -7.8, 0.0), Vec3::Z),
    ));
}
