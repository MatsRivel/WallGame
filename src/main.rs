mod camera;
mod game_board;
mod move_directions;
mod pos;
mod position_conversion;
mod wireframe;
mod player;
mod tiles;
mod visibility_toggle;

pub use bevy::color::palettes::css::*;
// use bevy::gizmos::grid;
use bevy::input::keyboard::KeyboardInput;
pub use bevy::input::mouse::MouseMotion;
use bevy::pbr::CascadeShadowConfigBuilder;
pub use bevy::prelude::*;
use camera::{ControlledCamera, ControlledCameraIndentifier};
use game_board::{GridType, MyGizmos, Player};
use move_directions::MoveDirections;
use player::{MyPlayer, MyPlayerBundle};
use pos::GridPosition;
use position_conversion::{Pos, pos_to_vec3, vec3_to_pos};
use tiles::TileBundle;
use visibility_toggle::{tag_invisible_on_hover_end, tag_visible_on_hover, IsVisible};

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
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_camera)
        .add_systems(Update, rotate_light)
        .add_systems(Update, gizmo_drawables)
        .run();
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

    let player_a = GridPosition::new(2, 0);
    let player_b = GridPosition::new(2, (N_TILES - 1) as usize);

    commands
        .spawn(MyPlayerBundle::new(
            MyPlayer::new(Player::A,player_a),
            Mesh3d(meshes.add(Sphere::new(TILE_WIDTH / 3f32).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 4.0),
                perceptual_roughness: 1.0,
                ..default()
            })),
            Transform::from_translation(player_a.into()),
        ))
        .observe(drag)
        .observe(snap_drop);

    commands
        .spawn(MyPlayerBundle::new(
            MyPlayer::new(Player::B,player_b),
            Mesh3d(meshes.add(Sphere::new(TILE_WIDTH / 3f32).mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                perceptual_roughness: 1.0,
                ..default()
            })),
            Transform::from_translation(player_b.into())
        ))
        .observe(drag)
        .observe(snap_drop);

    for (x, y) in (0..N_TILES as usize).flat_map(|x| (0..N_TILES as usize).map(move |y| (x, y))) {
        let grid_position = GridPosition::new(x, y);
        commands
            .spawn(
                TileBundle::new(grid_position, GridType::Tile, &mut materials, &mut meshes)
                    .unwrap(),
            )
            .observe(tag_invisible_on_hover_end)
            .observe(tag_visible_on_hover);
        if x as i32 != N_TILES - 1 {
            commands
                .spawn(
                    TileBundle::new(
                        grid_position,
                        GridType::Vertical,
                        &mut materials,
                        &mut meshes,
                    )
                    .unwrap(),
                )
                .observe(tag_invisible_on_hover_end)
                .observe(tag_visible_on_hover);
        }
        if y as i32 != N_TILES - 1 {
            commands
                .spawn(
                    TileBundle::new(
                        grid_position,
                        GridType::Horizontal,
                        &mut materials,
                        &mut meshes,
                    )
                    .unwrap(),
                )
                .observe(tag_invisible_on_hover_end)
                .observe(tag_visible_on_hover);
        }
        // commands.spawn(TileBundle::new(grid_position,GridType::Circle, &mut materials,&mut meshes).unwrap());
    }
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

fn drag(
    hit: Trigger<Pointer<Drag>>,
    mut player_query: Query<(Entity, &mut Transform), With<IsDraggable>>,
) {
    let target_id = hit.target;
    for (entity, mut target) in player_query.iter_mut() {
        if target_id != entity {
            continue;
        }
        let pointer_location = hit.delta;
        target.translation += Vec3::new(pointer_location.x, -pointer_location.y, 0.0);
    }
}

fn snap_drop(
    hit: Trigger<Pointer<DragEnd>>,
    mut player_query: Query<(Entity, &mut Transform), With<IsSnappable>>,
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

pub fn gizmo_drawables(
    query: Query<(&Transform, &WireFrame, &IsVisible), With<GridPosition>>,
    mut gizmos: Gizmos,
) {
    for (transform, frame,_) in query.iter().filter(|(_,_,visibility)| visibility.is_visible()) {
        let point = transform.translation;
        frame.draw(point, &mut gizmos);
    }
}



fn move_camera(
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

#[derive(Debug,Component,Default)]
struct IsDroppable;

#[derive(Debug,Component,Default)]
struct IsDraggable;

#[derive(Debug,Component)]
struct IsSnapTarget;

#[derive(Debug,Component,Default)]
#[require(IsDroppable)]
struct IsSnappable;

#[cfg(test)]
mod main_tests{
}