use super::{Point3, Vec3};

/// Three-dimensional orthonormal basis
#[derive(Debug, Clone, PartialEq)]
pub struct OrthNormBasis3 {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl OrthNormBasis3 {
    /// Constructs an orientation orthonormal basis from two points and a "view up" vector
    pub fn orientation(from: Point3, to: Point3, vup: Vec3) -> Self {
        let w = (from - to).unit_vec();
        let u = vup.cross(w).unit_vec();
        let v = w.cross(u);

        OrthNormBasis3 { u, v, w }
    }
}
