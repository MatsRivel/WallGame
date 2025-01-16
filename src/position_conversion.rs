use super::*;
pub type Pos = [usize;2];
fn pos_to_vec_component(x: usize, modifier: f32)->f32{
    let x = x as i32 - (N_TILES/2);
    let x = x as f32 * STEP_SIZE;
    let x = x + TILE_WIDTH/2f32 + modifier;
    x as f32
}

pub fn pos_to_vec3(pos: Pos, xmod: f32, ymod: f32)->Vec3{
    let x = pos_to_vec_component(pos[0],xmod);
    let y = pos_to_vec_component(pos[1],ymod);
    let pos = Vec3::new( x, y,0.0);
    pos
}

fn vec_to_pos_component(x: f32, modifier: f32)->usize{
    let x = x - TILE_WIDTH/2f32 - modifier ;
    let x = x / STEP_SIZE;
    let x = (x as i32) + (N_TILES/2) + (1-N_TILES%2);
    x as usize
}

pub fn vec3_to_pos(v: Vec3, xmod: f32, ymod: f32)->Pos{
    let x = vec_to_pos_component(v.x, xmod);
    let y = vec_to_pos_component(v.y, ymod);
    [x as usize, y as usize]
}

pub fn tile_to_pos(v: Vec3)->Pos{
    let xmod = - TILE_WIDTH/2f32;
    let ymod = - TILE_WIDTH/2f32;
    vec3_to_pos(v, xmod, ymod)
}
pub fn wall_horizontal_to_pos(v: Vec3)->Pos{
    let xmod = - TILE_WIDTH/2f32;
    let ymod = TRENCH_WIDTH/2f32;
    vec3_to_pos(v, xmod, ymod)
}
pub fn wall_vertical_to_pos(v: Vec3)->Pos{
    let xmod = TILE_WIDTH/2f32;
    let ymod = -TRENCH_WIDTH/2f32;
    vec3_to_pos(v, xmod, ymod)
}
pub fn wall_circle_to_pos(v: Vec3)->Pos{
    let xmod = TRENCH_WIDTH/2f32;
    let ymod = TRENCH_WIDTH/2f32;
    vec3_to_pos(v, xmod, ymod)
}

pub fn pos_to_tile(pos: Pos)->Vec3{
    let xmod = - TILE_WIDTH/2f32;
    let ymod = - TILE_WIDTH/2f32;
    pos_to_vec3(pos, xmod, ymod)
}

pub fn pos_to_wall_horizontal(pos: Pos)->Vec3{
    let xmod = - TILE_WIDTH/2f32;
    let ymod = TRENCH_WIDTH/2f32;
    pos_to_vec3(pos, xmod, ymod)
}

pub fn pos_to_wall_vertical(pos: Pos)->Vec3{
    let xmod = TILE_WIDTH/2f32;
    let ymod = -TRENCH_WIDTH/2f32;
    pos_to_vec3(pos, xmod, ymod)
}

pub fn pos_to_wall_cirlce(pos: Pos)->Vec3{
    let xmod = TRENCH_WIDTH/2f32;
    let ymod = TRENCH_WIDTH/2f32;
    pos_to_vec3(pos, xmod, ymod)
}

#[cfg(test)]
mod pos_tests{
    use super::*;
    const X: f32 = -932f32;
    const Y: f32 = 733f32;
    #[test]
    fn converstion_test_horizontal(){
        let v = Vec3::new(X, Y, 0.0);
        let a = wall_horizontal_to_pos(v);
        let b = pos_to_wall_horizontal(a);
        let c = wall_horizontal_to_pos(b);
        assert_eq!(a,c,"{v:?}, {a:?}, {b:?}, {c:?}")
    }
    #[test]
    fn converstion_test_vertical(){
        let v = Vec3::new(X, Y, 0.0);
        let a = wall_vertical_to_pos(v);
        let b = pos_to_wall_vertical(a);
        let c = wall_vertical_to_pos(b);
        assert_eq!(a,c,"{v:?}, {a:?}, {b:?}, {c:?}")
    }
    #[test]
    fn converstion_test_circle(){
        let v = Vec3::new(X, Y, 0.0);
        let a = wall_circle_to_pos(v);
        let b = pos_to_wall_cirlce(a);
        let c = wall_circle_to_pos(b);
        assert_eq!(a,c,"{v:?}, {a:?}, {b:?}, {c:?}")
    }
    #[test]
    fn converstion_test_tile(){
        let v = Vec3::new(X, Y, 0.0);
        let a = tile_to_pos(v);
        let b = pos_to_tile(a);
        let c = tile_to_pos(b);
        assert_eq!(a,c,"{v:?}, {a:?}, {b:?}, {c:?}")
    }
    #[test]
    fn converstion_test_tile_zero(){
        let v = Vec3::new(0.0, 0.0, 0.0);
        let a = tile_to_pos(v);
        let b = pos_to_tile(a);
        let c = tile_to_pos(b);
        assert_eq!(a,c,"{v:?}, {a:?}, {b:?}, {c:?}");
        assert_eq!(b, Vec3::ZERO);
    }
}
