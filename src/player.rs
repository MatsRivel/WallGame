use super::*;
#[derive(Bundle, Debug)]
pub struct MyPlayerBundle {
    my_player: MyPlayer,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}
impl MyPlayerBundle {
    pub fn new(
        my_player: MyPlayer,
        mesh: Mesh3d,
        material: MeshMaterial3d<StandardMaterial>,
        transform: Transform,
    ) -> Self {
        Self {
            my_player,
            mesh,
            material,
            transform,
        }
    }
}

#[derive(Debug, Clone, Component, PartialEq, Eq)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>, Transform, IsTileSnappable, IsDraggable, GizmoOutlineToggle)]
pub struct MyPlayer {
    player_id: PlayerId,
    pos: GridPosition,
}
impl MyPlayer {
    pub fn new(player_id: PlayerId, pos: GridPosition) -> Self {
        Self { player_id, pos }
    }
}
