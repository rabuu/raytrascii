use self::hittable::{Hittable, HittableList};
use crate::color::Color;

pub mod hittable;
pub mod material;

#[derive(Debug, Clone)]
pub enum SceneBackground {
    Solid(Color),
    VerticalGradient { top: Color, bottom: Color },
    HorizontalGradient { left: Color, right: Color },
}

#[derive(Debug)]
pub struct Scene {
    pub objects: HittableList,
    pub background: SceneBackground,
}

impl Scene {
    pub fn new(objects: HittableList, background: SceneBackground) -> Self {
        Scene {
            objects,
            background,
        }
    }

    pub fn builder(background: SceneBackground) -> SceneBuilder {
        SceneBuilder {
            objects: HittableList::empty(),
            background,
        }
    }
}

pub struct SceneBuilder {
    objects: HittableList,
    background: SceneBackground,
}

impl SceneBuilder {
    pub fn add_object(mut self, obj: Box<dyn Hittable>) -> SceneBuilder {
        self.objects.add(obj);
        self
    }

    pub fn build(self) -> Scene {
        Scene::new(self.objects, self.background)
    }
}
