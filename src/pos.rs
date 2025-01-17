use super::*;

#[derive(Debug,Component,Clone,Copy)]
pub struct GridPosition{
    x: usize,
    y: usize,
}
impl GridPosition{
    fn min_pos()->f32{
        ((N_TILES-1) as f32 * STEP_SIZE) / 2f32
    }
    pub fn new(x:usize, y:usize)->Self{
        let x = (N_TILES as usize).min(x);
        let y = (N_TILES as usize).min(y);
        Self{x,y}
    }
    fn to_usize(v: f32)->usize{
        // let v = v - Self::min_pos();
        let v = v / STEP_SIZE;
        let v = 0f32.max(v + Self::min_pos());
        v as usize
    }
    fn to_float(v: usize)->f32{
        let v = v as f32* STEP_SIZE;
        let v = v - Self::min_pos();
        v
    }
}
impl From<Vec3> for GridPosition{
    fn from(value: Vec3) -> Self {
        let x = Self::to_usize(value.x);
        let y = Self::to_usize(value.y);
        println!("GridPosition <-- Vec3: ({x}, {y}) <-- {value:?}");
        Self::new(x,y)
    }
}
impl From<GridPosition> for Vec3{
    fn from(value: GridPosition) -> Self {
        let x = GridPosition::to_float(value.x);
        let y = GridPosition::to_float(value.y);
        println!("Vec3 <-- GridPositon: ({x}, {y}) <-- {value:?}");
        Vec3::new(x, y, 0.0)
    }
}