use crate::color::Color;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Color,
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub struct Triangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}
