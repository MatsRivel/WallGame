#![feature(generic_const_exprs)]
mod move_directions;
// mod original_game_logic;

use core::panic;
use std::f32::consts::PI;

use bevy::{color::palettes::css, gizmos::gizmos};
use bevy::input::mouse::{self, MouseButtonInput};
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
type Pos = [usize;2];
mod pos_mod{
    use super::*;
    pub fn pos_to_vec3(pos: Pos, xmod: f32, ymod: f32)->Vec3{
        let [x,y] = pos;
        let a1 = x as i32 - (N_TILES/2);
        let b1 = y as i32 - (N_TILES/2);
        let a2 = a1 as f32 * STEP_SIZE - TRENCH_WIDTH + xmod;
        let b2 = b1 as f32 * STEP_SIZE - TRENCH_WIDTH + ymod;
        let pos = Vec3::new( a2,b2, 0.0);
        pos
    }
    pub fn vec3_to_pos(v: Vec3, xmod: f32, ymod: f32)->Pos{
        let a3 = v.x;
        let b3 = v.y;
        let a2 = (a3 + TRENCH_WIDTH - xmod) / STEP_SIZE;
        let b2 = (b3 + TRENCH_WIDTH - ymod) / STEP_SIZE;
        let a1 = (a2 as i32) + (N_TILES*2) + (1-N_TILES%2);
        let b1 = (b2 as i32) + (N_TILES*2) + (1-N_TILES%2);
        let x = a1 as usize;
        let y = b1 as usize;
        [x,y]
    }

    pub fn tile_to_pos(v: Vec3)->Pos{
        let xmod = - TILE_WIDTH/2f32;
        let ymod = - TILE_WIDTH/2f32;
        vec3_to_pos(v, xmod, ymod)
    }
    pub fn wall_horizontal_to_pos(v: Vec3)->Pos{
        let xmod = - TILE_WIDTH/2f32;
        let ymod = TRENCH_WIDTH/2f32;
        vec3_to_pos(v, xmod, ymod)
    }
    pub fn wall_vertical_to_pos(v: Vec3)->Pos{
        let xmod = TILE_WIDTH/2f32;
        let ymod = -TRENCH_WIDTH/2f32;
        vec3_to_pos(v, xmod, ymod)
    }
    pub fn wall_circle_to_pos(v: Vec3)->Pos{
        let xmod = TRENCH_WIDTH/2f32;
        let ymod = TRENCH_WIDTH/2f32;
        vec3_to_pos(v, xmod, ymod)
    }


    pub fn pos_to_tile(pos: Pos)->Vec3{
        let xmod = - TILE_WIDTH/2f32;
        let ymod = - TILE_WIDTH/2f32;
        pos_to_vec3(pos, xmod, ymod)
    }

    pub fn pos_to_wall_horizontal(pos: Pos)->Vec3{
        let xmod = - TILE_WIDTH/2f32;
        let ymod = TRENCH_WIDTH/2f32;
        pos_to_vec3(pos, xmod, ymod)
    }

    pub fn pos_to_wall_vertical(pos: Pos)->Vec3{
        let xmod = TILE_WIDTH/2f32;
        let ymod = -TRENCH_WIDTH/2f32;
        pos_to_vec3(pos, xmod, ymod)
    }

    pub fn pos_to_wall_cirlce(pos: Pos)->Vec3{
        let xmod = TRENCH_WIDTH/2f32;
        let ymod = TRENCH_WIDTH/2f32;
        pos_to_vec3(pos, xmod, ymod)
    }

    pub fn initial_tile_positions_tile()->Vec<Vec3>{
        (0..N_TILES).flat_map(|x| (0..N_TILES).map(move |y| {
            pos_to_tile([x as usize, y as usize])
        })).collect()
    }

    pub fn initial_tile_positions_wall_horizontal()->Vec<Vec3>{
        (0..N_TILES).flat_map(|x| {
            (0..N_TILES-1).map(move |y| {
                pos_to_wall_horizontal([x as usize, y as usize])
            })
        }).collect()
    }

