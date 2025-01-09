#![feature(generic_const_exprs)]
mod move_directions;
// mod original_game_logic;

use core::panic;
use std::f32::consts::PI;

use bevy::{color::palettes::css, gizmos::gizmos};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
pub use bevy::prelude::*;
// pub use bevy_mod_raycast::prelude::*;
pub use bevy::color::palettes::css::*;
pub use bevy::math::*;
pub use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;
// use original_game_logic::tile::Tile;
// use original_game_logic::{board::{Board, Player}, game_error::GameError, orientation::{self, Cardinality, Orientation}};
// use original_game_logic::position::Position;
use move_directions::MoveDirections;
use orientation::{Cardinality, Orientation};
const SPEED: f32 = 100.0;
const TILE_WIDTH: f32 = 64.0;
const TRENCH_WIDTH: f32 = 8.0;
const N_TILES: i32 = 5;
const N_WALLS: usize = N_TILES as usize;
const M_WALLS: usize = N_TILES as usize -1;
const DOUBLE_N_TILES: usize = N_TILES as usize * 2+1;
const STEP_SIZE: f32 = TILE_WIDTH + TRENCH_WIDTH;
const WALL_LENGTH: f32 = 128.0;
pub const SKY_COLOR: Color = Color::linear_rgb(0.5, 0.5, 0.1);
const ADJUSTER_STEP: f32 = (N_TILES as f32 * (0.59) as f32)*STEP_SIZE;
const ADJUSTER: Vec3 =  Vec3::new(ADJUSTER_STEP, ADJUSTER_STEP, 0.0);
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyGizmos;

fn main() {
    unsafe {std::env::set_var("WGPU_BACKEND", "vk");}
    App::new()
        .add_plugins((DefaultPlugins))
        .insert_resource(ClearColor(SKY_COLOR))
        .init_gizmo_group::<MyGizmos>()
        .add_systems(Startup,   setup)
        // .add_systems(Update, place_block)
        .add_systems(Update, move_cursor)
        .add_systems(Update, grid_gizmos)
        .run();
}

#[derive(Bundle)]
pub struct WallBundle{
    sprite: Sprite,
    transform: Transform,
    visible: IsVisible
}
impl WallBundle{
    pub fn new(image_pos: Vec3, asset_server: &Res<AssetServer>, rot: Option<f32>)->Self{
        let wall: Handle<Image> = asset_server.load("Tile.png");
        let sprite = Sprite::from_image(wall);
        let mut transform = Transform::from_translation(image_pos.clone())
            .with_scale(Vec3::new(1.0, 0.25, 1.0));
        if let Some(radians) = rot{
            transform.rotate(Quat::from_rotation_z(radians));
        }
        Self{sprite,transform, visible: IsVisible::Visible}
    }
    pub fn set_pos(&mut self, translation: Vec3){
        self.transform.translation = translation
    }
}
mod orientation{
    use bevy::math::Vec3;

