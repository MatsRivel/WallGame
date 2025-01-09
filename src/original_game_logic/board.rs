use std::{collections::HashSet,fmt::{Debug, Display}, ops::{Index, IndexMut}, usize};
use super::game_error::GameError;
use super::orientation::{Cardinality, Orientation};
use super::position::Position;
use super::tile::{SpaceTile, Tile, WallTile};

#[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Player{
    A,
    B
}
pub struct Board<const HEIGHT: usize, const WIDTH: usize>{
    matrix: Vec<Tile>,
    pub a_pos: Position<HEIGHT,WIDTH>,
    pub b_pos: Position<HEIGHT,WIDTH>
}
impl<const HEIGHT: usize, const WIDTH: usize> Board<HEIGHT, WIDTH>{
    pub fn new()->Self{
        debug_assert!(WIDTH  % 2 == 1,"WIDTH  must be an odd number to work properly");
        debug_assert!(HEIGHT % 2 == 1,"HEIGHT must be an odd number to work properly");

        // Initializing with wall-spaces first.
        let matrix = vec![Tile::WallTile(WallTile::Open);HEIGHT*WIDTH];

        // Setting player positions:
        let center_of_row = WIDTH/2;
        let player_a_start = Position::<HEIGHT,WIDTH>::new(0, center_of_row).expect("Starting position of A should be a know valid position");
        let player_b_start = Position::<HEIGHT,WIDTH>::new(HEIGHT-1, center_of_row).expect("Starting position of B should be a know valid position");
        let mut board = Self{matrix, a_pos: player_a_start, b_pos: player_b_start};
        
        // Putting in spaces for players
        for i in (0..HEIGHT).step_by(2){
            for j in (0..WIDTH).step_by(2){
                let position = Position::new(i, j).expect("Known valid position");
                board[position] = Tile::SpaceTile(SpaceTile::Empty);
            }
        }

        // Placing players
        board[player_a_start] = Tile::SpaceTile(SpaceTile::PlayerA);
        board[player_b_start] = Tile::SpaceTile(SpaceTile::PlayerB);
        board
    }

