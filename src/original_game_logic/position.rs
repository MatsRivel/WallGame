use std::ops::{Add, Sub};

use game_error::GameError;

use super::*;
#[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct PositionDelta{
    pub delta_height: i32,
    pub delta_width: i32
}
impl PositionDelta{
    pub fn new(height: i32, width: i32)->Self{
        Self{delta_height:height,delta_width:width}
    }
    pub fn doubled(self)->Self{
        Self::new(self.delta_height*2, self.delta_width*2)
    }
    pub fn tripled(self)->Self{
        Self::new(self.delta_height*2, self.delta_width*3)
    }
}
impl Display for PositionDelta{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.delta_height, self.delta_width)
    }
}
#[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Position<const MAX_HEIGHT: usize, const MAX_WIDTH: usize>{
    height: usize,
    width: usize
}
impl <const MAX_HEIGHT: usize, const MAX_WIDTH: usize>Position<MAX_HEIGHT,MAX_WIDTH>{
    pub fn new(height: usize, width: usize)->Option<Self>{
        if height >= MAX_HEIGHT || width >= MAX_WIDTH{
            None
        }else{
            Some(Self{height,width})
        }
    }
    pub fn width(&self)->usize{
        self.width
    }
    pub fn height(&self)->usize{
        self.height
    }
    pub fn step_right(&self)->Option<Self>{
        Position::new(self.height(), self.width()+1)
    }
    pub fn step_down(&self)->Option<Self>{
        Position::new(self.height()+1, self.width())
    }
    pub fn step_left(&self)->Option<Self>{
        if self.width() == 0{
            return None;
        }
        Position::new(self.height(), self.width()-1)
    }
    pub fn step_up(&self)->Option<Self>{
        if self.height() == 0{
            return None;
        }
        Position::new(self.height()-1, self.width())
    }
    pub fn as_tuple(&self)->(usize,usize){
        (self.height,self.width)
    }
}

impl <const MAX_HEIGHT: usize, const MAX_WIDTH: usize> TryFrom<usize> for Position<MAX_HEIGHT,MAX_WIDTH>{
    type Error = GameError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let height = value / MAX_WIDTH;
        let width  = value % MAX_WIDTH;
        if let Some(pos) = Self::new(height, width){
            Ok(pos)
        }else{
            Err(GameError::IndexToPositionError(value))
        }
    }
}
impl <const MAX_HEIGHT: usize, const MAX_WIDTH: usize> From<Position<MAX_HEIGHT,MAX_WIDTH>> for usize{
    fn from(value: Position<MAX_HEIGHT,MAX_WIDTH>) -> Self {
        (value.height*MAX_WIDTH) + value.width
    }
}
impl <const MAX_HEIGHT: usize, const MAX_WIDTH: usize> Sub<PositionDelta> for Position<MAX_HEIGHT,MAX_WIDTH>{
    type Output = Result<Position::<MAX_HEIGHT,MAX_WIDTH>, GameError>;

    fn sub(self, rhs: PositionDelta) -> Self::Output {
         // Todo: This casting might be inapropriate
        if (self.height as i64 - rhs.delta_height as i64) >= 0 || (self.width as i64 - rhs.delta_width as i64) >= 0{
            let height = self.height as i64 - rhs.delta_height as i64;
            let width = self.width as i64 - rhs.delta_width as i64;
            if let Some(pos) = Position::new(height as usize, width as usize){
                return Ok(pos)
            }
        }
        Err(GameError::TriedToGoOutOfBounds((self.height,self.width), rhs))
    }
}
impl <const MAX_HEIGHT: usize, const MAX_WIDTH: usize> Add<PositionDelta> for Position<MAX_HEIGHT,MAX_WIDTH>{
    type Output = Result<Position::<MAX_HEIGHT,MAX_WIDTH>,GameError>;

    fn add(self, rhs: PositionDelta) -> Self::Output {
        // Todo: This casting might be inapropriate
        if (self.height as i64 + rhs.delta_height as i64) >= 0|| (self.width as i64 + rhs.delta_width as i64) >= 0{
            let height = self.height as i64 + rhs.delta_height as i64;
            let width = self.width as i64 + rhs.delta_width as i64;
            if let Some(pos) = Position::new(height as usize, width as usize){
                return Ok(pos)
            }
        }
        Err(GameError::TriedToGoOutOfBounds((self.height,self.width), rhs))
    }
}
impl <const MAX_HEIGHT: usize, const MAX_WIDTH: usize> Display for Position<MAX_HEIGHT,MAX_WIDTH>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"({},{})",self.height,self.width)
    }
}
