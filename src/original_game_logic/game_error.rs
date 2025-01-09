use super::*;
use super::position::PositionDelta;

#[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum GameError{
    InvalidTileChar(char),
    IndexToPositionError(usize),
    PositionIsBlocked((usize,usize)),
    PositionIsNotWall((usize,usize)),
    PositionIsNotSpace((usize,usize)),
    SpaceIsOutOfReach((usize,usize)),
    TriedToGoOutOfBounds((usize,usize),PositionDelta),
    PositionUnderflow((usize,usize),(i64,i64)),
    PositionOverflow((usize,usize),(i64,i64)),
    SwappingDifferentTileTypes((usize,usize),(usize,usize)),

}
impl Error for GameError{}
impl Display for GameError{
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use GameError::*;
    match self{
        InvalidTileChar(c) => write!(f,"Char '{c}' does not correspond to an existing tile type."),
        IndexToPositionError(idx) => write!(f,"Index '{idx}' does not fit inside the matrix"),
        PositionIsBlocked((height,width)) => write!(f,"Position ({height},{width}) is occupied."),
        PositionIsNotWall((height,width)) => write!(f,"Position ({height},{width}) was expected to be a an open wall tile, but is a space tile."),
        PositionIsNotSpace((height,width)) => write!(f,"Position ({height},{width}) was expected to be a an open space tile, but is a wall tile."),
        SpaceIsOutOfReach((height,width)) => write!(f,"Position ({height},{width}) is too far away."),
        TriedToGoOutOfBounds((height,width), position_delta) => write!(f,"Position ({height},{width}) + {position_delta} is out of boumds"),
        PositionUnderflow((height,width), (dheight,dwidth)) => write!(f,"Position ({height},{width}) + ({dheight},{dwidth}) is out of boumds"),
        PositionOverflow((height,width), (dheight,dwidth)) => write!(f,"Position ({height},{width}) + ({dheight},{dwidth}) is out of boumds"),
        SwappingDifferentTileTypes((a_height,a_width), (b_height,b_width)) => write!(f,"Position ({a_height},{a_width}) is not the same type of tile as position ({b_height},{b_width})"),
    }
}
}
