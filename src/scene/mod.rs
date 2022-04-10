use crate::scene::hittable::{Hittable, HittableList};

pub mod hittable;
pub mod material;

#[derive(Debug, Default)]
pub struct Scene {
    pub objects: HittableList,
}

impl Scene {
    pub fn new(objects: HittableList) -> Self {
        Scene { objects }
    }

    pub fn builder() -> SceneBuilder {
        SceneBuilder::default()
    }
}

#[derive(Default)]
pub struct SceneBuilder {
    objects: HittableList,
}

impl SceneBuilder {
    pub fn add_object(mut self, obj: Box<dyn Hittable>) -> SceneBuilder {
        self.objects.add(obj);
        self
    }

    pub fn build(self) -> Scene {
        Scene::new(self.objects)
    }
}
