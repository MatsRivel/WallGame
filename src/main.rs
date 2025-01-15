#![feature(random)]
mod position_conversion;
mod game_board;
use std::f32::consts::PI;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::math::VectorSpace;
use bevy::pbr::CascadeShadowConfigBuilder;
use game_board::{GizmoStruct, GizmoStructBundle, GridType, MyGizmos};
use move_directions::MoveDirections;
use position_conversion::{pos_to_vec3, vec3_to_pos, Pos};
pub use bevy::prelude::*;
pub use bevy::color::palettes::css::*;
pub use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;
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
        .add_systems(Update, move_cursor,)
        .add_systems(Update, adjust_material_color)
    .run();
}

pub fn setup( mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>,mut meshes: ResMut<Assets<Mesh>>,){
    let cursor = asset_server.load("Cursor.png");
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
    commands.spawn((
        MouseIdentifier,
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        Sprite::from_image(cursor.clone()),
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
}

fn rotate_light(time: Res<Time>, mut light_query: Query<&mut Transform, With<PointLight>>, mut gizmos: Gizmos){

    for mut light in light_query.iter_mut(){
        let current_rot = light.rotation;
        let new_rot = Quat::from_rotation_z(((time.delta_secs()) / 25.0) * (360.0/(2.0*PI)));
        let new_rot = new_rot.normalize();
        println!("-- {new_rot}");
        light.rotate_around(Vec3::ZERO,new_rot);
        gizmos.circle(Isometry3d::from_translation(light.translation), 5.0, WHITE);
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
        if target == entity && !gs.is_tile(){
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

fn move_cursor(mut cursor: Query<&mut Transform, With<MouseIdentifier>>, mouse_movement:Query<&Window, With<PrimaryWindow>>){
    let window = mouse_movement.single();
    if let Some(position) = window.cursor_position(){
        let corrected_position = Vec3::new(position.x -window.width()/2.0,   window.height()/2.0 - position.y,2.0);
        cursor.single_mut().translation = corrected_position;
    }
}

mod move_directions;
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

#[derive(Component)]
pub struct MouseIdentifier;

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

#[cfg(test)]
mod main_tests{
}