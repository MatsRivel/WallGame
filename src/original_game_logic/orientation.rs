use super::position::PositionDelta;

#[derive(Debug,Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Orientation{
    Horizontal,
    Vertical
}
pub enum Cardinality{
    North,East,South,West
}
impl Cardinality{
    pub fn to_delta(&self)->PositionDelta{
        match self{
            Cardinality::North => PositionDelta::new(-1, 0),
            Cardinality::East  => PositionDelta::new(0, 1),
            Cardinality::South => PositionDelta::new(1, 0),
            Cardinality::West  => PositionDelta::new(0, -1),
        }
    }
}