    pub fn print_board(&self){
        for idx in 0..self.matrix.len(){
            let pos = Position::<HEIGHT,WIDTH>::try_from(idx).expect("Known valid index");
            if pos.width() == 0 && idx != 0{
                println!();
            }
            print!("{}",self.matrix[idx]);
        }
        println!();
    }
    fn tile_check(&self, pos: Position::<HEIGHT,WIDTH>)->Result<(),GameError>{
        let tile = self[pos];
        if !tile.is_wall_tile(){
            return Err(GameError::PositionIsNotWall((pos.height(),pos.width())));
        }
        if tile.is_occupied(){
            return Err(GameError::PositionIsBlocked((pos.height(),pos.width())));
        }
        Ok(())
    }
    fn get_valid_wall_positions(&self, pos: Position::<HEIGHT,WIDTH>, orientation: Orientation)->Result<Vec<Position::<HEIGHT,WIDTH>>,GameError>{
        let delta = match orientation{
            Orientation::Horizontal => Cardinality::East.to_delta(),
            Orientation::Vertical   => Cardinality::South.to_delta(),
        };
        let first_wall = pos;
        let second_wall: Result<Position<HEIGHT, WIDTH>, GameError> = pos + delta;
        let last_wall: Result<Position<HEIGHT, WIDTH>, GameError> = pos + delta.doubled();
        let mut positions_in_bounds = [last_wall, second_wall].into_iter().collect::<Result<Vec<Position<HEIGHT,WIDTH>>,GameError>>()?;
        positions_in_bounds.push(first_wall);
        positions_in_bounds.into_iter().map(|wall|{
            match self.tile_check(wall){
                Ok(_) => Ok(wall),
                Err(e) => Err(e),
            }
        }).collect::<Result<Vec<Position::<HEIGHT,WIDTH>>,GameError>>()
    }
    pub fn can_place_wall(&self, pos: Position::<HEIGHT,WIDTH>, orientation: Orientation)->bool{
        self.get_valid_wall_positions(pos,orientation).is_ok()
        
    }
    pub fn place_wall(&mut self, pos: Position::<HEIGHT,WIDTH>, orientation: Orientation)->Result<(),GameError>{
        let valid_positions = self.get_valid_wall_positions(pos,orientation)?;
        valid_positions.into_iter().for_each(|wall| self[wall] = Tile::WallTile(WallTile::Blocked));
        Ok(())
    }
    fn swap_spaces(&mut self, from_pos: Position<HEIGHT,WIDTH>, to_pos: Position<HEIGHT,WIDTH>, player: Option<Player>)->Result<(),GameError>{
        if !self[from_pos].is_space_tile() || !self[to_pos].is_space_tile(){
            return Err(GameError::SwappingDifferentTileTypes(from_pos.as_tuple(),to_pos.as_tuple()))
        }
        let temp = self[from_pos];
        self[from_pos]  = self[to_pos];
        self[to_pos] = temp;
        match player{
            Some(Player::A) => self.a_pos = to_pos,
            Some(Player::B) => self.b_pos = to_pos,
            None => ()
        }
        Ok(())
    }
    pub fn move_player(&mut self, player: Player, cardinality: Cardinality)->Result<(),GameError>{
        let delta = cardinality.to_delta().doubled(); // We double to step over walls
        let wall_check = cardinality.to_delta();
        let (player_pos, player_tile) = match player{
            Player::A => (self.a_pos, Tile::SpaceTile(SpaceTile::PlayerA)),
            Player::B => (self.b_pos, Tile::SpaceTile(SpaceTile::PlayerB)),
        };
        let wall_check_position = (player_pos+wall_check)?;
        debug_assert!(self[wall_check_position].is_wall_tile(), "Taking a single index step in any direction from a player position should allways be a wall tile.");
        let new_position = (player_pos+delta)?;
        
        if self[wall_check_position].is_occupied(){
            return Err(GameError::PositionIsBlocked(wall_check_position.as_tuple()));
        }
        
        if !self[new_position].is_space_tile(){
            return Err(GameError::PositionIsNotSpace(new_position.as_tuple()))
        }
        
        if self[new_position].is_occupied(){
            return Err(GameError::PositionIsBlocked(new_position.as_tuple()))
        }
        self.swap_spaces(player_pos,new_position, Some(player))?;

        Ok(())
    }
    fn pathing_cost_approximation(from:Position::<HEIGHT,WIDTH>,to:Position::<HEIGHT,WIDTH>)->usize{
        let (ax,ay) = from.as_tuple();
        let (bx,by) = to.as_tuple();
        ax.abs_diff(bx) + ay.abs_diff(by)
    }
    pub fn check_for_path(&self, player_pos: Position::<HEIGHT,WIDTH>, player: Player)->bool{
        debug_assert!(self[player_pos].is_space_tile());
        debug_assert!(self[player_pos].is_occupied());
        let row_idx = match player{
            Player::A => HEIGHT-1,
            Player::B => 0,
        };
        let mut seen = HashSet::<Position<HEIGHT, WIDTH>>::new();
        let output= self.path_check(&mut seen, player_pos, row_idx);
        #[cfg(test)]
        {
            for idx in 0..self.matrix.len(){
                let pos = Position::<HEIGHT,WIDTH>::try_from(idx).expect("Known valid index");
                if pos.width() == 0 && idx != 0{
                    println!();
                }
                if seen.contains(&pos){
                    print!("*");

                }else{
                    print!("{}",self.matrix[idx]);

                }
            }
            println!();
        }
        output
    }
    fn path_check(&self, seen: &mut HashSet<Position<HEIGHT, WIDTH>>, current: Position<HEIGHT, WIDTH>, goal_height: usize)->bool{
        use Cardinality::{North,South,East,West};
        #[cfg(test)]
        println!("{current:?}");
        if current.height() == goal_height{
            return true;
        }
        let valid_neighbours = [North, South, East, West].into_iter()
            .map(|dir| (dir.to_delta(),dir.to_delta().doubled()))
            .filter_map(|(wall_check, delta)| {
                match (current + wall_check, current + delta){
                    (Ok(v),Ok(w)) => Some((v,w)),
                    (_,_) => None,
                }

            }).filter_map(|(wall_check,pos)| {
                let no_walls_in_the_way = self[wall_check].is_wall_tile() && !self[wall_check].is_occupied();
                let target_position_is_available = self[pos].is_space_tile() && !self[pos].is_occupied();
                if no_walls_in_the_way && target_position_is_available{
                    Some(pos)
                }else{
                    None
                }
            });
        
        for neighbour in valid_neighbours{
            if seen.contains(&neighbour){
                continue;
            }
            seen.insert(neighbour);
            if self.path_check(seen, neighbour, goal_height){
                return true;
            }
        }
        false

    }   
    pub fn check_for_winner(&self)->Option<Player>{
        for width in 0..WIDTH{
            let pos = Position::new(0, width).unwrap();
            if self[pos] == Tile::SpaceTile(SpaceTile::PlayerB){
                return Some(Player::B);
            }
            let pos = Position::new(HEIGHT-1, width).unwrap();
            if self[pos] == Tile::SpaceTile(SpaceTile::PlayerA){
                return Some(Player::A);
            }
        }
        None
    }
    pub fn get_tile(&self,position:Position::<HEIGHT,WIDTH>)->Tile{
        self[position]
    }
}

