use avian3d::prelude::{Collider, RigidBody};

use super::*;
#[derive(Bundle, Debug)]
pub struct MyPlayerBundle {
    my_player: MyPlayer,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    collider: Collider,
    body: RigidBody
}
impl MyPlayerBundle {
    pub fn new(
        my_player: MyPlayer,
        mesh: Mesh3d,
        material: MeshMaterial3d<StandardMaterial>,
        transform: Transform,
        collider: Collider,
        body: RigidBody
    ) -> Self {
        Self {
            my_player,
            mesh,
            material,
            transform,
            collider,
            body
        }
    }
}

#[derive(Debug, Clone, Component, PartialEq, Eq, Copy)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>, Transform, IsTileSnappable, IsCollidingDraggable, GizmoOutlineToggle)]
pub struct MyPlayer {
    player_id: PlayerId,
    pos: GridPosition,
}
impl MyPlayer {
    pub fn new(player_id: PlayerId, pos: GridPosition) -> Self {
        Self { player_id, pos }
    }
}

fn spawn_player(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    player:MyPlayer,
    color: Color,
    ){
    let sphere_radius = TILE_WIDTH / 3f32;
    let body = RigidBody::Kinematic;
    commands.spawn(MyPlayerBundle::new(
        player,
        Mesh3d(meshes.add(Sphere::new(sphere_radius).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color:color,
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_translation(player.pos.into()),
        Collider::sphere(sphere_radius),
        body
    ))
    .observe(drag_with_collision)
    .observe(snap_drop_tile);
}

pub fn spawn_player_bundle(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    player: PlayerId
){
    match player{
        PlayerId::A => {
            let pos = GridPosition::new(2, 0);
            let player = MyPlayer::new(PlayerId::A, pos);
            let color = Color::srgb(0.0, 0.0, 1.0);
            spawn_player(commands, materials, meshes, player, color);
        },
        PlayerId::B => {
            let pos = GridPosition::new(2, (N_TILES - 1) as usize);
            let player = MyPlayer::new(PlayerId::B, pos);
            let color = Color::srgb(1.0, 0.0, 0.0);
            spawn_player(commands, materials, meshes, player, color);
        },
    }

}

