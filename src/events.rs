use bevy::{input::mouse::MouseMotion, prelude::*};

const FIXED_DELTA_TIME: f32 = 1.0 / 144.0;
const SPEED: f32 = 100.0;

#[derive(Component)]
pub struct GameCamera {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        Self { yaw: 0.0, pitch: 0.0 }
    }
}

pub fn handle_keyboard(keys: Res<Input<KeyCode>>, mut camera: Query<&mut Transform, With<Camera>>, mut window: ResMut<Windows>) {
    if keys.pressed(KeyCode::Z) {
        for mut transform in camera.iter_mut() {
            let forward = transform.forward();
            transform.translation += forward * FIXED_DELTA_TIME * SPEED;
        }
    }
    if keys.pressed(KeyCode::S) {
        for mut transform in camera.iter_mut() {
            let forward = transform.forward();
            transform.translation -= forward * FIXED_DELTA_TIME * SPEED;
        }
    }
    if keys.pressed(KeyCode::Q) {
        for mut transform in camera.iter_mut() {
            let right = transform.right();
            transform.translation -= right * FIXED_DELTA_TIME * SPEED;
        }
    }
    if keys.pressed(KeyCode::D) {
        for mut transform in camera.iter_mut() {
            let right = transform.right();
            transform.translation += right * FIXED_DELTA_TIME * SPEED;
        }
    }
    if keys.pressed(KeyCode::Space) {
        for mut transform in camera.iter_mut() {
            transform.translation.y += FIXED_DELTA_TIME * SPEED;
        }
    }
    if keys.pressed(KeyCode::LShift) {
        for mut transform in camera.iter_mut() {
            transform.translation.y -= FIXED_DELTA_TIME * SPEED;
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        for window in window.iter_mut() {
            window.set_cursor_lock_mode(!window.cursor_locked());
            window.set_cursor_visibility(!window.cursor_visible());
        }
    }
}

pub fn handle_mouse_motion(mut mouse_motion_events: EventReader<MouseMotion>, mut camera: Query<(&mut Transform, &mut GameCamera)>) {
    for event in mouse_motion_events.iter() {
        for (mut transform, mut game_camera) in camera.iter_mut() {
            game_camera.yaw += event.delta.x * 0.1;
            game_camera.pitch -= event.delta.y * 0.1;

            game_camera.pitch = game_camera.pitch.clamp(-89.0, 89.0);

            let yaw_radians = game_camera.yaw.to_radians();
            let pitch_radians = game_camera.pitch.to_radians();

            let direction = Vec3::new(yaw_radians.cos() * pitch_radians.cos(), pitch_radians.sin(), yaw_radians.sin() * pitch_radians.cos());

            let translation = transform.translation;
            transform.look_at(translation + direction, Vec3::Y);
        }
    }
}
