use super::*;
#[derive(Bundle, Debug)]
pub struct TileBundle {
    transform: Transform,
    pos: GridPosition,
    wire_frame_gizmo: WireFrame,
    mesh_3d: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    grid_type: GridType,
    visibility: GizmoOutlineToggle,
    hoverable: IsHoverable
}
impl TileBundle {
    pub fn new(
        grid_position: GridPosition,
        grid_type: GridType,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Option<Self> {
        match grid_type {
            GridType::Tile => Some(Self::new_tile(
                grid_position,
                Vec2::splat(TILE_WIDTH),
                materials,
                meshes,
            )),
            GridType::Circle => None, // todo!(),
            GridType::Horizontal => Some(Self::new_horizontal(
                grid_position,
                Vec2::new(TILE_WIDTH - 1f32, TRENCH_WIDTH / 2f32 - 1f32),
                materials,
                meshes,
            )),
            GridType::Vertical => Some(Self::new_vertical(
                grid_position,
                Vec2::new(TRENCH_WIDTH / 2f32 - 1f32, TILE_WIDTH - 1f32),
                materials,
                meshes,
            )),
        }
    }
    
    fn new_square(
        position: Vec3,
        x: f32,
        y: f32,
        size: Vec2,
        frame_color: Color,
        tile_color: Color,
        alpha: f32,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        let transform = Transform::from_translation(position);
        let wire_frame_gizmo = WireFrame::new_square(size, frame_color);
        let shape = Cuboid::from_corners(
            position - Vec3::new(x, y, 0.0),
            position + Vec3::new(x, y, 0.0),
        );
        let mesh_3d = Mesh3d(meshes.add(shape));
        let material = MeshMaterial3d(materials.add(tile_color.with_alpha(alpha)));
        Self {
            transform,
            pos: position.into(),
            wire_frame_gizmo,
            mesh_3d,
            material,
            grid_type: GridType::Tile,
            visibility: GizmoOutlineToggle::Invisible,
            hoverable: IsHoverable,

        }
    }
    fn new_tile(
        grid_position: GridPosition,
        size: Vec2,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        let position = Vec3::from(grid_position);
        let x = size.x / 2f32;
        let y = size.y / 2f32;
        let frame_color = BLUE.into();
        let tile_color = Color::WHITE;
        let alpha = 0.5;

        Self::new_square(
            position,
            x,
            y,
            size,
            frame_color,
            tile_color,
            alpha,
            materials,
            meshes,
        )
    }

    fn new_horizontal(
        grid_position: GridPosition,
        size: Vec2,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> TileBundle {
        let position = Vec3::from(grid_position) + Vec3::ZERO.with_y(STEP_SIZE / 2f32);
        let x = TILE_WIDTH / 2f32;
        let y = TRENCH_WIDTH / 2f32;
        let frame_color = GREEN.into();
        let tile_color = Color::BLACK;
        let alpha = 0.25;

        Self::new_square(
            position,
            x,
            y,
            size,
            frame_color,
            tile_color,
            alpha,
            materials,
            meshes,
        )
    }

    fn new_vertical(
        grid_position: GridPosition,
        size: Vec2,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> TileBundle {
        let position = Vec3::from(grid_position) + Vec3::ZERO.with_x(STEP_SIZE / 2f32);
        let x = TRENCH_WIDTH / 2f32;
        let y = TILE_WIDTH / 2f32;
        let frame_color = GREEN.into();
        let tile_color = Color::BLACK;
        let alpha = 0.15;

        Self::new_square(
            position,
            x,
            y,
            size,
            frame_color,
            tile_color,
            alpha,
            materials,
            meshes,
        )
    }
}
