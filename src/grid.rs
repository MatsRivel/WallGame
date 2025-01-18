use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum GridType {
    Tile,
    Circle,
    Horizontal,
    Vertical,
}
impl GridType {
    pub fn all() -> Vec<Self> {
        vec![Self::Tile, Self::Circle, Self::Horizontal, Self::Vertical]
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    A,
    B,
}
