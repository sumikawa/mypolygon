use crate::color::Color;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub pos: Vec3,
    pub z: f64,
    pub color: Color,
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Triangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

impl Triangle {
    pub fn bounding_box(&self) -> (i32, i32, i32, i32) {
        let v0 = self.v0.pos;
        let v1 = self.v1.pos;
        let v2 = self.v2.pos;

        let min_x = v0.x.min(v1.x.min(v2.x)).floor();
        let max_x = v0.x.max(v1.x.max(v2.x)).ceil();
        let min_y = v0.y.min(v1.y.min(v2.y)).floor();
        let max_y = v0.y.max(v1.y.max(v2.y)).ceil();

        (min_x as i32, max_x as i32, min_y as i32, max_y as i32)
    }
}

impl From<Pixel> for Vec3 {
    fn from(item: Pixel) -> Self {
        Vec3 {
            // Convert to center pixel by adding 0.5
            x: item.x as f64 + 0.5,
            y: item.y as f64 + 0.5,
            z: 0.0
        }
    }
}
