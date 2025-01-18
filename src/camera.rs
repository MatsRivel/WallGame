use super::*;

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
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        println!("Making camera!");
        let identifier = ControlledCameraIndentifier;
        let camera = Camera::default();
        let render_graph = Camera3d::default();
        let mut transform = Transform {
            translation: Vec3::new(0.0, 0.0, 1000.0),
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
        println!("{event:?}");
        let move_dir = MoveDirections::new_event(event).to_vec3();
        let mut cam = controlled_camera_query.single_mut();
        cam.translation += move_dir * 25.0;
        *cam = cam.looking_at(Vec3::splat(0.0), Vec3::Y);
    }
}
