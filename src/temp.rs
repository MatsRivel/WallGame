fn move_cursor_debug(
    mut cursor: Query<&mut Transform, With<MouseIdentifier>>, 
    camera_query: Query<(&Camera,&GlobalTransform)>,
    mouse_movement:Query<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
    board_query: Query<&Game>){
    let window = mouse_movement.single();
    let (camera, camera_transform) = camera_query.single();
    if let Some(window_cursor_position) = window.cursor_position(){
        if let Ok(position) = camera.viewport_to_world_2d(camera_transform, window_cursor_position){
            // let corrected_position = vec3(position.x -window.width()/2.0,   window.height()/2.0 - position.y,2.0);
            let corrected_position = Vec3::new(position.x, position.y, 2.0);
            cursor.single_mut().translation = corrected_position;
            if let Some(p) = vec3_to_position_adjusted(corrected_position){
                let board = board_query.single();
                let tile = board.get_tile(p);
                if !tile.is_wall_tile(){
                    return;
                } 
                for orientation in [Orientation::Horizontal, Orientation::Vertical]{
                    if board.can_place_wall(p, orientation){
                        let (length, adjustment) = match orientation{
                            Orientation::Horizontal => (vec3(WALL_LENGTH, 0.0, 0.0),vec3(TILE_WIDTH/2.0, 0.0, 0.0)),
                            Orientation::Vertical => (vec3(0.0, WALL_LENGTH, 0.0),vec3(0.0,TILE_WIDTH,  0.0))
                        };
                        let screen_position = position_to_vec3_adjusted(p) - adjustment;
                        gizmos.line(screen_position, screen_position + length, RED);
                    }
                }
            }
        };
    }
}

fn initial_tile_positions_v2()->Vec<Vec3>{
    (0..=DOUBLE_N_TILES)
        .step_by(2)
        .flat_map(|i| {
            (0..=DOUBLE_N_TILES)
                .step_by(2)
                .filter_map(move |j| {
                    Position::new(i, j)
                })
            }).map(|p| position_to_vec3_adjusted(p))
            .collect()
}

pub fn position_to_vec3_adjusted(pos: Position::<DOUBLE_N_TILES,DOUBLE_N_TILES>)->Vec3{
    let vec = position_to_vec3(pos);
    vec - ADJUSTER
    
}
pub fn position_to_vec3(pos: Position::<DOUBLE_N_TILES,DOUBLE_N_TILES>)->Vec3{
    let x = position_component_to_vec3_component(pos.width());
    let y = position_component_to_vec3_component(pos.height());
    vec3(x, y, 0.0)
}
fn position_component_to_vec3_component(x:usize)->f32{
    let a = ((x+1)/2) as f32;
    if x % 2 == 0{
        a * STEP_SIZE + (TILE_WIDTH / 2f32)
    }else{
        a * STEP_SIZE - (TRENCH_WIDTH / 2f32)
    }
}

fn vec3_component_to_position_component(x:f32)->usize{
    let a1 = (x*2f32) / STEP_SIZE;
    a1 as usize
}
pub fn vec3_to_position_adjusted(v: Vec3)->Option<Position::<DOUBLE_N_TILES,DOUBLE_N_TILES>>{
    vec3_to_position(v + ADJUSTER)
}

pub fn vec3_to_position(v: Vec3)->Option<Position::<DOUBLE_N_TILES,DOUBLE_N_TILES>>{
    let x =  v.x;
    let y =  v.y;
    let width = vec3_component_to_position_component(x);
    let height = vec3_component_to_position_component(y);
    Position::new(height, width)
}
#[cfg(test)]
mod main_tests{
    use bevy::math::Vec3;

    use crate::{original_game_logic::position::Position, position_to_vec3, vec3_to_position, DOUBLE_N_TILES, TILE_WIDTH, TRENCH_WIDTH};
    #[test]
    fn pos_to_vec3_conversion0(){
        const N: usize = DOUBLE_N_TILES;
        let pos = Position::<N,N>::new(0, 0).unwrap();
        let vec = position_to_vec3(pos);
        let point = 0f32*TILE_WIDTH + TILE_WIDTH /2f32;
        assert_eq!(point,32f32);
        assert_eq!(vec, Vec3::new(point, point, 0.0))
    }
    #[test]
    fn pos_to_vec3_conversion1(){
        const N: usize = DOUBLE_N_TILES;
        let pos = Position::<N,N>::new(1, 1).unwrap();
        let vec = position_to_vec3(pos);
        let point = TILE_WIDTH + (TRENCH_WIDTH / 2f32);
        assert_eq!(point,64f32 + 4f32);
        assert_eq!(vec, Vec3::new(point, point, 0.0))
    }

