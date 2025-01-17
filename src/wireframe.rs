use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct CircleGizmo{
    radius: f32,
    color: Color
}
impl CircleGizmo{
    fn new(radius: f32, color: Color)->Self{
        Self{radius,color}
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SquareGizmo{
    size: Vec2,
    color: Color
}
impl SquareGizmo{
    fn new(size: Vec2, color: Color)->Self{
        Self{size,color}
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum WireFrameGizmo{
    Circle(CircleGizmo),
    Square(SquareGizmo)
}
impl From<SquareGizmo> for WireFrameGizmo{
    fn from(value: SquareGizmo) -> Self {
        Self::Square(value)
    }
}
impl From<CircleGizmo> for WireFrameGizmo{
    fn from(value: CircleGizmo) -> Self {
        Self::Circle(value)
    }
}

impl WireFrameGizmo{
    pub fn draw(&self, point: Vec3, gizmos: &mut Gizmos){
        match self{
            WireFrameGizmo::Circle(circle_gizmo) => {
                gizmos.circle(point, circle_gizmo.radius, circle_gizmo.color);
            },
            WireFrameGizmo::Square(square_gizmo) => {
                gizmos.rect(point, square_gizmo.size, square_gizmo.color);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component)]
pub struct WireFrame{
    frame: WireFrameGizmo
}
impl WireFrame{
    // pub fn new(frame: WireFrameGizmo)->Self{
    //     Self{frame}
    // }
    pub fn new_circle(radius: f32, color: Color)->Self{
        let frame = CircleGizmo::new(radius, color).into();
        Self{frame}
    }
    pub fn new_square(size: Vec2, color: Color)->Self{
        let frame = SquareGizmo::new(size, color).into();
        Self{frame}
    }
    pub fn draw(&self, point: Vec3, gizmos: &mut Gizmos){
        self.frame.draw(point, gizmos);
    }
}