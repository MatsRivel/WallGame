use avian3d::prelude::{Collider, RigidBody};

use super::*;
#[derive(Debug, Bundle)]
pub struct WallBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    wall: Wall,
    hoverable: IsHoverable,
    frame: WireFrame,
    collider: Collider,
    body: RigidBody
}
impl WallBundle {
    pub fn new(
        mesh: Mesh3d,
        material: MeshMaterial3d<StandardMaterial>,
        transform: Transform,
        wall: Wall,
    ) -> Self {
        let frame = WireFrame::new_square(Vec2::new(wall.length, wall.width), WHITE.into());
        let collider = Collider::cuboid(wall.length, wall.width, WALL_HEIGHT);
        let body = RigidBody::Kinematic;
        Self {
            mesh,
            material,
            transform,
            wall,
            hoverable: IsHoverable,
            frame,
            collider,
            body
        }
    }
}

#[derive(Debug, Component)]
#[require(GizmoOutlineToggle, IsHoverable, Transform, IsWall, IsDraggable, IsWallSnappable, Mesh3d, MeshMaterial3d<StandardMaterial> )]
pub struct Wall {
    length: f32,
    width: f32,
}
impl Wall {
    pub fn new(length: f32, width: f32) -> Self {
        Self { length, width }
    }
}
impl Default for Wall {
    fn default() -> Self {
        Self {
            length: TILE_WIDTH * 2.0,
            width: TRENCH_WIDTH,
        }
    }
}
#[derive(Debug, Component, Default)]
pub struct IsWall;

pub fn spawn_wall(
    start_pos: Vec3,
    n_walls: usize,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let half_length = TILE_WIDTH;
    let half_width = TRENCH_WIDTH / 2f32;
    let range_width = 0..1;
    let range_height = 0..n_walls;
    for (x, y) in range_width.flat_map(|x| range_height.clone().map(move |y| (x, y))) {
        let pos_modifier: Vec3 = Vec3::new(
            (x) as f32 * (TILE_WIDTH + 1.0),
            (y) as f32 * (TRENCH_WIDTH + 1.0),
            0.0,
        );
        let pos = start_pos + pos_modifier;
        let shape = Cuboid::from_corners(
            pos - Vec3::new(half_length, half_width, 0.0),
            pos + Vec3::new(half_length + TRENCH_WIDTH, half_width, 1.0) + Vec3::Z * WALL_HEIGHT,
        );

        let mesh = Mesh3d(meshes.add(shape));
        let material = MeshMaterial3d(materials.add(Color::srgba(0.824, 0.412, 0.118, 1.0)));
        let transform = Transform::from_translation(pos);
        let wall = Wall::new(2.0 * half_length, 2.0 * half_width);
        let bundle = WallBundle::new(mesh, material, transform, wall);
        commands
            .spawn(bundle)
            .observe(tag_visible_on_hover)
            .observe(tag_invisible_on_hover_end)
            .observe(drag)
            .observe(snap_drop_wall)
            .observe(rotate_hovered_wall)
            ;
    }
}
