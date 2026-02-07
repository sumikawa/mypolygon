use crate::color::Color;
use crate::transform::Transform;
use crate::triangle::Triangle;
use image::ImageBuffer;

pub fn line(
    fb: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    transform: &Transform,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = (x1 - x0).signum();
    let sy = (y1 - y0).signum();

    let mut x = x0;
    let mut y = y0;

    if dx >= dy {
        // x 主軸（横〜斜め）
        let mut d = 2 * dy - dx;

        for _ in 0..=dx {
            let (sx2, sy2) = transform.to_screen(x, y);
            fb.put_pixel(
                sx2 as u32,
                sy2 as u32,
                image::Rgb([color.r, color.g, color.b]),
            );

            if d > 0 {
                y += sy;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
            x += sx;
        }
    } else {
        // y 主軸（縦〜急勾配）
        let mut d = 2 * dx - dy;

        for _ in 0..=dy {
            let (sx2, sy2) = transform.to_screen(x, y);
            fb.put_pixel(
                sx2 as u32,
                sy2 as u32,
                image::Rgb([color.r, color.g, color.b]),
            );

            if d > 0 {
                x += sx;
                d += 2 * (dx - dy);
            } else {
                d += 2 * dx;
            }
            y += sy;
        }
    }
}

pub fn polygon_outline(
    fb: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    transform: &Transform,
    vertices: &[(i32, i32)],
    color: Color,
) {
    if vertices.len() < 2 {
        return;
    }

    vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .for_each(|(a, b)| {
            line(fb, transform, a.0, a.1, b.0, b.1, color);
        });
}

pub fn polygon_fill(
    fb: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    transform: &Transform,
    triangle: &Triangle,
    color: Color,
) {
    let mut verts = [triangle.v0, triangle.v1, triangle.v2];
    verts.sort_by_key(|v| v.y); // sort vertex as v0.y <= v1.y <= v2.y

    // scanline for first half
    for y in verts[0].y..verts[1].y {
        let dy = y - verts[0].y;

        let x01 = verts[0].x + (verts[1].x - verts[0].x) * dy / (verts[1].y - verts[0].y);
        let x02 = verts[0].x + (verts[2].x - verts[0].x) * dy / (verts[2].y - verts[0].y);

        let left_x = x01.min(x02);
        let right_x = x01.max(x02);

        line(fb, transform, left_x, y, right_x, y, color);
    }

    // scanline for second half
    for y in verts[1].y..verts[2].y {
        let dy = y - verts[2].y;

        let x01 = verts[2].x + (verts[0].x - verts[2].x) * dy / (verts[0].y - verts[2].y);
        let x02 = verts[2].x + (verts[1].x - verts[2].x) * dy / (verts[1].y - verts[2].y);

        let left_x = x01.min(x02);
        let right_x = x01.max(x02);

        line(fb, transform, left_x, y, right_x, y, color);
    }
}
