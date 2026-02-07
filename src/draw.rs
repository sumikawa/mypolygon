use crate::color::Color;
use crate::transform::Transform;
use crate::triangle::{Triangle, Vec2};
use image::ImageBuffer;

fn edge(a: Vec2, b: Vec2, p: Vec2) -> i32 {
    (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x)
}

fn barycentric(triangle: &Triangle, p: Vec2) -> Option<(f64, f64, f64)> {
    let v0 = triangle.v0.pos;
    let v1 = triangle.v1.pos;
    let v2 = triangle.v2.pos;

    let area = edge(v0, v1, v2) as f64;
    if area == 0.0 {
        return None;
    }

    let w0 = edge(v1, v2, p) as f64 / area;
    let w1 = edge(v2, v0, p) as f64 / area;
    let w2 = 1.0 - w0 - w1; // w2 = edge(v0, v1, p) as f64 / area;

    if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
        Some((w0, w1, w2))
    } else {
        None
    }
}

fn interpolate_color(triangle: &Triangle, w0: f64, w1: f64, w2: f64) -> Color {
    let v0 = triangle.v0;
    let v1 = triangle.v1;
    let v2 = triangle.v2;

    let r = w0 * v0.color.r + w1 * v1.color.r + w2 * v2.color.r;
    let g = w0 * v0.color.g + w1 * v1.color.g + w2 * v2.color.g;
    let b = w0 * v0.color.b + w1 * v1.color.b + w2 * v2.color.b;

    Color::new(r, g, b)
}

pub fn polygon_fill(
    fb: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    transform: &Transform,
    triangle: &Triangle,
) {
    let min_x = triangle.v0.pos.x.min(triangle.v1.pos.x.min(triangle.v2.pos.x));
    let max_x = triangle.v0.pos.x.max(triangle.v1.pos.x.max(triangle.v2.pos.x));
    let min_y = triangle.v0.pos.y.min(triangle.v1.pos.y.min(triangle.v2.pos.y));
    let max_y = triangle.v0.pos.y.max(triangle.v1.pos.y.max(triangle.v2.pos.y));

    for j in min_y..=max_y {
        for i in min_x..=max_x {
            let p = Vec2 { x: i, y: j };
            if let Some((w0, w1, w2)) = barycentric(triangle, p) {
                let color = interpolate_color(triangle, w0, w1, w2);
                let rgb = image::Rgb([
                    (color.r.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.g.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.b.clamp(0.0, 1.0) * 255.0) as u8,
                ]);

                let (sx, sy) = transform.to_screen(i, j);
                if sx >= 0 && sx < fb.width() as i32 && sy >= 0 && sy < fb.height() as i32 {
                    fb.put_pixel(sx as u32, sy as u32, rgb);
                }
            }
        }
    }
}
