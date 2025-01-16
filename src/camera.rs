use super::*;

#[derive(Component)]
pub struct ControlledCameraIndentifier;
#[derive(Bundle)]
pub struct ControlledCamera{
    identifier: ControlledCameraIndentifier,
    camera: Camera,
    render_graph: Camera3d,
    transform: Transform,
}

impl ControlledCamera{
    pub fn new()->Self{
        #[cfg(debug_assertions)]
        println!("Making camera!");
        let identifier = ControlledCameraIndentifier;
        let camera = Camera::default();
        let render_graph = Camera3d::default();
        let mut transform = Transform{
            translation: Vec3::new(0.0, 0.0, 1000.0),
            ..default()
        };
        transform.look_at(Vec3::ZERO, Vec3::Y);
        Self{identifier, camera,render_graph,transform}
    }
}
