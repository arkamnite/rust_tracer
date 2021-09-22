use crate::{Vec3, unit_vector, Ray};

pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub focal_length: f64,
    pub position: Vec3,

    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(width: u32,
               aspect_ratio: f64,
               viewport_height: u32,
               focal_length: f64,
               position: Vec3
    ) -> Camera {
        Camera {
            width,
            height: (width as f64 / aspect_ratio) as u32,
            aspect_ratio,
            viewport_width: (aspect_ratio * viewport_height as f64) as u32,
            viewport_height,
            focal_length,
            position: position.clone(),
            horizontal: Vec3 { x: aspect_ratio * viewport_height as f64, y: 0.0, z: 0.0, },
            vertical: Vec3 { x: 0.0, y: viewport_height as f64, z: 0.0, },
            lower_left_corner: position.clone()
                - Vec3 { x: aspect_ratio * viewport_height as f64, y: 0.0, z: 0.0, }.div(2.0)
                - Vec3 { x: 0.0, y: viewport_height as f64, z: 0.0, }.div(2.0)
                - Vec3 {x: 0.0, y: 0.0, z: focal_length, }
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(1280,
                    16.0 / 9.0,
                    2,
                    1.0,
                    unit_vector(0.0)
        )
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.position.clone(),
            direction: self.lower_left_corner.clone() + self.horizontal.mul(u) + self.vertical.mul(v) - self.position.clone()
        }
    }
}