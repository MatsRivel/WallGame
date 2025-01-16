mod position_conversion;
mod game_board;
mod camera;
mod move_directions;

use std::f32::consts::PI;
use bevy::input::keyboard::KeyboardInput;
use bevy::pbr::CascadeShadowConfigBuilder;
use camera::{ControlledCamera, ControlledCameraIndentifier};
use game_board::{GizmoStruct, GizmoStructBundle, GridType, MyGizmos, Player};
use move_directions::MoveDirections;
use position_conversion::{pos_to_vec3, vec3_to_pos, Pos};
pub use bevy::prelude::*;
pub use bevy::color::palettes::css::*;
pub use bevy::input::mouse::MouseMotion;
const TILE_WIDTH: f32 = 64.0;
const TRENCH_WIDTH: f32 = 8.0;
const N_TILES: i32 = 5;
const STEP_SIZE: f32 = TILE_WIDTH + TRENCH_WIDTH;
pub const SKY_COLOR: Color = Color::linear_rgb(0.5, 0.5, 0.1);

fn main() {
    unsafe {std::env::set_var("WGPU_BACKEND", "vk");}
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(ClearColor(SKY_COLOR))
        .init_gizmo_group::<MyGizmos>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_camera)
        .add_systems(Update, rotate_light)
        .add_systems(Update, draw_gizmos)
        .add_systems(Update, adjust_material_color)
        // .add_systems(Update, move_player)
    .run();
}

pub fn setup( mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>,mut meshes: ResMut<Assets<Mesh>>){
    // let cursor = asset_server.load("Cursor.png");
    commands.spawn(ControlledCamera::new());
    commands.spawn((
        PointLight{ 
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
        }.build()
    ));
    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 5.0),
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

    let plane_dims = (N_TILES as f32)*STEP_SIZE/2f32;
    // Ground Plane
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -1.0),
        Mesh3d(meshes.add(Cuboid::from_corners(Vec3::new(-plane_dims, -plane_dims, -0.2), Vec3::new(plane_dims, plane_dims,  -0.1)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
    ));
    for grid_type in GridType::all(){
        for initial in GizmoStruct::initials(grid_type){
            let gs = initial;
            commands.spawn(GizmoStructBundle::new(gs, &mut materials, &mut meshes))
                .observe(tag_visible)
                .observe(tag_invisible)
                .observe(clickable_tile);
        }
    }
    let player_a = GizmoStruct::new_usize(2, 0, GridType::Tile);
    let player_b = GizmoStruct::new_usize(2, (N_TILES-1) as usize, GridType::Tile);

    commands.spawn(MyPlayerBundle{
        my_player: MyPlayer{
                player_id: Player::A,
                pos: player_a.pos()
            },
        mesh: Mesh3d(meshes.add(Sphere::new(TILE_WIDTH/3f32).mesh())),
        material: MeshMaterial3d(materials.add(StandardMaterial{ 
            base_color: Color::srgb(0.0, 1.0, 0.0),
            perceptual_roughness: 1.0,
            ..default()
        })),
        transform: Transform::from_translation(player_a.vec()),
    }).observe(drag).observe(drop);

    commands.spawn(MyPlayerBundle{
        my_player: MyPlayer{
                player_id: Player::B,
                pos: player_b.pos()
            },
        mesh: Mesh3d(meshes.add(Sphere::new(TILE_WIDTH/3f32).mesh())),
        material: MeshMaterial3d(materials.add(StandardMaterial{ 
            base_color: Color::srgb(1.0, 0.0, 0.0),
            perceptual_roughness: 1.0,
            ..default()
        })),
        transform: Transform::from_translation(player_b.vec()),
    }).observe(drag).observe(drop);
}

