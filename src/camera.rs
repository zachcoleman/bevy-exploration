use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;

/// camera plugin (used to navigate a map)
pub struct CameraPlayerPlugin;
impl Plugin for CameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraLookAt>()
            .insert_resource(CameraSettings {
                speed: 20.0,
                y_bounds: (3.0, 30.0),
                starting_pos: Vec3::new(5., 5., 5.),
            })
            .add_startup_system(spawn_camera)
            .add_system(key_moves)
            .add_system(scroll_wheel_zoom);
    }
}

/// camera settings
#[derive(Resource)]
pub struct CameraSettings {
    speed: f32,
    y_bounds: (f32, f32),
    starting_pos: Vec3,
}
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            speed: 10.0,
            y_bounds: (3.0, 30.0),
            starting_pos: Vec3::new(5., 5., 5.),
        }
    }
}

/// component to track where the camera is looking
#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct CameraLookAt {
    pub target: Vec3,
}

/// marker component for the camera
#[derive(Component)]
pub struct CameraPlayer;

/// spawn camera by default starts w/ set distance and looking at 45 degrees to ground
pub fn spawn_camera(settings: Res<CameraSettings>, mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(settings.starting_pos)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        CameraPlayer,
        CameraLookAt::default(),
        PickingCameraBundle::default(),
    ));
}

/// move the camera based on input (WASD+QE+space+lshift+scroll)
fn key_moves(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    settings: Res<CameraSettings>,
    mut query_transform: Query<(&mut Transform, &mut CameraLookAt), With<CameraPlayer>>,
) {
    let (mut transform, mut look_at) = query_transform.single_mut();
    let mut cam_delta = Vec3::ZERO;
    let mut look_delta = Vec3::ZERO;
    let local_z = transform.local_z();
    let forward = -Vec3::new(local_z.x, 0., local_z.z);
    let right = Vec3::new(local_z.z, 0., -local_z.x);
    let vertical_ratio = (settings.starting_pos.x.powi(2) + settings.starting_pos.z.powi(2)).sqrt()
        / settings.starting_pos.y.abs();
    for key in keys.get_pressed() {
        match key {
            KeyCode::W => {
                cam_delta = forward * settings.speed * time.delta_seconds();
                look_delta = cam_delta;
            }
            KeyCode::S => {
                cam_delta = -forward * settings.speed * time.delta_seconds();
                look_delta = cam_delta;
            }
            KeyCode::A => {
                cam_delta = -right * settings.speed * time.delta_seconds();
                look_delta = cam_delta;
            }
            KeyCode::D => {
                cam_delta = right * settings.speed * time.delta_seconds();
                look_delta = cam_delta;
            }
            KeyCode::Space => {
                cam_delta = Vec3::Y * settings.speed * time.delta_seconds();
                look_delta =
                    forward.normalize() * settings.speed * time.delta_seconds() * vertical_ratio;
            }
            KeyCode::LShift => {
                cam_delta = -Vec3::Y * settings.speed * time.delta_seconds();
                look_delta =
                    -forward.normalize() * settings.speed * time.delta_seconds() * vertical_ratio;
            }
            KeyCode::Q => {
                transform.rotate_around(
                    look_at.target,
                    Quat::from_rotation_y(-0.1 * settings.speed * time.delta_seconds()),
                );
            }
            KeyCode::E => {
                transform.rotate_around(
                    look_at.target,
                    Quat::from_rotation_y(0.1 * settings.speed * time.delta_seconds()),
                );
            }
            _ => {}
        }
        if (transform.translation + cam_delta).y < settings.y_bounds.0
            || settings.y_bounds.1 < (transform.translation + cam_delta).y
        {
            cam_delta = Vec3::ZERO;
            look_delta = Vec3::ZERO;
        }
        transform.translation += cam_delta;
        look_at.target += look_delta;
    }
}

fn scroll_wheel_zoom(
    mut scroll: EventReader<MouseWheel>,
    time: Res<Time>,
    settings: Res<CameraSettings>,
    mut query_transform: Query<&mut Transform, With<CameraPlayer>>,
) {
    let mut cam_delta = Vec3::ZERO;
    let mut transform = query_transform.single_mut();
    for ev in scroll.iter() {
        match ev.unit {
            bevy::input::mouse::MouseScrollUnit::Line => {
                cam_delta = transform.local_z() * settings.speed * 5. * time.delta_seconds() * ev.y;
            }
            bevy::input::mouse::MouseScrollUnit::Pixel => {
                cam_delta = transform.local_z() * settings.speed * time.delta_seconds() * ev.y;
            }
        }
    }
    if (transform.translation + cam_delta).y > settings.y_bounds.0
        && settings.y_bounds.1 > (transform.translation + cam_delta).y
    {
        transform.translation += cam_delta;
    }
}
