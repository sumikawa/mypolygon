use mypolygon::color::Color;
use mypolygon::config::Settings;
use mypolygon::draw::polygon_fill;
use mypolygon::transform::Transform;
use mypolygon::triangle::{Triangle, Vec2};
use rand::Rng;

fn main() {
    let settings = Settings::new();
    let aspect_ratio = settings.aspect_ratio;
    let image_width = settings.image_width;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let transform = Transform::new(image_height as i32);

    let mut rng = rand::rng();
    let mut imgbuf = image::ImageBuffer::new(image_width as u32, image_height as u32);

    for _ in 1..100 {
        let tri = Triangle {
            v0: Vec2 {
                x: rng.random_range(0..image_width as i32),
                y: rng.random_range(0..image_height as i32),
            },
            v1: Vec2 {
                x: rng.random_range(0..image_width as i32),
                y: rng.random_range(0..image_height as i32),
            },
            v2: Vec2 {
                x: rng.random_range(0..image_width as i32),
                y: rng.random_range(0..image_height as i32),
            },
        };
        polygon_fill(
            &mut imgbuf,
            &transform,
            &tri,
            Color::new(
                rng.random_range(0..=255),
                rng.random_range(0..=255),
                rng.random_range(0..=255),
            ),
        );
    }

    imgbuf.save("output.png").unwrap();
}
