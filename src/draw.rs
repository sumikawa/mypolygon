use crate::color::Color;
use crate::transform::Transform;
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
