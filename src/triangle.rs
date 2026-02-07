use crate::color::Color;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Color,
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

pub struct Triangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

impl From<Pixel> for Vec2 {
    fn from(item: Pixel) -> Self {
        Vec2 {
            // Convert to center pixel by adding 0.5
            x: item.x as f64 + 0.5,
            y: item.y as f64 + 0.5,
        }
    }
}