impl <const HEIGHT:usize, const WIDTH: usize> Index<Position<HEIGHT,WIDTH>>  for Board<HEIGHT,WIDTH>{
    type Output = Tile;

    fn index(&self, index: Position<HEIGHT,WIDTH>) -> &Self::Output {
        let idx = usize::from(index);
        &self.matrix[idx]
    }
}
impl <const HEIGHT:usize, const WIDTH: usize> IndexMut<Position<HEIGHT,WIDTH>> for Board<HEIGHT,WIDTH>{
    fn index_mut(&mut self, index: Position<HEIGHT,WIDTH>) -> &mut Self::Output {
        let idx = usize::from(index);
        &mut self.matrix[idx]
    }
}

#[cfg(test)]
mod tests{
    use std::collections::HashSet;
    use super::{Board, Position, SpaceTile, Tile, Orientation, Player, Cardinality};
    mod win_condition{
        use super::*;
        const HEIGHT: usize = 5;
        const WIDTH: usize = 5;
        #[test]
        fn check_for_winner(){
            let mut board = Board::<HEIGHT,WIDTH>::new();
            let target_pos = Position::new(HEIGHT-1, 0).unwrap();
            board.swap_spaces(board.a_pos, target_pos, Some(Player::A)).unwrap();
            assert_eq!(board.check_for_winner(),Some(Player::A))
        }
        #[test]
        fn check_for_no_winner(){
            let board = Board::<HEIGHT,WIDTH>::new();
            assert_eq!(board.check_for_winner(), None)
        }
    }
    mod path_finding{
        const HEIGHT: usize = 9;
        const WIDTH: usize = 9;
        use super::*;
        #[test]
        fn finds_existing_path(){
            let mut board = Board::<HEIGHT,WIDTH>::new();
            board.place_wall(Position::new(0, 3).unwrap(), Orientation::Vertical).unwrap();
            board.place_wall(Position::new(1, 4).unwrap(), Orientation::Horizontal).unwrap();
            board.place_wall(Position::new(3, 6).unwrap(), Orientation::Horizontal).unwrap();
            board.place_wall(Position::new(5, 4).unwrap(), Orientation::Horizontal).unwrap();
            board.place_wall(Position::new(5, 0).unwrap(), Orientation::Horizontal).unwrap();
            board.place_wall(Position::new(5, 7).unwrap(), Orientation::Vertical).unwrap();
            println!("\n_________");
            board.print_board();
            assert!(board.check_for_path(board.a_pos, Player::A));


        }
        #[test]
        fn finds_no_path_when_no_path_exists(){
            let mut board = Board::<HEIGHT,WIDTH>::new();
            board.place_wall(Position::new(0, 3).unwrap(), Orientation::Vertical).unwrap();
            board.place_wall(Position::new(1, 4).unwrap(), Orientation::Horizontal).unwrap();
            board.place_wall(Position::new(3, 6).unwrap(), Orientation::Horizontal).unwrap();
            board.place_wall(Position::new(5, 4).unwrap(), Orientation::Horizontal).unwrap();
            board.place_wall(Position::new(5, 0).unwrap(), Orientation::Horizontal).unwrap();
            board.place_wall(Position::new(5, 7).unwrap(), Orientation::Vertical).unwrap();
            board.place_wall(Position::new(3, 3).unwrap(), Orientation::Horizontal).unwrap();
            println!("\n_________");
            board.print_board();
            assert!(!board.check_for_path(board.a_pos, Player::A));

        }
    }
    mod player_movements{
        const HEIGHT: usize = 5;
        const WIDTH: usize = 5;
        use super::*;
        #[test]
        fn player_moves(){
            let mut board = Board::<HEIGHT,WIDTH>::new();
            board.move_player(Player::A, Cardinality::West).unwrap();
            let expected_position = Position::<HEIGHT,WIDTH>::new(0,0).unwrap();
            assert_eq!(board.a_pos,expected_position);
            assert_eq!(board[expected_position],Tile::SpaceTile(SpaceTile::PlayerA));

            board.move_player(Player::A, Cardinality::South).unwrap();
            let expected_position = Position::<HEIGHT,WIDTH>::new(2,0).unwrap();
            assert_eq!(board.a_pos, expected_position);
            assert_eq!(board[expected_position],Tile::SpaceTile(SpaceTile::PlayerA));

            board.move_player(Player::A, Cardinality::East).unwrap();
            let expected_position = Position::<HEIGHT,WIDTH>::new(2,2).unwrap();
            assert_eq!(board.a_pos, expected_position);
            assert_eq!(board[expected_position],Tile::SpaceTile(SpaceTile::PlayerA));

            board.move_player(Player::A, Cardinality::North).unwrap();
            let expected_position = Position::<HEIGHT,WIDTH>::new(0,2).unwrap();
            assert_eq!(board.a_pos, expected_position);
            assert_eq!(board[expected_position],Tile::SpaceTile(SpaceTile::PlayerA));
        }
    }
    mod wall_placement_tests{
        const HEIGHT: usize = 5;
        const WIDTH: usize = 5;
        use super::*;
        #[test]
        fn all_horizontal_valid_placements_work(){
            for height in (1..HEIGHT).step_by(2){
                for width in 0..(WIDTH-2){
                    let pos = Position::<HEIGHT,WIDTH>::new(height, width).unwrap();
                    let mut board = Board::<HEIGHT,WIDTH>::new();
                    let placement_result = board.place_wall(pos, Orientation::Horizontal);
                    // board.print_board();
                    // println!("{pos:?}\n_______");
                    assert_eq!(placement_result, Ok(()))
                }
            }
        }
        #[test]
        fn all_vertical_valid_placements_work(){
            for height in 0..(HEIGHT-2){
                for width in (1..WIDTH).step_by(2){
                    let pos = Position::<HEIGHT,WIDTH>::new(height, width).unwrap();
                    let mut board = Board::<HEIGHT,WIDTH>::new();
                    let placement_result = board.place_wall(pos, Orientation::Vertical);
                    assert_eq!(placement_result, Ok(()))
                }
            }
        }
        #[test]
        fn walls_can_not_be_placed_on_spaces(){
            for i in 1..(HEIGHT-1){
                for j in (0..WIDTH).step_by(2){
                    let pos = Position::<HEIGHT,WIDTH>::new(i, j).unwrap();
                    let mut board = Board::<HEIGHT,WIDTH>::new();
                    let placement_result = board.place_wall(pos, Orientation::Vertical);
                    assert_ne!(placement_result,Ok(()))
                }
            }
            for i in (0..HEIGHT).step_by(2){
                for j in 1..(WIDTH-1){
                    let pos = Position::<HEIGHT,WIDTH>::new(i, j).unwrap();
                    let mut board = Board::<HEIGHT,WIDTH>::new();
                    let placement_result = board.place_wall(pos, Orientation::Horizontal);
                    assert_ne!(placement_result,Ok(()))
                }
            }
        }
        #[test]
        fn walls_are_blocked_by_players(){
            let mut board = Board::<HEIGHT,WIDTH>::new();
            let a_pos = Position::<HEIGHT,WIDTH>::new(0, 2).unwrap();
            let b_pos = Position::<HEIGHT,WIDTH>::new(HEIGHT-1, 2).unwrap();
            let placement_result_a = board.place_wall(a_pos.step_left().unwrap(), Orientation::Horizontal);
            let placement_result_b = board.place_wall(b_pos.step_up().unwrap(), Orientation::Vertical);
            board.print_board();println!("____");
            assert_ne!(placement_result_a, Ok(()));
            assert_ne!(placement_result_b, Ok(()));
        }
    }
    mod basic_tests{
        const HEIGHT: usize = 5;
        const WIDTH: usize = 5;
        use super::*;
        #[test]
        fn valid_positions_are_constructable(){
            const HEIGHT: usize = 5;
            const WIDTH: usize = 5;
            let board = Board::<HEIGHT, WIDTH>::new();
            for i in 0..9usize{
                let pos = Position::<HEIGHT,WIDTH>::try_from(i);
                assert!(pos.is_ok());
                let pos = pos.unwrap();
                let _indexing = board[pos]; 
                assert!(true,"If this assert is reached, no indexings paniced.")
            }
        }
        
