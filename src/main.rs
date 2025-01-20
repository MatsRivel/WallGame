mod camera;
mod grid;
mod move_directions;
mod player;
mod pos;
mod position_conversion;
mod tiles;
mod visibility_toggle;
mod walls;
mod wireframe;

pub use bevy::color::palettes::css::*;
// use bevy::gizmos::grid;
use bevy::{input::keyboard::KeyboardInput, picking, ui::picking_backend};
pub use bevy::input::mouse::MouseMotion;
use bevy::pbr::CascadeShadowConfigBuilder;
pub use bevy::prelude::*;
use camera::{ControlledCamera, move_camera};
use grid::{GridType, PlayerId};
use move_directions::MoveDirections;
use player::{MyPlayer, MyPlayerBundle};
use pos::GridPosition;
use tiles::TileBundle;
use visibility_toggle::{GizmoOutlineToggle, tag_invisible_on_hover_end, tag_visible_on_hover};
use walls::{IsWall, spawn_wall};

use std::f32::consts::PI;
use wireframe::WireFrame;
const TILE_WIDTH: f32 = 64.0;
const TRENCH_WIDTH: f32 = 8.0;
const N_TILES: i32 = 5;
const STEP_SIZE: f32 = TILE_WIDTH + TRENCH_WIDTH;
pub const SKY_COLOR: Color = Color::linear_rgb(0.5, 0.5, 0.1);

fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vk");
    }
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(ClearColor(SKY_COLOR))
        .init_gizmo_group::<MyGizmos>()
        // .add_systems(Startup, simple_setup)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_camera)
        .add_systems(Update, rotate_light)
        .add_systems(Update, draw_toggelable_visible_wireframes)
        .add_systems(Update, draw_always_visible_wireframes)
        .run();
}
pub fn simple_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(ControlledCamera::new());
    // Directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 5.0, 5.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
    let start_pos = Vec3::ZERO;
    let n_walls = 4;
    spawn_wall(
        start_pos,
        n_walls,
        &mut commands,
        &mut materials,
        &mut meshes,
    );
}
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(ControlledCamera::new());
    // Point-light
    commands.spawn((
        PointLight {
            color: Color::Srgba(Srgba::new(1.0, 0.0, 0.0, 0.25)),
            radius: 0.25,
            shadows_enabled: true,
            // intensity: 10f32.powf(1_000_000.0),
            range: STEP_SIZE,
            ..default()
        },
        Transform::from_xyz(5.0, 5.0, 4.5),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 10.0,
            maximum_distance: 20.0,
            ..default()
        }
        .build(),
    ));

    // Directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 5.0, 5.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));

    let plane_dims = (N_TILES as f32) * STEP_SIZE / 2f32;
    // Ground Plane
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -1.0),
        Mesh3d(meshes.add(Cuboid::from_corners(
            Vec3::new(-plane_dims, -plane_dims, -0.2),
            Vec3::new(plane_dims, plane_dims, -0.1),
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
    ));

    spawn_player_bundle_a(&mut commands, &mut materials, &mut meshes)
        .observe(drag_with_collision)
        .observe(snap_drop_tile);

    spawn_player_bundle_b(&mut commands, &mut materials, &mut meshes)
        .observe(drag_with_collision)
        .observe(snap_drop_tile);

    spawn_grid(&mut commands, &mut materials, &mut meshes);
    let start_pos = Vec3::new(STEP_SIZE * (N_TILES - 2) as f32, 0.0, 0.0);
    let n_walls = 20;
    spawn_wall(
        start_pos,
        n_walls,
        &mut commands,
        &mut materials,
        &mut meshes,
    );
}

fn spawn_grid<'a>(
    commands: &'a mut Commands,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
    meshes: &'a mut ResMut<Assets<Mesh>>,
) {
    for (x, y) in (0..N_TILES as usize).flat_map(|x| (0..N_TILES as usize).map(move |y| (x, y))) {
        let grid_position = GridPosition::new(x, y);
        commands
            .spawn(TileBundle::new(grid_position, GridType::Tile, materials, meshes).unwrap())
            .observe(tag_invisible_on_hover_end)
            .observe(tag_visible_on_hover);
        if x as i32 != N_TILES - 1 {
            commands
                .spawn(
                    TileBundle::new(grid_position, GridType::Vertical, materials, meshes).unwrap(),
                )
                .observe(tag_invisible_on_hover_end)
                .observe(tag_visible_on_hover);
        }
        if y as i32 != N_TILES - 1 {
            commands
                .spawn(
                    TileBundle::new(grid_position, GridType::Horizontal, materials, meshes)
                        .unwrap(),
                )
                .observe(tag_invisible_on_hover_end)
                .observe(tag_visible_on_hover);
        }
    }
}

fn spawn_player_bundle_a<'a>(
    commands: &'a mut Commands,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
    meshes: &'a mut ResMut<Assets<Mesh>>,
) -> EntityCommands<'a> {
    let player_a = GridPosition::new(2, 0);
    commands.spawn(MyPlayerBundle::new(
        MyPlayer::new(PlayerId::A, player_a),
        Mesh3d(meshes.add(Sphere::new(TILE_WIDTH / 3f32).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 4.0),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_translation(player_a.into()),
    ))
}

