use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
