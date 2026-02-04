use image::ImageBuffer;
use mypolygon::color::Color;

fn draw_line(
    fb: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
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
        fb.put_pixel(
            x as u32,
            y as u32,
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
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    draw_line(&mut imgbuf, 30, 30, 300, 150, Color::new(255, 255, 0));

    imgbuf.save("output.png").unwrap();
}
