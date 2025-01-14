mod position_conversion;
use bevy::asset::RenderAssetUsages;
use bevy::picking::focus::PickingInteraction;
use bevy::picking::pointer::PointerInteraction;
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
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyGizmos;

fn main() {
    unsafe {std::env::set_var("WGPU_BACKEND", "vk");}
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .insert_resource(ClearColor(SKY_COLOR))
        .init_gizmo_group::<MyGizmos>()
        .add_systems(Startup,   setup)
        // .add_systems(Update, mouse_ray_update)
        .add_systems(Update, draw_gizmos)
        .add_systems(Update, move_cursor,)
        // .add_systems(Update, mouse_visualization)
        .run();
}

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
pub enum GridType{
    Tile,
    Cirle,
    Horizontal,
    Vertical
}
impl GridType{
    pub fn all()->Vec<Self>{
        vec![Self::Tile,Self::Cirle,Self::Horizontal,Self::Vertical]
    }
}
#[derive(Clone,Debug,Bundle)]
pub struct GizmoStructBundle{
    gs: GizmoStruct,
    transform: Transform,
    mesh_3d: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>
}
impl GizmoStructBundle{
    pub fn new(gs: GizmoStruct, materials: &mut ResMut<Assets<StandardMaterial>>, meshes: &mut ResMut<Assets<Mesh>>)->Self{
        let transform = Transform::from_translation(gs.vec());
        let material = MeshMaterial3d(materials.add(Color::BLACK));
        let shape = match gs.grid_type{
            GridType::Tile => Cuboid::from_corners(gs.vec() - Vec3::new(TILE_WIDTH/2f32, TILE_WIDTH/2f32, 0.0), gs.vec() + Vec3::new(TILE_WIDTH/2f32, TILE_WIDTH/2f32, 4.0)),
            GridType::Cirle => Cuboid::default(),
            GridType::Horizontal => Cuboid::default(),
            GridType::Vertical => Cuboid::default(),
        };
        let mesh_3d = Mesh3d(meshes.add(shape));
        Self { gs, transform, mesh_3d, material }
    }
}

#[derive(Clone, Debug, Component)]
#[require(Mesh3d,Transform)]
pub struct GizmoStruct{
    xmod: f32,
    ymod: f32,
    vec: Vec3,
    pos: Pos,
    grid_type: GridType,
    is_visible: IsVisible,
}

impl GizmoStruct{
    pub fn new_float(x: f32, y: f32, grid_type: GridType)->Self{
        let (xmod, ymod) = match grid_type{
            GridType::Tile => (- TILE_WIDTH/2f32, - TILE_WIDTH/2f32),
            GridType::Cirle => (TRENCH_WIDTH/2f32,TRENCH_WIDTH/2f32),
            GridType::Horizontal => (- TILE_WIDTH/2f32, TRENCH_WIDTH/2f32),
            GridType::Vertical => ( -TILE_WIDTH/2f32, TRENCH_WIDTH/2f32),
        };
        let pos = vec3_to_pos(Vec3::new(x, y, 0.0), xmod, ymod);
        let vec = pos_to_vec3(pos, xmod, ymod);
        let is_visible = IsVisible::Invisible;
        Self { xmod, ymod, vec, pos, grid_type, is_visible }
    }
    pub fn new_usize(x: usize, y: usize, grid_type: GridType)->Self{
        let (xmod, ymod) = match grid_type{
            GridType::Tile => (- TILE_WIDTH/2f32, - TILE_WIDTH/2f32),
            GridType::Cirle => (TRENCH_WIDTH/2f32,TRENCH_WIDTH/2f32),
            GridType::Horizontal => (- TILE_WIDTH/2f32, TRENCH_WIDTH/2f32),
            GridType::Vertical => ( TRENCH_WIDTH/2f32, -TILE_WIDTH/2f32),
        };
        let pos = [x,y];
        let vec = pos_to_vec3([x as usize, y as usize], xmod, ymod);
        let is_visible = IsVisible::Invisible;
        Self { xmod, ymod, vec, pos, grid_type, is_visible }
    }
    pub fn xmod(&self)-> f32{
        self.xmod
    }
    pub fn ymod(&self)-> f32{
        self.ymod
    }
    pub fn vec(&self)-> Vec3{
        self.vec
    }
    pub fn center(&self)-> Vec3{
        pos_to_vec3(self.pos, self.xmod, self.ymod)
    }
    pub fn pos(&self)-> Pos{
        self.pos
    }
    pub fn initials(grid_type: GridType)->Vec<Self>{
        match grid_type{
            GridType::Tile => {
                (0..N_TILES).flat_map(|x| (0..N_TILES).map(move |y| {
                    Self::new_usize(x as usize, y as usize, grid_type)
                })).collect()
            },
            GridType::Cirle => {
                (0..N_TILES-1).flat_map(|x| {
                    (0..N_TILES-1).map(move |y| {
                        Self::new_usize(x as usize, y as usize, grid_type)
                    })
                }).collect()
            },
            GridType::Horizontal => {
                (0..N_TILES).flat_map(|x| {
                    (0..N_TILES-1).map(move |y| {
                        Self::new_usize(x as usize, y as usize, grid_type)
                    })
                }).collect()
            },
            GridType::Vertical => {
                (0..N_TILES-1).flat_map(|x| {
                    (0..N_TILES).map(move |y| {
                        Self::new_usize(x as usize, y as usize, grid_type)
                    })
                }).collect()
            },
        }

    }
    
