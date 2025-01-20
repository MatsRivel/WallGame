use bevy::{input::mouse::MouseWheel, pbr::VolumetricFog};

use super::*;

#[derive(Component,Default,Clone,Copy)]
pub struct ZoomCameraIdentifier;

#[derive(Bundle)]
pub struct ZoomCamera {
    identifier: ZoomCameraIdentifier,
    camera: Camera,
    render_graph: Camera3d,
    transform: Transform,
}
impl ZoomCamera {
    pub fn new(position: Vec3)  -> Self {
        #[cfg(debug_assertions)]
        println!("Zoom Camera!");
        let identifier = ZoomCameraIdentifier;
        let camera = Camera::default();
        let render_graph = Camera3d::default();
        let mut transform = Transform {
            translation: position,
            ..default()
        };
        transform.look_at(Vec3::ZERO, Vec3::Y);
        Self {
            identifier,
            camera,
            render_graph,
            transform,
        }
    }
}

#[derive(Component)]
pub struct ControlledCameraIndentifier;
#[derive(Bundle)]
pub struct ControlledCamera {
    identifier: ControlledCameraIndentifier,
    camera: Camera,
    render_graph: Camera3d,
    transform: Transform,
}

impl ControlledCamera {
    pub fn new(position: Vec3) -> Self {
        #[cfg(debug_assertions)]
        println!("Controlled camera!");
        let identifier = ControlledCameraIndentifier;
        let camera = Camera::default();
        let render_graph = Camera3d::default();
        let mut transform = Transform {
            translation: position,
            ..default()
        };
        transform.look_at(Vec3::ZERO, Vec3::Y);
        Self {
            identifier,
            camera,
            render_graph,
            transform,
        }
    }
}

pub fn move_camera(
    mut events: EventReader<KeyboardInput>,
    time: Res<Time>,
    mut controlled_camera_query: Query<&mut Transform, With<ControlledCameraIndentifier>>,
) {
    for event in events.read() {
        // Only check for characters when the key is pressed.
        if !event.state.is_pressed() {
            continue;
        }
        let move_dir = MoveDirections::new_event(event).to_vec3();
        let mut cam = controlled_camera_query.single_mut();
        cam.translation += move_dir * 25.0;
        *cam = cam.looking_at(Vec3::splat(0.0), Vec3::Y);
    }
}

pub fn spawn_camera(commands: &mut Commands, pos: GridPosition){
    let position = pos.into();
    commands.spawn(ZoomCamera::new(position));
        // .insert(VolumetricFog::default());
}

pub const ZOOM_WHEEL_SPEED_MULTIPLIER: i32 = 2;
pub fn zoom_camera(mut mouse_wheel_event: EventReader<MouseWheel>, mut query: Query<(Entity, &mut Transform),With<ZoomCameraIdentifier>>){
    for mouse_wheel in mouse_wheel_event.read(){
        let (_camera_entity, mut zoom) = query.single_mut();
        let z_mod = mouse_wheel.y * (ZOOM_WHEEL_SPEED_MULTIPLIER as f32);
        zoom.translation += Vec3::ZERO.with_z(z_mod)

    }
}