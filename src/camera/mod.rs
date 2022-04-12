use crate::lalg::{OrthNormBasis3, Point3, Vec3};
use crate::utils;

use direction::{MoveDirection, RotationDirection};
use view::CameraView;

pub mod direction;
mod view;

#[derive(Debug, Clone)]
pub struct Camera {
    pos: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: Point3::origin(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 90.0,
        }
    }
}

impl Camera {
    pub(crate) fn get_view(&self, aspect_ratio: f64) -> CameraView {
        let theta = utils::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = self.pos;
        let orientation = OrthNormBasis3::orientation(self.pos, self.lookat, self.vup);

        let horiz = viewport_width * orientation.u;
        let vert = viewport_height * orientation.v;
        let lower_left_corner = origin - (horiz / 2.0) - (vert / 2.0) - orientation.w;

        CameraView {
            origin,
            lower_left_corner,
            horiz,
            vert,
        }
    }
}

/* MOVEMENT */
impl Camera {
    pub fn move_absolute(&mut self, x: f64, y: f64, z: f64) {
        self.pos.x += x;
        self.pos.y += y;
        self.pos.z += z;

        self.lookat.x += x;
        self.lookat.y += y;
        self.lookat.z += z;
    }

    pub fn move_relative(&mut self, dir: MoveDirection, step: f64) {
        let look_dir: Vec3 = (self.lookat - self.pos).unit_vec();

        match dir {
            MoveDirection::Forward => {
                self.pos += look_dir * step;
                self.lookat += look_dir * step;
            }
            MoveDirection::Backward => {
                self.pos -= look_dir * step;
                self.lookat -= look_dir * step;
            }
            MoveDirection::Left => {
                let dir = self.vup.cross(look_dir);
                self.pos += dir * step;
                self.lookat += dir * step;
            }
            MoveDirection::Right => {
                let dir = self.vup.cross(look_dir);
                self.pos -= dir * step;
                self.lookat -= dir * step;
            }
            MoveDirection::Up => {
                self.pos += self.vup * step;
                self.lookat += self.vup * step;
            }
            MoveDirection::Down => {
                self.pos -= self.vup * step;
                self.lookat -= self.vup * step;
            }
        }
    }

    pub fn rotate(&mut self, dir: RotationDirection, step: f64) {
        let look_dir: Vec3 = (self.lookat - self.pos).unit_vec();

        match dir {
            RotationDirection::Left => {
                let dir = self.vup.cross(look_dir);
                self.lookat += dir * step;
            }
            RotationDirection::Right => {
                let dir = self.vup.cross(look_dir);
                self.lookat -= dir * step;
            }
        }
    }
}