    #[test]
    fn pos_to_vec3_conversion2(){
        const N: usize = DOUBLE_N_TILES;
        let pos = Position::<N,N>::new(2, 2).unwrap();
        let vec = position_to_vec3(pos);
        let point = TILE_WIDTH + TRENCH_WIDTH + (TILE_WIDTH / 2f32);
        assert_eq!(point,64f32 + 8f32 + 32f32);
        assert_eq!(vec, Vec3::new(point, point, 0.0))
    }
    #[test]
    fn pos_to_vec3_conversion3(){
        const N: usize = DOUBLE_N_TILES;
        let pos = Position::<N,N>::new(3, 3).unwrap();
        let vec = position_to_vec3(pos);
        let point = TILE_WIDTH + TRENCH_WIDTH + TILE_WIDTH + (TRENCH_WIDTH / 2f32);
        assert_eq!(point,64f32 + 8f32 + 64f32 + 4f32);
        assert_eq!(vec, Vec3::new(point, point, 0.0))
    }

    #[test]
    fn vec3_to_pos_conversion1(){
        const N: usize = DOUBLE_N_TILES;
        // All values here are in the ranges [+- TILE_WIDT / 2]
        // So they should all end up pointing at Pos(0,0)
        let half_tile = TILE_WIDTH / 2f32;
        let a = Vec3::new(-half_tile, -half_tile, 0.0);
        let b = Vec3::new(half_tile, half_tile, 0.0);
        let c = Vec3::new(-half_tile, half_tile, 0.0);
        let d = Vec3::new(half_tile, -half_tile, 0.0);
        let expected = Position::<N,N>::new(0, 0);
        for v in [a,b,c,d]{
            let actual = vec3_to_position(v);
            assert_eq!(actual,expected,"{v:?}")
        }
    }
    #[test]
    fn vec3_to_pos_conversion2(){
        const N: usize = DOUBLE_N_TILES;
        // All values here are in the ranges [TILE_WIDT -> TILE_WIDTH + TRENCH_WIDTH]
        // So they should all end up pointing at Pos(0,0)
        let center_trench = TILE_WIDTH + (TRENCH_WIDTH / 2f32);
        let a = Vec3::new(center_trench - 1f32, center_trench + 1f32, 0.0);
        let b = Vec3::new(center_trench - 1f32, center_trench - 1f32, 0.0);
        let c = Vec3::new(center_trench + 1f32, center_trench - 1f32, 0.0);
        let d = Vec3::new(center_trench + 1f32, center_trench + 1f32, 0.0);
        let expected = Position::<N,N>::new(1, 1);
        for v in [a,b,c,d]{
            let actual = vec3_to_position(v);
            assert_eq!(actual,expected,"{v:?}")
        }
    }

}

#[derive(Component)]
pub struct Game{
    board: Board::<DOUBLE_N_TILES,DOUBLE_N_TILES>
}
impl Game{
pub fn new()->Self{
    let board = Board::new();
    Self { board }
}
pub fn place_wall(&mut self, pos: Position::<DOUBLE_N_TILES,DOUBLE_N_TILES>, orientation: Orientation)->Result<(),GameError>{
    self.board.place_wall(pos, orientation)
}

pub fn move_player(&mut self, player: Player, cardinality: Cardinality)->Result<(),GameError>{
    self.board.move_player(player, cardinality)
}

pub fn check_for_path(&self, player_pos: Position::<DOUBLE_N_TILES,DOUBLE_N_TILES>, player: Player)->bool{
    self.board.check_for_path(player_pos, player)
}

pub fn check_for_winner(&self)->Option<Player>{
    self.board.check_for_winner()
}
pub fn get_tile(&self,position: Position::<DOUBLE_N_TILES,DOUBLE_N_TILES>)->Tile{
    self.board.get_tile(position)
}
pub fn can_place_wall(&self, position:Position::<DOUBLE_N_TILES,DOUBLE_N_TILES>, orientation: Orientation)->bool{
    self.board.can_place_wall(position, orientation)
}

}