fn spawn_player_bundle_b<'a>(
    commands: &'a mut Commands,
    materials: &'a mut ResMut<Assets<StandardMaterial>>,
    meshes: &'a mut ResMut<Assets<Mesh>>,
) -> EntityCommands<'a> {
    let player_b = GridPosition::new(2, (N_TILES - 1) as usize);
    commands.spawn(MyPlayerBundle::new(
        MyPlayer::new(PlayerId::B, player_b),
        Mesh3d(meshes.add(Sphere::new(TILE_WIDTH / 3f32).mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_translation(player_b.into()),
    ))
}

fn rotate_light(
    time: Res<Time>,
    mut light_query: Query<&mut Transform, With<PointLight>>,
    mut gizmos: Gizmos,
) {
    for mut light in light_query.iter_mut() {
        let new_rot = Quat::from_rotation_z(((time.delta_secs()) / 25.0) * (360.0 / (2.0 * PI)));
        let new_rot = new_rot.normalize();
        light.rotate_around(Vec3::ZERO, new_rot);
        gizmos.circle(Isometry3d::from_translation(light.translation), 1.0, WHITE);
    }
}
/// When an object is "Dragged" (prolonged click), the object follows the mouse.
fn drag(
    hit: Trigger<Pointer<Drag>>,
    mut target_query: Query<(Entity, &mut Transform), With<IsDraggable>>,
) {
    let target_id = hit.target;
    for (entity, mut target) in target_query.iter_mut() {
        if target_id != entity {
            continue;
        }
        let pointer_location = hit.delta;
        target.translation += Vec3::new(pointer_location.x, -pointer_location.y, 0.0);
    }
}
//TODO: Correct collisions.
fn drag_with_collision(
    hit: Trigger<Pointer<Drag>>,
    mut target_query: Query<(Entity, &mut Transform), With<IsCollidingDraggable>>,
) {
    let target_id = hit.target;
    for (entity, mut target) in target_query.iter_mut() {
        if target_id != entity {
            continue;
        }
        
        let distance = &hit.event().distance;
        let dir = match Dir3::from_xyz(target.translation.x + distance.x, target.translation.y + distance.y, 0.0){
            Ok(v) => v,
            Err(_e) => return,
        };
        // let ray = Ray3d::new(target.translation,dir) ;
        let pointer_location = hit.delta;
        target.translation += Vec3::new(pointer_location.x, -pointer_location.y, 0.0);
    }
}
fn rotate_dragged_wall(
    hit: Trigger<Pointer<Drag>>,
    keypress: Res<ButtonInput<KeyCode>>,
    mut target_query: Query<
        (Entity, &mut Transform, &mut WireFrame),
        (With<IsDraggable>, With<IsWall>)>
) {
    if keypress.just_released(KeyCode::KeyR) {
        let target_id = hit.target;
        for (entity, mut target, mut wireframe) in target_query.iter_mut() {
            if target_id != entity {
                continue;
            }
            target.rotate(Quat::from_axis_angle(Vec3::Z, PI / 2f32));
            wireframe.rotate();
        }
    }
}
/// Snaps to an integer position in a grid defined mathematically.
fn snap_drop_tile(
    hit: Trigger<Pointer<DragEnd>>,
    mut player_query: Query<(Entity, &mut Transform), With<IsTileSnappable>>,
) {
    let target_id = hit.target;
    for (entity, mut target) in player_query.iter_mut() {
        if target_id != entity {
            continue;
        }
        let pos: GridPosition = target.translation.into();
        let next = pos.into();
        target.translation = next;
    }
}

/// Snaps to an integer position in a grid defined mathematically.
fn snap_drop_wall(
    hit: Trigger<Pointer<DragEnd>>,
    mut player_query: Query<(Entity, &mut Transform), With<IsWallSnappable>>,
) {
    let target_id = hit.target;
    for (entity, mut target) in player_query.iter_mut() {
        if target_id != entity {
            continue;
        }
        // let modifier1 = (TRENCH_WIDTH-TILE_WIDTH)/2f32;
        // let modifier2 = (TILE_WIDTH + TRENCH_WIDTH)/2f32;
        // let modified_mouse_pos = target.translation + Vec3::ZERO.with_y(modifier1);
        // let pos: GridPosition = modified_mouse_pos.into();
        // let next = Vec3::from(pos) + Vec3::ZERO.with_y(modifier2);
        // target.translation = next;
        let mod1 = (TRENCH_WIDTH - TILE_WIDTH) / 2f32;
        let modified_mouse_pos = target.translation + Vec3::new(-TILE_WIDTH / 2f32, mod1, 0.0);
        let pos: GridPosition = modified_mouse_pos.into();
        let mut pos_as_vec: Vec3 = pos.into();
        // if pos_as_vec.x == 
        let next = pos_as_vec+ Vec3::new(STEP_SIZE / 2f32, STEP_SIZE / 2f32, 0.0);
        target.translation = next;
    }
}

pub fn draw_always_visible_wireframes(
    query: Query<(&Transform, &WireFrame), Without<GizmoOutlineToggle>>,
    mut gizmos: Gizmos,
) {
    for (transform, frame) in query.iter() {
        let point = transform.translation;
        frame.draw(point, &mut gizmos);
    }
}

pub fn draw_toggelable_visible_wireframes(
    query: Query<(&Transform, &WireFrame, &GizmoOutlineToggle)>,
    mut gizmos: Gizmos,
) {
    for (transform, frame, _) in query
        .iter()
        .filter(|(_, _, visibility)| visibility.is_visible())
    {
        let point = transform.translation;
        frame.draw(point, &mut gizmos);
    }
}

#[derive(Debug, Component, Default)]
struct IsHoverable;

#[derive(Debug, Component, Default)]
struct IsDroppable;

#[derive(Debug, Component, Default)]
struct IsDraggable;

#[derive(Debug, Component, Default)]
#[require(IsDraggable)]
struct IsCollidingDraggable;

#[derive(Debug, Component)]
struct IsSnapTarget;

#[derive(Debug, Component, Default)]
#[require(IsDroppable)]
struct IsTileSnappable;

#[derive(Debug, Component, Default)]
#[require(IsDroppable)]
struct IsWallSnappable;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyGizmos;

#[cfg(test)]
mod main_tests {}
