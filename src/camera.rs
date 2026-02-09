use crate::vec3::Vec3;

pub struct Camera {
    pub eye: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn world_to_view(&self, v: Vec3) -> Vec3 {
        let p = v - self.eye;

        let forward = (self.look_at - self.eye).unit_vector();
        let right = forward.cross(self.up).unit_vector();
        let up = right.cross(forward);

        Vec3 {
            x: p.dot(right),
            y: p.dot(up),
            z: -p.dot(forward),
        }
    }

    pub fn perspective_projection(&self, v: Vec3) -> Option<Vec3> {
        if v.z <= 0.0 {
            return None;
        }

        Some(Vec3 {
            x: v.x / v.z,
            y: v.y / v.z,
            z: v.z,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_view() {
        let eye = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0); // Looking down negative Z
        let up = Vec3::new(0.0, 1.0, 0.0);
        let camera = Camera { eye, look_at, up };

        // Test case 1: world_to_view(camera.eye) == (0,0,0)
        let view_eye = camera.world_to_view(camera.eye);
        assert_eq!(view_eye, Vec3::new(0.0, 0.0, 0.0));

        // Test case 2: world_to_view(camera.look_at).z < 0 (OpenGL style)
        let view_look_at = camera.world_to_view(camera.look_at);
        assert!(view_look_at.z < 0.0);
    }
}
