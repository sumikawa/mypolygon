use mypolygon::color::Color;
use mypolygon::config::Settings;
use mypolygon::draw::line;
use mypolygon::transform::Transform;

fn main() {
    let settings = Settings::new();
    let aspect_ratio = settings.aspect_ratio;
    let image_width = settings.image_width;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let transform = Transform::new(image_height as i32);

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    for x in 0..image_width {
        line(
            &mut imgbuf,
            &transform,
            0,
            0,
            x as i32,
            (image_height - 1) as i32,
            Color::new(
                (255 * x / image_width / 2) as u8,
                255 - (255 * x / image_width) as u8,
                0,
            ),
        );
    }
    for y in 0..image_height {
        line(
            &mut imgbuf,
            &transform,
            0,
            0,
            (image_width - 1) as i32,
            y as i32,
            Color::new(
                ((255 * y / image_width / 2) + 128) as u8,
                0,
                255 - (255 * y / image_width) as u8,
            ),
        );
    }

    imgbuf.save("output.png").unwrap();
}