    pub fn initial_tile_positions_wall_vertical()->Vec<Vec3>{
        (0..N_TILES-1).flat_map(|x| {
            (0..N_TILES).map(move |y| {
                pos_to_wall_vertical([x as usize, y as usize])
            })
        }).collect()
    }

    pub fn initial_tile_positions_wall_circle()->Vec<Vec3>{
        (0..N_TILES-1).flat_map(|x| {
            (0..N_TILES-1).map(move |y| {
                pos_to_wall_cirlce([x as usize, y as usize])
            })
        }).collect()
    }
    #[cfg(test)]
    mod pos_tests{
        use super::*;
        const X: f32 = -9f32;
        const Y: f32 = 2f32;
        #[test]
        fn converstion_test_horizontal(){
            let v = Vec3::new(X, Y, 0.0);
            let a = wall_horizontal_to_pos(v);
            let b = pos_to_wall_horizontal(a);
            let c = wall_horizontal_to_pos(b);
            assert_eq!(a,c)
        }
        #[test]
        fn converstion_test_vertical(){
            let v = Vec3::new(X, Y, 0.0);
            let a = wall_vertical_to_pos(v);
            let b = pos_to_wall_vertical(a);
            let c = wall_vertical_to_pos(b);
            assert_eq!(a,c)
        }
        #[test]
        fn converstion_test_circle(){
            let v = Vec3::new(X, Y, 0.0);
            let a = wall_circle_to_pos(v);
            let b = pos_to_wall_cirlce(a);
            let c = wall_circle_to_pos(b);
            assert_eq!(a,c)
        }
        #[test]
        fn converstion_test_tile(){
            let v = Vec3::new(X, Y, 0.0);
            let a = tile_to_pos(v);
            let b = pos_to_tile(a);
            let c = tile_to_pos(b);
            assert_eq!(a,c)
        }
    }
}

pub fn setup(
        mut commands: Commands, 
        mut meshes: ResMut<Assets<Mesh>>, 
        mut materials: ResMut<Assets<StandardMaterial>>, 
        asset_server: Res<AssetServer>,
        mut gizmos: Gizmos){
    // let tile = asset_server.load("Tile.png");
    let cursor = asset_server.load("Cursor.png");
    commands.spawn(ControlledCamera::new());

    commands.spawn((
        MouseIdentifier,
        Transform::from_translation(vec3(0.0, 0.0, 2.0)),
        Sprite::from_image(cursor.clone()),
    ));

}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericGrid<const N: usize, const M: usize>{
    grid: [[IsVisible;N];M]
}
impl <const N: usize, const M: usize>GenericGrid<N,M>{
    fn new()->Self {
        Self{grid:[[IsVisible::Visible;N];M]}
    }

    fn get_visibility(&self,pos: Pos)->Option<IsVisible> {
        self.grid.get(pos[0])?.get(pos[1]).copied()
    }

