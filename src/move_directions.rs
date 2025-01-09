use super::*;

/// Represents movement along the x-z axis (== along the ground), but not y axis motion (jumping/falling)
#[derive(PartialEq, Eq)]
pub enum MoveDirections{
    NorthWest,
    North,
    NorthEast,
    West,
    Stationary,
    East,
    SouthWest,
    South,
    SouthEast
}

impl MoveDirections{
    // pub fn new_just_pressed(input: Res<ButtonInput<KeyCode>>)->Self{
    //     [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD].iter().map(|&key| input.just_pressed(key)).collect::<Vec<bool>>().into()
    // }
    pub fn new_pressed(input: &Res<ButtonInput<KeyCode>>)->Self{
        [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD].iter().map(|&key| input.pressed(key)).collect::<Vec<bool>>().into()
    }

    pub fn to_vec3(self)->Vec3{
        const NORTH: Vec3 = Vec3::Z;
        const SOUTH: Vec3 = Vec3::NEG_Z;
        const EAST: Vec3 = Vec3::NEG_X;
        const WEST: Vec3 = Vec3::X;

        let direction = match self{
            MoveDirections::NorthWest   => NORTH + WEST,
            MoveDirections::North       => NORTH,
            MoveDirections::NorthEast   => NORTH + EAST,
            MoveDirections::West        => WEST,
            MoveDirections::East        => EAST,
            MoveDirections::SouthWest   => SOUTH + WEST,
            MoveDirections::South       => SOUTH,
            MoveDirections::SouthEast   => SOUTH + EAST,
            MoveDirections::Stationary  => Vec3::ZERO,
        };
        direction.normalize_or_zero()
        
    }
}
impl From<[bool;4]> for MoveDirections{
    fn from(value: [bool;4]) -> Self {
        // [W,A,S,D]
        match value{
            [false, false, true, false] => Self::South,
            [false, false, true, true] => Self::SouthEast,
            [false, false, false, true] => Self::East,
            [true, false, false, true] => Self::NorthEast,
            [true, false, true, true] => Self::East,
            [true, true, false, false] => Self::NorthWest,
            [false, true, true, false] => Self::SouthWest,
            [false, true, true, true] =>  Self::South,
            [true, true, false, true] => Self::North,
            [true, false, false, false] => Self::North,
            [true, true, true, false] => Self::West,
            [false, true, false, false] => Self::West,
            [false, true, false, true] => Self::Stationary,
            [true, false, true, false] => Self::Stationary,
            [true, true, true, true] => Self::Stationary,
            [false, false, false, false] => Self::Stationary,
        }
    }
}
impl From<Vec<bool>> for MoveDirections{
    fn from(value: Vec<bool>) -> Self {
        let arr: [bool;4] = value.try_into().expect("We expect the Vec<[bool]> to be of size 4, as it represents four cardinal directions.");
        arr.into()
    }
}
