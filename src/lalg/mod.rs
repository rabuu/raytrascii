//! Basic implementation of vectors etc

mod orthnormbasis;
mod vec;

pub use orthnormbasis::OrthNormBasis3;
pub use vec::Vec3;

/// Three-dimensional point (type alias to [Vec3])
pub type Point3 = Vec3;