    #[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
    pub enum Orientation{
        Horizontal,
        Vertical
    }
    pub enum Cardinality{
        North,East,South,West
    }
    impl Cardinality{
        pub fn to_vec3(&self)->Vec3{
            match self{
                Cardinality::North => Vec3::new(-1.0,  0.0, 0.0),
                Cardinality::East  => Vec3::new( 0.0,  1.0, 0.0),
                Cardinality::South => Vec3::new( 1.0,  0.0, 0.0),
                Cardinality::West  => Vec3::new( 0.0, -1.0, 0.0),
            }
        }
    }
}
pub fn initial_tile_positions_tile()->Vec<Vec3>{
    (0..N_TILES).flat_map(|x| (0..N_TILES).map(move |y| {
        let a1 = x - (N_TILES)/2;
        let a2 = a1 as f32 * STEP_SIZE - TRENCH_WIDTH - TILE_WIDTH/2f32;
        let b1 = y - (N_TILES)/2;
        let b2 = b1 as f32 * STEP_SIZE - TRENCH_WIDTH - TILE_WIDTH/2f32;
        let pos = Vec3::new( a2,b2, 0.0);
        pos
    })).collect()
}
pub fn initial_tile_positions_wall_horizontal()->Vec<Vec3>{
    (0..N_TILES).flat_map(|x| {
        (0..N_TILES-1).map(move |y| {
            let a1 = x - (N_TILES)/2;
            let b1 = y - (N_TILES)/2;
            let a2 = a1 as f32 * STEP_SIZE - TRENCH_WIDTH - TILE_WIDTH/2f32;
            let b2 = b1 as f32 * STEP_SIZE - TRENCH_WIDTH/2f32;
            let pos = Vec3::new( a2,b2, 1.0);
            pos
        })
    }).collect()
}
pub fn initial_tile_positions_wall_vertical()->Vec<Vec3>{
    (0..N_TILES-1).flat_map(|x| {
        (0..N_TILES).map(move |y| {
            let a1 = x - (N_TILES)/2;
            let b1 = y - (N_TILES)/2;
            let a2 = a1 as f32 * STEP_SIZE - TRENCH_WIDTH/2f32;
            let b2 = b1 as f32 * STEP_SIZE - TRENCH_WIDTH - TILE_WIDTH/2f32;
            let pos = Vec3::new( a2,b2, 1.0);
            pos
        })
    }).collect()
}
pub fn initial_tile_positions_wall_circle()->Vec<Vec3>{
    (0..N_TILES-1).flat_map(|x| {
        (0..N_TILES-1).map(move |y| {
            let a1 = x - (N_TILES)/2;
            let b1 = y - (N_TILES)/2;
            let a2 = a1 as f32 * STEP_SIZE - TRENCH_WIDTH/2f32;
            let b2 = b1 as f32 * STEP_SIZE - TRENCH_WIDTH/2f32;
            let pos = Vec3::new( a2,b2, 1.0);
            pos
        })
    }).collect()
}
pub fn setup(
        mut commands: Commands, 
        mut meshes: ResMut<Assets<Mesh>>, 
        mut materials: ResMut<Assets<StandardMaterial>>, 
        asset_server: Res<AssetServer>,
        mut gizmos: Gizmos){
    // let tile = asset_server.load("Tile.png");
    // let p1 = asset_server.load("src/assets/P1.png");
    // let p2 = asset_server.load("src/assets/P2.png");
    // let wall = asset_server.load("Wall.png");
    let cursor = asset_server.load("Cursor.png");
    commands.spawn(ControlledCamera::new());

    // for image_position in initial_tile_positions_tile(){
    //     let tile_transform = Transform::from_translation(image_position);
    //     commands.spawn((
    //         Sprite::from_image(tile.clone()),
    //         tile_transform
    //     ));
    // }
    // for wall_position in initial_tile_positions_wall_horizontal(){
    //     commands.spawn(WallBundle::new(wall_position, &asset_server, Some(0.0) ));
    // }
    // for wall_position in initial_tile_positions_wall_vertical(){
    //     commands.spawn(WallBundle::new(wall_position, &asset_server, Some(PI/2.0) ));
    // }


    commands.spawn((
        MouseIdentifier,
        Transform::from_translation(vec3(0.0, 0.0, 2.0)),
        Sprite::from_image(cursor.clone()),
    ));

}
#[derive(Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WallGrid{
    horizontal: [[IsVisible;M_WALLS];N_WALLS],
    vertical: [[IsVisible;N_WALLS];M_WALLS]
}
impl WallGrid{
    pub fn new()->Self{
        let horizontal = [[IsVisible::Visible;M_WALLS];N_WALLS];
        let vertical = [[IsVisible::Visible;N_WALLS];M_WALLS];
        Self { horizontal, vertical }
    }
    pub fn get_visibility(&self,pos: [usize;2], orientation: Orientation)->Option<IsVisible>{
        let row = match orientation{
            Orientation::Horizontal => self.horizontal.get(pos[0]),
            Orientation::Vertical => self.horizontal.get(pos[0]),
        }?;
        match row.get(pos[1]){
            Some(v) => Some(*v),
            None => None,
        }
    }
    pub fn set_visible(&mut self, pos: [usize;2], orientation: Orientation)->bool{
        let row = match orientation{
            Orientation::Horizontal => self.horizontal.get_mut(pos[0]),
            Orientation::Vertical => self.horizontal.get_mut(pos[0]),
        };
        if row.is_none(){
            return false;
        }
        let row = row.unwrap();
        match row.get_mut(pos[1]){
            Some(v) => {
                *v = IsVisible::Visible;
                true
            },
            None => false,
        }
    }
    pub fn set_invisible(&mut self, pos: [usize;2], orientation: Orientation)->bool{
        let row = match orientation{
            Orientation::Horizontal => self.horizontal.get_mut(pos[0]),
            Orientation::Vertical => self.horizontal.get_mut(pos[0]),
        };
        if row.is_none(){
            return false;
        }
        let row = row.unwrap();
        match row.get_mut(pos[1]){
            Some(v) => {
                *v = IsVisible::Invisible;
                true
            },
            None => false,
        }
    }
    pub fn get_visible_coords_horizontal(&self)->Vec<[usize;2]>{
        self.horizontal.iter().enumerate().flat_map(|(x,row)|{
            row.iter().enumerate().filter_map(move |(y, point)|{
                if *point == IsVisible::Visible{
                    Some([x,y])
                }else{
                    None
                }
            })
        }).collect()
    }
    pub fn get_visible_coords_vertical(&self)->Vec<[usize;2]>{
        self.vertical.iter().enumerate().flat_map(|(x,row)|{
            row.iter().enumerate().filter_map(move |(y, point)|{
                if *point == IsVisible::Visible{
                    Some([x,y])
                }else{
                    None
                }
            })
        }).collect()
    }
}
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsVisible{
    Visible,
    Invisible
}
#[derive(Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TileGrid{
    grid: Vec<Vec<Player>>
}
#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player{
    A,
    B,
    None
}

