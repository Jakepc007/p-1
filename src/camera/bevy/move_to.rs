use bevy::{
    prelude::{Camera, Query, Res, Transform, Vec2, With},
    time::Time,
};

use super::MainCamera;

const CAMERA_TRANSITION_DURATION: f64 = 0.5;

pub struct CameraTarget {
    pub position: Vec2,
    pub start_time: f64,
}

pub fn update_camera_position(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    camera_target: Res<CameraTarget>,
    time: Res<Time>,
) {
    // grab the 'main' camera
    let mut camera = camera.single_mut();

    let time_passed = time.seconds_since_startup() - camera_target.start_time;
    let target_time =
        (camera_target.start_time + CAMERA_TRANSITION_DURATION) - camera_target.start_time;

    camera.translation = camera.translation.lerp(
        camera_target.position.extend(camera.translation.z),
        if time_passed > target_time {
            1.0
        } else {
            time_passed / target_time
        } as f32,
    );
}
