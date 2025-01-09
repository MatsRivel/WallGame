use game_error::GameError;

use super::*;
#[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Tile{
    SpaceTile(SpaceTile),
    WallTile(WallTile)
}
impl Tile{
    pub fn is_wall_tile(&self)->bool{
        match self{
            Tile::SpaceTile(_) => false,
            Tile::WallTile(_) => true,
        }
    }
    pub fn is_space_tile(&self)->bool{
        match self{
            Tile::SpaceTile(_) => true,
            Tile::WallTile(_) => false,
        }
    }
    pub fn is_occupied(&self)->bool{
        match self{
            Tile::SpaceTile(space_tile) => {
                match space_tile{
                    SpaceTile::PlayerA | SpaceTile::PlayerB => true,
                    SpaceTile::Empty => false,
                }
            },
            Tile::WallTile(wall_tile) => {
                match wall_tile{
                    WallTile::Open => false,
                    WallTile::Blocked => true,
                }
            },
        }
    }
}
#[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum SpaceTile{
    PlayerA,
    PlayerB,
    Empty
}
impl SpaceTile{
    pub fn to_char(&self)->char{
        match self{
            SpaceTile::PlayerA => 'A',
            SpaceTile::PlayerB => 'B',
            SpaceTile::Empty => '.',
        }
    }
}
#[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum WallTile{
    Open,
    Blocked
}
impl WallTile{
    pub fn to_char(&self)->char{
        match self{
            WallTile::Open => ' ',
            WallTile::Blocked => 'X',
        }
    }
}
impl Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        let c = char::from(*self);
        write!(f,"{c}")
    }
}
impl TryFrom<char> for Tile{
    type Error = GameError;
    
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use SpaceTile::*;
        use WallTile::*;
        // Doing it this way keeps ONE source of truth, 
        // therefor not causing errors if we decide to change the characters that corresponds to which tiles.
        if PlayerA.to_char() == value{
            Ok(Tile::SpaceTile(SpaceTile::PlayerA))
        }
        else if PlayerB.to_char() == value{
            Ok(Tile::SpaceTile(SpaceTile::PlayerB))
        }
        else if Blocked.to_char() == value{
            Ok(Tile::WallTile(WallTile::Blocked))
        }
        else if Open.to_char() == value{
            Ok(Tile::WallTile(WallTile::Open))
        }
        else if Empty.to_char() == value{
            Ok(Tile::SpaceTile(SpaceTile::Empty))
        }
        else{ Err(GameError::InvalidTileChar(value))}
    }
}

impl From<Tile> for char{
fn from(value: Tile) -> Self {
    match value{
        Tile::SpaceTile(space_tile) => {
            space_tile.to_char()
        },
        Tile::WallTile(wall_tile) => {
            wall_tile.to_char()
        },
    }
}
}
