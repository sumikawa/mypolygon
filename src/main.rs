use image::ImageBuffer;
use mypolygon::color::Color;
use mypolygon::config::Settings;
use mypolygon::transform::Transform;

fn draw_line(
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
            fb.put_pixel(sx2 as u32, sy2 as u32, image::Rgb([color.r, color.g, color.b]));

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
            fb.put_pixel(sx2 as u32, sy2 as u32, image::Rgb([color.r, color.g, color.b]));

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

fn main() {
    let settings = Settings::new();
    let aspect_ratio = settings.aspect_ratio;
    let image_width = settings.image_width;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let transform = Transform::new(image_height as i32);

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    for x in 0..image_width {
        draw_line(
            &mut imgbuf,
            &transform,
            0,
            0,
            x as i32,
            (image_height - 1) as i32,
            Color::new((255 * x / image_width / 2) as u8, 255 - (255 * x / image_width) as u8, 0),
        );
    }
    for y in 0..image_height {
        draw_line(
            &mut imgbuf,
            &transform,
            0,
            0,
            (image_width - 1) as i32,
            y as i32,
            Color::new(((255 * y / image_width / 2) + 128) as u8, 0, 255 - (255 * y / image_width) as u8),
        );
    }

    imgbuf.save("output.png").unwrap();
}