        #[test]
        fn invalid_positions_are_not_constructable(){
            const HEIGHT: usize = 3;
            const WIDTH: usize = 3;
            for i in 9..100usize{
                let pos = Position::<HEIGHT,WIDTH>::try_from(i);
                assert!(pos.is_err(),"index {i} lead to the position {:?}, which should not work in a {HEIGHT}x{WIDTH} matrix.",pos.unwrap());
            }
        }
        
        #[test]
        fn players_occur_in_expected_positions(){
            let board = Board::<HEIGHT,WIDTH>::new();
            let a_pos = Position::<HEIGHT,WIDTH>::new(0, 2).unwrap();
            assert_eq!(board[a_pos], Tile::SpaceTile(SpaceTile::PlayerA));
            let b_pos = Position::<HEIGHT,WIDTH>::new(HEIGHT-1, 2).unwrap();
            assert_eq!(board[b_pos], Tile::SpaceTile(SpaceTile::PlayerB));
            
        }
        
        #[test]
        fn only_one_instance_of_each_player_exists(){
            let board = Board::<HEIGHT,WIDTH>::new();
            let a_count = board.matrix.iter().filter(|&&tile| tile == Tile::SpaceTile(SpaceTile::PlayerA) ).count();
            assert_eq!(a_count,1);
            let b_count = board.matrix.iter().filter(|&&tile| tile == Tile::SpaceTile(SpaceTile::PlayerB) ).count();
            assert_eq!(b_count,1);
        }

        #[test]
        fn every_idx_maps_to_exactly_one_position(){
        const HEIGHT: usize = 1000;
        const WIDTH: usize = 100;
        let mut seen = HashSet::new();
        for index in 0..(HEIGHT*WIDTH){
            let pos = Position::<HEIGHT,WIDTH>::try_from(index).unwrap();
            assert!(! seen.contains(&pos), "{pos:?} has been seen, but occurred again at idx=={index}");
            seen.insert(pos);
        }
    }
}

}