    pub fn draw_gizmo(&self, gizmos: &mut Gizmos){
        let point = self.vec();
        let width = TILE_WIDTH -4.0;
        let height = 1.0;
        match self.grid_type{
            GridType::Tile       => { gizmos.rect(point, Vec2::new(TILE_WIDTH, TILE_WIDTH), GREEN);},
            GridType::Cirle      => { gizmos.circle(point, 5.0, RED);},
            GridType::Horizontal => { gizmos.rect(point, Vec2::new(width, height), RED);},
            GridType::Vertical   => { gizmos.rect(point, Vec2::new(height, width), RED);},
        };
    }
}

pub fn setup( mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>,mut meshes: ResMut<Assets<Mesh>>,){
    let cursor = asset_server.load("Cursor.png");
    commands.spawn(ControlledCamera::new());
    commands.spawn((
        MouseIdentifier,
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        Sprite::from_image(cursor.clone()),
    ));
    for grid_type in GridType::all(){
        for initial in GizmoStruct::initials(grid_type){
            let x = initial.vec.x;
            let y = initial.vec.y;
            let z = initial.vec.z;
            let gs = initial;
            commands.spawn(GizmoStructBundle::new(gs, &mut materials, &mut meshes))
                .observe(tag_visible)
                .observe(tag_invisible);
        }
    }
}

fn tag_visible(hit: Trigger<Pointer<Over>>, mut gs_query: Query<(Entity, &mut GizmoStruct)>){
    let target = hit.target;
    for (entity, mut gs) in gs_query.iter_mut(){
        if target == entity && gs.grid_type != GridType::Tile{
            gs.is_visible = IsVisible::Visible;
        }
    }
}

fn tag_invisible(hit: Trigger<Pointer<Out>>, mut gs_query: Query<(Entity, &mut GizmoStruct)>){
    let target = hit.target;
    for (entity, mut gs) in gs_query.iter_mut(){
        if target == entity && gs.grid_type != GridType::Tile{
            gs.is_visible = IsVisible::Invisible;
        }
    }
}

fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK);
    }
}

pub fn draw_gizmos(gs_query: Query<&GizmoStruct>, mut gizmos: Gizmos){
    for gs in gs_query.iter(){
        if gs.is_visible == IsVisible::Visible || gs.grid_type ==  GridType::Tile{
            gs.draw_gizmo(&mut gizmos);
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum IsVisible{
    Visible,
    Invisible
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Player{
    A,
    B,
    None
}

fn mouse_visualization(mut gizmos: Gizmos, mouse_movement:Query<&Window, With<PrimaryWindow>> ){
    let window = mouse_movement.single();
    if let Some(position) = window.cursor_position(){
        let v = Vec3::new(position.x, -position.y, 1.0) + Vec3::new(-600.0, 400.0, 0.0) - Vec3::new(STEP_SIZE/2.0, STEP_SIZE/2.0, 0.0);
        let a = GizmoStruct::new_float(v.x, v.y, GridType::Tile);
        let b = GizmoStruct::new_float(v.x, v.y, GridType::Horizontal);
        let c = GizmoStruct::new_float(v.x, v.y, GridType::Vertical);
        let d = GizmoStruct::new_float(v.x, v.y, GridType::Cirle);
        // println!("{v:?}, {a:?}");
        for gs in [a,b,c,d].iter(){
            let point = gs.vec();
            gizmos.circle(point, 5.0, PURPLE);
            gizmos.line(v, point, RED);
            gizmos.line(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, point, BLACK);
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

fn on_click_print_hello(click: Trigger<Pointer<Click>>) {
    println!("{} was clicked!", click.entity());
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
}