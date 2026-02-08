use crate::vec3::Vec3;

pub struct Camera {
    pub eye: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
}