fn rotate_light(time: Res<Time>, mut light_query: Query<&mut Transform, With<PointLight>>, mut gizmos: Gizmos){
    for mut light in light_query.iter_mut(){
        let new_rot = Quat::from_rotation_z(((time.delta_secs()) / 25.0) * (360.0/(2.0*PI)));
        let new_rot = new_rot.normalize();
        light.rotate_around(Vec3::ZERO,new_rot);
        gizmos.circle(Isometry3d::from_translation(light.translation), 5.0, WHITE);
    }
}
fn drag(hit: Trigger<Pointer<Drag>>, mut player_query: Query<(Entity, &mut Transform), With<MyPlayer>>){
    let target_id = hit.target;
    for (entity, mut target) in player_query.iter_mut(){
        if target_id != entity{
            continue;
        }
        let pointer_location = hit.delta;
        target.translation += Vec3::new(pointer_location.x, -pointer_location.y, 0.0);
    }
}
fn drop(hit: Trigger<Pointer<DragEnd>>, mut player_query: Query<(Entity, &mut Transform), With<MyPlayer>>){
    println!("Drop triggered!");
    let target_id = hit.target;
    for (entity, mut target) in player_query.iter_mut(){
        if target_id != entity{
            continue;
        }
        let current_x = target.translation.x;
        let current_y = target.translation.y;
        let next = pos_to_vec3(vec3_to_pos(Vec3::new(current_x, current_y, 0.0), 0.0, 0.0), 0.0, 0.0);
        println!("current: {:?}, next: {:?}",target.translation, next);

        target.translation = next;
    }
}
fn clickable_tile(hit: Trigger<Pointer<Click>>, mut gs_query: Query<(Entity, &mut GizmoStruct)>){
    let target = hit.target;
    for (entity, mut gs) in gs_query.iter_mut(){
        if target == entity{
            match gs.get_occupant(){
                Some(p) => match p{
                    game_board::Player::A => {
                        gs.remove_player().unwrap();
                        gs.put_player(game_board::Player::B).unwrap();
                    },
                    game_board::Player::B => {
                        gs.remove_player().unwrap();
                    },
                },
                None => gs.put_player(game_board::Player::A).unwrap(),
            }
        }
    }
}

fn tag_visible(hit: Trigger<Pointer<Over>>, mut gs_query: Query<(Entity, &mut GizmoStruct)>){
    let target = hit.target;
    for (entity, mut gs) in gs_query.iter_mut(){
        if target == entity{
            gs.set_visible();
        }
    }
}

fn tag_invisible(hit: Trigger<Pointer<Out>>, mut gs_query: Query<(Entity, &mut GizmoStruct)>){
    let target = hit.target;
    for (entity, mut gs) in gs_query.iter_mut(){
        if target == entity && !gs.is_tile(){
            gs.set_invisible();
        }
    }
}

pub fn draw_gizmos(gs_query: Query<&GizmoStruct>, mut gizmos: Gizmos){
    for gs in gs_query.iter(){
        if gs.is_visible(){
            gs.draw_gizmo(&mut gizmos);
        }
    }
}

pub fn adjust_material_color(gs_query: Query<(&GizmoStruct, &MeshMaterial3d<StandardMaterial>)>, mut materials: ResMut<Assets<StandardMaterial>>){
    for (gs, mat) in gs_query.iter(){
        let id = mat.id();
        let color = match gs.get_occupant(){
            Some(p) => {
                match p{
                    game_board::Player::A => Color::Srgba(Srgba::new(0.0, 1.0, 0.0, 0.95)),
                    game_board::Player::B => Color::Srgba(Srgba::new( 1.0,0.0, 0.0, 0.95)),
                }
            },
            None => Color::Srgba(Srgba::new( 0.0,0.0, 0.0, 0.05)),
        };
        if let Some(material_asset) = materials.get_mut(id){
            material_asset.base_color = color;
        }
    }
}

fn move_camera(mut events: EventReader<KeyboardInput>, time: Res<Time>, mut controlled_camera_query: Query<&mut Transform,With<ControlledCameraIndentifier>>) {
    for event in events.read() {
        // Only check for characters when the key is pressed.
        if !event.state.is_pressed() {
            continue;
        }
        println!("{event:?}");
        let move_dir = MoveDirections::new_event(event).to_vec3();
        let mut cam = controlled_camera_query.single_mut();
        cam.translation += move_dir*25.0;
        *cam = cam.looking_at(Vec3::splat(0.0), Vec3::Y);
    }
}

#[derive(Bundle,Debug)]
pub struct MyPlayerBundle{
    my_player: MyPlayer,
    mesh: Mesh3d, 
    material: MeshMaterial3d<StandardMaterial>, 
    transform: Transform
}
#[derive(Debug,Clone,Component,PartialEq, Eq)]
#[require(Mesh3d, MeshMaterial3d<StandardMaterial>, Transform)]
pub struct MyPlayer{
    player_id: Player,
    pos: Pos
}

#[cfg(test)]
mod main_tests{
}