use mypolygon::color::Color;
use mypolygon::config::Settings;
use mypolygon::draw::{polygon_fill, polygon_outline};
use mypolygon::transform::Transform;
use mypolygon::triangle::{Triangle, Vec2};

fn main() {
    let settings = Settings::new();
    let aspect_ratio = settings.aspect_ratio;
    let image_width = settings.image_width;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let transform = Transform::new(image_height as i32);

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    let tri = Triangle {
        v0: Vec2 { x: 80, y: 30 },
        v1: Vec2 { x: 180, y: 120 },
        v2: Vec2 { x: 130, y: 210 },
    };
    polygon_fill(&mut imgbuf, &transform, &tri, Color::new(0, 0, 255));

    let tri2 = [(80, 30), (180, 120), (130, 210)];
    polygon_outline(&mut imgbuf, &transform, &tri2, Color::new(0, 255, 255));

    imgbuf.save("output.png").unwrap();
}
