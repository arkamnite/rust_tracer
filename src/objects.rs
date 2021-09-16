use crate::Vec3;

pub struct Camera {
    pub width: f64,
    height: f64,
    pub aspect_ratio: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub position: Vec3,
}

impl Camera {
    pub fn new(width: f64,
               aspect_ratio: f64,
               viewport_height: f64,
               focal_length: f64,
               position: Vec3
    ) -> Camera {
        Camera {
            width, height: width / aspect_ratio, aspect_ratio,
            viewport_width: aspect_ratio * viewport_height, viewport_height,
            focal_length, position
        }
    }
}

// impl Default for Camera {
//     fn default() -> Self {
//         Camera {
//             width: 800.0,
//             height: 450.0,
//             aspect_ratio: 16.0 / 9.0,
//             viewport_width: 16.0 / 9.0 * 2.0,
//             viewport_height: 0.0,
//             focal_length: 0.0,
//             position: Default::default()
//         }
//     }
// }