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
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut d: i32 = 2 * dy - dx;
    let mut y = y0;

    for x in x0..=x1 {
        let (sx, sy) = transform.to_screen(x, y);
        fb.put_pixel(
            sx as u32,
            sy as u32,
            image::Rgb([color.r, color.g, color.b]),
        );

        if d > 0 {
            d += 2 * (dy - dx);
            y += 1;
        } else {
            d += 2 * dy;
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

    draw_line(
        &mut imgbuf,
        &transform,
        30,
        30,
        300,
        150,
        Color::new(255, 255, 0),
    );

    imgbuf.save("output.png").unwrap();
}
