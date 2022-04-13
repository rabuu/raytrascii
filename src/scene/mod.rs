//! The virtual scene of objects

use crate::color::Color;
use hittable::{Hittable, HittableList};

pub mod hittable;
pub mod material;

/// Background of a scene
#[derive(Debug, Clone)]
pub enum SceneBackground {
    Solid(Color),
    VerticalGradient { top: Color, bottom: Color },
    HorizontalGradient { left: Color, right: Color },
}

/// A scene consisting of various objects (hittables) and a background
#[derive(Debug)]
pub struct Scene {
    pub objects: HittableList,
    pub background: SceneBackground,
}

impl Scene {
    /// Default shorthand constructor
    pub fn new(objects: HittableList, background: SceneBackground) -> Self {
        Scene {
            objects,
            background,
        }
    }

    /// Constructs a scene using a builder
    pub fn builder(background: SceneBackground) -> SceneBuilder {
        SceneBuilder {
            objects: HittableList::empty(),
            background,
        }
    }
}

/// A builder for the [Scene] type
pub struct SceneBuilder {
    objects: HittableList,
    background: SceneBackground,
}

impl SceneBuilder {
    /// Add an object to the later [Scene]
    pub fn add_object(mut self, obj: Box<dyn Hittable>) -> SceneBuilder {
        self.objects.add(obj);
        self
    }

    /// Build the final scene
    pub fn build(self) -> Scene {
        Scene::new(self.objects, self.background)
    }
}
