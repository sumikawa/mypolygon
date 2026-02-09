use crate::config::Settings;
use crate::vec3::Vec3;

pub const PI: f64 = std::f64::consts::PI;

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

    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    pub fn project_to_ndc(&self, v: Vec3, fov: f64) -> Option<Vec3> {
        let settings = Settings::new();
        let aspect_ratio = settings.aspect_ratio;
        let theta = Self::degrees_to_radians(fov);
        let f = 1.0 / (theta / 2.0).tan();

        if v.z <= 0.0 {
            return None;
        }

        Some(Vec3 {
            x: v.x * f / aspect_ratio / v.z,
            y: v.y * f / v.z,
            z: v.z,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

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

    #[test]
    fn project_center_point() {
        let eye = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0); // Looking down negative Z
        let up = Vec3::new(0.0, 1.0, 0.0);
        let camera = Camera { eye, look_at, up };

        // 視線方向の点は画面中央に来る
        let v = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
        let fov = 90.0;
        let aspect = 1.0;

        let p = camera.project_to_ndc(v, fov, aspect).unwrap();

        assert!(approx_eq(p.x, 0.0));
        assert!(approx_eq(p.y, 0.0));
        assert!(approx_eq(p.z, 1.0));
    }

    #[test]
    fn project_right_point() {
        let eye = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0); // Looking down negative Z
        let up = Vec3::new(0.0, 1.0, 0.0);
        let camera = Camera { eye, look_at, up };

        // 右にある点は x > 0 になる
        let v = Vec3 { x: 1.0, y: 0.0, z: 1.0 };
        let fov = 90.0;
        let aspect = 1.0;

        let p = camera.project_to_ndc(v, fov, aspect).unwrap();

        assert!(p.x > 0.0);
        assert!(approx_eq(p.y, 0.0));
    }

    #[test]
    fn project_far_point_is_smaller() {
        let eye = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0); // Looking down negative Z
        let up = Vec3::new(0.0, 1.0, 0.0);
        let camera = Camera { eye, look_at, up };

        // 遠い点ほど投影後の x が小さくなる
        let near = Vec3 { x: 1.0, y: 0.0, z: 1.0 };
        let far = Vec3 { x: 1.0, y: 0.0, z: 2.0 };
        let fov = 90.0;
        let aspect = 1.0;

        let p_near = camera.project_to_ndc(near, fov, aspect).unwrap();
        let p_far = camera.project_to_ndc(far, fov, aspect).unwrap();

        assert!(p_far.x < p_near.x);
    }

    #[test]
    fn project_top_point() {
        let eye = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0); // Looking down negative Z
        let up = Vec3::new(0.0, 1.0, 0.0);
        let camera = Camera { eye, look_at, up };

        // 上にある点は y > 0 になる
        let v = Vec3 { x: 0.0, y: 1.0, z: 1.0 };
        let fov = 90.0;
        let aspect = 1.0;

        let p = camera.project_to_ndc(v, fov, aspect).unwrap();

        assert!(p.y > 0.0);
        assert!(approx_eq(p.x, 0.0));
    }

    #[test]
    fn project_behind_camera_is_none() {
        let eye = Vec3::new(0.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0); // Looking down negative Z
        let up = Vec3::new(0.0, 1.0, 0.0);
        let camera = Camera { eye, look_at, up };

        // カメラの後ろは描画されない
        let v = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let fov = 90.0;
        let aspect = 1.0;

        let p = camera.project_to_ndc(v, fov, aspect);

        assert!(p.is_none());
    }
}
