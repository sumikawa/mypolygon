use mypolygon::color::Color;
use mypolygon::config::Settings;
use mypolygon::draw::polygon_outline;
use mypolygon::transform::Transform;

fn main() {
    let settings = Settings::new();
    let aspect_ratio = settings.aspect_ratio;
    let image_width = settings.image_width;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let transform = Transform::new(image_height as i32);

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    let tri = [(80, 70), (180, 70), (130, 170)];
    polygon_outline(&mut imgbuf, &transform, &tri, Color::new(0, 0, 255));

    let rect = [(30, 30), (130, 30), (130, 80), (30, 80)];
    polygon_outline(&mut imgbuf, &transform, &rect, Color::new(255, 0, 255));

    let concave = [ (150, 50), (250, 50), (250, 150), (200, 100), (150, 150)];
    polygon_outline(&mut imgbuf, &transform, &concave, Color::new(255, 255, 0));

    imgbuf.save("output.png").unwrap();
}
