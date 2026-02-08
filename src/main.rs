use mypolygon::color::Color;
use mypolygon::config::Settings;
use mypolygon::draw::polygon_fill;
use mypolygon::transform::Transform;
use mypolygon::triangle::{Triangle, Vec2, Vertex};

fn main() {
    let settings = Settings::new();
    let aspect_ratio = settings.aspect_ratio;
    let image_width = settings.image_width;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let transform = Transform::new(image_height as i32);

    let mut imgbuf = image::ImageBuffer::new(image_width as u32, image_height as u32);

    let tri = Triangle {
        v0: Vertex {
            pos: Vec2 { x: 129.7, y: -26.4 },
            color: Color::new(0.0, 0.0, 1.0),
        },
        v1: Vertex {
            pos: Vec2 { x: 1327.3, y: 480.1 },
            color: Color::new(0.0, 1.0, 0.0),
        },
        v2: Vertex {
            pos: Vec2 { x: 524.0, y: 841.0 },
            color: Color::new(1.0, 0.0, 0.0),
        },
    };
    polygon_fill(&mut imgbuf, &transform, &tri);

    imgbuf.save(&settings.output_filename).unwrap();
}