fn grid_gizmos(mut gizmos: Gizmos){
    let wall_grid = WallGrid::new();
    for image_position in initial_tile_positions_tile(){
        gizmos.rect(image_position, Vec2::new(TILE_WIDTH, TILE_WIDTH), GREEN);
    }
    let width = TILE_WIDTH -4.0;
    let height = 1.0;
    for [x,y] in wall_grid.get_visible_coords_horizontal(){
        let a1 = x as i32 - (N_TILES)/2;
        let b1 = y as i32 - (N_TILES)/2;
        let a2 = a1 as f32 * STEP_SIZE - TRENCH_WIDTH - TILE_WIDTH/2f32;
        let b2 = b1 as f32 * STEP_SIZE - TRENCH_WIDTH/2f32;
        let pos = Vec3::new( a2,b2, 1.0);
        gizmos.rect(pos, Vec2::new(width, height), RED);
    }
    for circle_pos in initial_tile_positions_wall_circle().into_iter().skip(1){
        gizmos.circle(circle_pos, 5.0, RED);
    }
    for [x,y] in wall_grid.get_visible_coords_vertical(){
        let a1 = x as i32 - (N_TILES)/2;
        let b1 = y as i32 - (N_TILES)/2;
        let a2 = a1 as f32 * STEP_SIZE - TRENCH_WIDTH/2f32;
        let b2 = b1 as f32 * STEP_SIZE - TRENCH_WIDTH - TILE_WIDTH/2f32;
        let pos = Vec3::new( a2,b2, 1.0);
        gizmos.rect(pos, Vec2::new(height, width), RED);
    }
}

fn move_cursor(mut cursor: Query<&mut Transform, With<MouseIdentifier>>, mouse_movement:Query<&Window, With<PrimaryWindow>>){
    let window = mouse_movement.single();
    if let Some(position) = window.cursor_position(){
        let corrected_position = vec3(position.x -window.width()/2.0,   window.height()/2.0 - position.y,2.0);
        cursor.single_mut().translation = corrected_position;
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
    render_graph: Camera2d,
    transform: Transform,
}
impl ControlledCamera{
    pub fn new()->Self{
        #[cfg(debug_assertions)]
        println!("Making camera!");
        let identifier = ControlledCameraIndentifier;
        let camera = Camera::default();
        let render_graph = Camera2d::default();
        let mut transform = Transform{
            translation: Vec3::new(0.0, 0.0, 10.0),
            ..default()
        };
        transform.look_at(Vec3::ZERO, Vec3::Y);
        Self{identifier, camera,render_graph,transform}
    }
}
pub fn central_light(mut commands: Commands){
    // Add a light source
    commands.spawn((PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
    }, Transform::from_xyz(0.0, 0.0, 0.0)));
}