    fn set_visible(&mut self, pos: Pos)->bool {
        match self.grid.get_mut(pos[0]){
            Some(v) => {
                match v.get(pos[1]){
                    Some(v) => {Some(*v); true},
                    None => false,
                }
            }
            None => false
        }
    }
    fn set_invisible(&mut self, pos: Pos)->bool {
        match self.grid.get_mut(pos[0]){
            Some(v) => {
                match v.get(pos[1]){
                    Some(v) => {Some(*v); true},
                    None => false,
                }
            }
            None => false
        }
    }
    pub fn get_visible_coords(&self)->Vec<Pos>{
        self.grid.iter().enumerate().flat_map(|(x,row)|{
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


#[derive(Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WallGrid{
    horizontal: GenericGrid<M_WALLS,N_WALLS>,
    vertical: GenericGrid<N_WALLS,M_WALLS>,
}
impl WallGrid{
    pub fn new()->Self{
        let horizontal = GenericGrid::<M_WALLS,N_WALLS>::new();
        let vertical = GenericGrid::<N_WALLS,M_WALLS>::new();
        Self { horizontal, vertical }
    }
    pub fn get_visibility(&self,pos: Pos, orientation: Orientation)->Option<IsVisible>{
        match orientation{
            Orientation::Horizontal => self.horizontal.get_visibility(pos),
            Orientation::Vertical => self.vertical.get_visibility(pos),
        }

    }
    pub fn set_visible(&mut self, pos: Pos, orientation: Orientation)->bool{
        match orientation{
            Orientation::Horizontal => self.horizontal.set_visible(pos),
            Orientation::Vertical => self.vertical.set_visible(pos),
        }

    }
    pub fn set_invisible(&mut self, pos: Pos, orientation: Orientation)->bool{
        match orientation{
            Orientation::Horizontal => self.horizontal.set_invisible(pos),
            Orientation::Vertical => self.vertical.set_invisible(pos),
        }

    }
    pub fn get_visible_coords(&self, orientation: Orientation)->Vec<Pos>{
        match orientation{
            Orientation::Horizontal => self.horizontal.get_visible_coords(),
            Orientation::Vertical => self.vertical.get_visible_coords(),
        }
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

use pos_mod::*;
fn grid_gizmos(mut gizmos: Gizmos, mouse_movement:Query<&Window, With<PrimaryWindow>> ){
    let wall_grid = WallGrid::new();
    for image_position in initial_tile_positions_tile(){
        gizmos.rect(image_position, Vec2::new(TILE_WIDTH, TILE_WIDTH), GREEN);
    }
    let width = TILE_WIDTH -4.0;
    let height = 1.0;
    for [x,y] in wall_grid.get_visible_coords(Orientation::Horizontal){
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
    for [x,y] in wall_grid.get_visible_coords(Orientation::Vertical){
        let a1 = x as i32 - (N_TILES)/2;
        let b1 = y as i32 - (N_TILES)/2;
        let a2 = a1 as f32 * STEP_SIZE - TRENCH_WIDTH/2f32;
        let b2 = b1 as f32 * STEP_SIZE - TRENCH_WIDTH - TILE_WIDTH/2f32;
        let pos = Vec3::new( a2,b2, 1.0);
        gizmos.rect(pos, Vec2::new(height, width), RED);
    }
    let window = mouse_movement.single();
    if let Some(position) = window.cursor_position(){
        let v = Vec3::new(position.x, position.y, 1.0);
        let a = pos_to_tile(tile_to_pos(v));
        println!("{v:?}, {a:?}");
        let b = pos_to_wall_horizontal(wall_horizontal_to_pos(v));
        let c = pos_to_wall_vertical(wall_vertical_to_pos(v));
        let d = pos_to_wall_cirlce(wall_circle_to_pos(v));
        for point in [a,b,c,d].iter(){
            gizmos.circle(*point, 5.0, PURPLE);
        }
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

#[cfg(test)]
mod main_tests{
    // use super::*;
    // const X: f32 = -9f32;
    // const Y: f32 = 2f32;
    // #[test]
    // fn converstion_test_horizontal(){
    //     let v = Vec3::new(X, Y, 0.0);
    //     let a = wall_horizontal_to_pos(v);
    //     let b = pos_to_wall_horizontal(a);
    //     let c = wall_horizontal_to_pos(b);
    //     assert_eq!(a,c)
    // }
    // #[test]
    // fn converstion_test_vertical(){
    //     let v = Vec3::new(X, Y, 0.0);
    //     let a = wall_vertical_to_pos(v);
    //     let b = pos_to_wall_vertical(a);
    //     let c = wall_vertical_to_pos(b);
    //     assert_eq!(a,c)
    // }
    // #[test]
    // fn converstion_test_circle(){
    //     let v = Vec3::new(X, Y, 0.0);
    //     let a = wall_circle_to_pos(v);
    //     let b = pos_to_wall_cirlce(a);
    //     let c = wall_circle_to_pos(b);
    //     assert_eq!(a,c)
    // }
    // #[test]
    // fn converstion_test_tile(){
    //     let v = Vec3::new(X, Y, 0.0);
    //     let a = tile_to_pos(v);
    //     let b = pos_to_tile(a);
    //     let c = tile_to_pos(b);
    //     assert_eq!(a,c)
    // }
}