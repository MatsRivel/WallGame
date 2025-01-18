use super::*;



#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyGizmos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum GridType {
    Tile,
    Circle,
    Horizontal,
    Vertical,
}
impl GridType {
    pub fn all() -> Vec<Self> {
        vec![Self::Tile, Self::Circle, Self::Horizontal, Self::Vertical]
    }
}
#[derive(Clone, Debug, Bundle)]
pub struct GizmoStructBundle {
    gs: GizmoStruct,
    transform: Transform,
    mesh_3d: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
}
impl GizmoStructBundle {
    pub fn new(
        gs: GizmoStruct,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> Self {
        let transform = Transform::from_translation(gs.vec());

        let material = MeshMaterial3d(materials.add(Color::BLACK.with_alpha(0.1)));
        let mesh = match gs.grid_type {
            GridType::Tile => {
                let shape = Cuboid::from_corners(
                    gs.vec() - Vec3::new(TILE_WIDTH / 2f32, TILE_WIDTH / 2f32, 0.0),
                    gs.vec() + Vec3::new(TILE_WIDTH / 2f32, TILE_WIDTH / 2f32, 1.0),
                );
                meshes.add(shape)
            }
            GridType::Circle => {
                let shape = Cylinder::new(10.0, 1.0);
                meshes.add(shape)
            }
            GridType::Horizontal => {
                let shape = Cuboid::from_corners(
                    gs.vec() - Vec3::new(TILE_WIDTH / 2f32, TRENCH_WIDTH / 2f32, 0.0),
                    gs.vec() + Vec3::new(TILE_WIDTH / 2f32, TRENCH_WIDTH / 2f32, 1.0),
                );
                meshes.add(shape)
            }
            GridType::Vertical => {
                let shape = Cuboid::from_corners(
                    gs.vec() - Vec3::new(TRENCH_WIDTH / 2f32, TILE_WIDTH / 2f32, 0.0),
                    gs.vec() + Vec3::new(TRENCH_WIDTH / 2f32, TILE_WIDTH / 2f32, 1.0),
                );
                meshes.add(shape)
            }
        };
        let mesh_3d = Mesh3d(mesh);
        Self {
            gs,
            transform,
            mesh_3d,
            material,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    A,
    B,
}

#[derive(Clone, Debug, Component)]
#[require(Mesh3d, Transform)]
pub struct GizmoStruct {
    xmod: f32,
    ymod: f32,
    pos: Pos,
    grid_type: GridType,
    is_visible: IsVisible,
    is_occupied: Option<Player>,
}

impl GizmoStruct {
    fn get_mod(grid_type: GridType) -> (f32, f32) {
        match grid_type {
            GridType::Tile => (-TILE_WIDTH / 2f32, -TILE_WIDTH / 2f32),
            GridType::Circle => (TRENCH_WIDTH / 2f32, TRENCH_WIDTH / 2f32),
            GridType::Horizontal => (-TILE_WIDTH / 2f32, TRENCH_WIDTH / 2f32),
            GridType::Vertical => (TRENCH_WIDTH / 2f32, -TILE_WIDTH / 2f32),
        }
    }
    pub fn new_float(x: f32, y: f32, grid_type: GridType) -> Self {
        let (xmod, ymod) = Self::get_mod(grid_type);
        let [x, y] = vec3_to_pos(Vec3::new(x, y, 0.0), xmod, ymod);
        Self::new_usize(x, y, grid_type)
    }
    pub fn new_usize(x: usize, y: usize, grid_type: GridType) -> Self {
        let (xmod, ymod) = Self::get_mod(grid_type);
        let pos = [x, y];
        let is_visible = match grid_type {
            GridType::Tile => IsVisible::Visible,
            _ => IsVisible::Invisible,
        };
        let is_occupied = None;
        Self {
            xmod,
            ymod,
            pos,
            grid_type,
            is_visible,
            is_occupied,
        }
    }
    pub fn xmod(&self) -> f32 {
        self.xmod
    }
    pub fn ymod(&self) -> f32 {
        self.ymod
    }
    pub fn vec(&self) -> Vec3 {
        pos_to_vec3(self.pos, self.xmod, self.ymod)
    }
    pub fn pos(&self) -> Pos {
        self.pos
    }
    pub fn grid_type(&self) -> GridType {
        self.grid_type
    }
    pub fn is_tile(&self) -> bool {
        self.grid_type == GridType::Tile
    }
    pub fn is_visible(&self) -> bool {
        self.is_visible == IsVisible::Visible
    }
    pub fn get_occupant(&self) -> Option<Player> {
        self.is_occupied
    }
    pub fn set_visible(&mut self) {
        self.is_visible = IsVisible::Visible;
    }
    pub fn set_invisible(&mut self) {
        self.is_visible = IsVisible::Invisible;
    }
    pub fn remove_player(&mut self) -> Result<(), ()> {
        if !self.is_tile() {
            return Ok(());
        }
        if self.is_occupied.is_some() {
            self.is_occupied = None;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn put_player(&mut self, player: Player) -> Result<(), ()> {
        if !self.is_tile() {
            return Ok(());
        }
        if self.is_occupied.is_none() {
            self.is_occupied = Some(player);
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn initials(grid_type: GridType) -> Vec<Self> {
        match grid_type {
            GridType::Tile => (0..N_TILES)
                .flat_map(|x| {
                    (0..N_TILES).map(move |y| Self::new_usize(x as usize, y as usize, grid_type))
                })
                .collect(),
            GridType::Circle => (0..N_TILES - 1)
                .flat_map(|x| {
                    (0..N_TILES - 1)
                        .map(move |y| Self::new_usize(x as usize, y as usize, grid_type))
                })
                .collect(),
            GridType::Horizontal => (0..N_TILES)
                .flat_map(|x| {
                    (0..N_TILES - 1)
                        .map(move |y| Self::new_usize(x as usize, y as usize, grid_type))
                })
                .collect(),
            GridType::Vertical => (0..N_TILES - 1)
                .flat_map(|x| {
                    (0..N_TILES).map(move |y| Self::new_usize(x as usize, y as usize, grid_type))
                })
                .collect(),
        }
    }

    pub fn draw_gizmo(&self, gizmos: &mut Gizmos) {
        let point = self.vec();
        let width = TILE_WIDTH - 4.0;
        let height = 1.0;
        match self.grid_type {
            GridType::Tile => {
                let size = Vec2::new(TILE_WIDTH, TILE_WIDTH);
                gizmos.rect(point, size, GREEN);
            }
            GridType::Circle => {
                gizmos.circle(point, 5.0, RED);
            }
            GridType::Horizontal => {
                let size = Vec2::new(width, height);
                gizmos.rect(point, size, RED);
            }
            GridType::Vertical => {
                let size = Vec2::new(height, width);
                gizmos.rect(point, size, RED);
            }
        };
    }
}
