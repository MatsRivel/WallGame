use super::*;
#[derive(Debug, Clone, Copy, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CircleGizmo {
    radius: f32,
    color: Color,
    orientation: Orientation,
}
impl CircleGizmo {
    fn new(radius: f32, color: Color) -> Self {
        Self {
            radius,
            color,
            orientation: Orientation::Horizontal,
        }
    }
}
impl Default for CircleGizmo {
    fn default() -> Self {
        Self {
            radius: 10.0,
            color: Color::Srgba(Srgba::new(1.0, 0.0, 0.0, 1.0)),
            orientation: Orientation::Horizontal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SquareGizmo {
    size: Vec2,
    color: Color,
    orientation: Orientation,
}
impl SquareGizmo {
    fn new(size: Vec2, color: Color) -> Self {
        Self {
            size,
            color,
            orientation: Orientation::Horizontal,
        }
    }
    fn rotate(&mut self) {
        println!("ROTATE!");
        match self.orientation {
            Orientation::Horizontal => {
                self.orientation = Orientation::Vertical;
                let temp = self.size.x;
                self.size.x = self.size.y;
                self.size.y = temp;
            }
            Orientation::Vertical => {
                self.orientation = Orientation::Horizontal;
                let temp = self.size.x;
                self.size.x = self.size.y;
                self.size.y = temp;
            }
        }
    }
    fn rotated(mut self) -> Self {
        self.rotate();
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
enum WireFrameGizmo {
    Circle(CircleGizmo),
    Square(SquareGizmo),
}
impl From<SquareGizmo> for WireFrameGizmo {
    fn from(value: SquareGizmo) -> Self {
        Self::Square(value)
    }
}
impl From<CircleGizmo> for WireFrameGizmo {
    fn from(value: CircleGizmo) -> Self {
        Self::Circle(value)
    }
}
impl Default for WireFrameGizmo {
    fn default() -> Self {
        Self::Circle(CircleGizmo::default())
    }
}

impl WireFrameGizmo {
    pub fn draw(&self, point: Vec3, gizmos: &mut Gizmos) {
        match self {
            WireFrameGizmo::Circle(circle_gizmo) => {
                gizmos.circle(point, circle_gizmo.radius, circle_gizmo.color);
            }
            WireFrameGizmo::Square(square_gizmo) => {
                gizmos.rect(point, square_gizmo.size, square_gizmo.color);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component)]
#[require(Transform)]
pub struct WireFrame {
    frame: WireFrameGizmo,
}
impl WireFrame {
    pub fn new_circle(radius: f32, color: Color) -> Self {
        let frame = CircleGizmo::new(radius, color).into();
        Self { frame }
    }
    pub fn new_square(size: Vec2, color: Color) -> Self {
        let frame = SquareGizmo::new(size, color).into();
        Self { frame }
    }
    pub fn draw(&self, point: Vec3, gizmos: &mut Gizmos) {
        self.frame.draw(point, gizmos);
    }
    pub fn rotate(&mut self) {
        self.frame = match self.frame {
            WireFrameGizmo::Circle(circle) => WireFrameGizmo::Circle(circle),
            WireFrameGizmo::Square(square_gizmo) => WireFrameGizmo::Square(square_gizmo.rotated()),
        }
    }
}
impl Default for WireFrame {
    fn default() -> Self {
        Self {
            frame: Default::default(),
        }
    }
}
