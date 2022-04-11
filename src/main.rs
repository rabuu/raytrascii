use raytrascii::{
    color::Color,
    render::{RenderDimensions, RenderMode},
    scene::{
        hittable::Sphere,
        material::{Lambertian, Metal},
        Scene, SceneBackground,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = Scene::builder(SceneBackground::VerticalGradient {
        top: Color::white(),
        bottom: Color::from_u8(125, 200, 255),
    })
    // ground
    .add_object(
        Sphere::builder()
            .center_xyz(0.0, -100.5, -1.0)
            .radius(100.0)
            .material(Lambertian::new(Color::from_u8(0, 255, 0)).boxed())
            .build_boxed(),
    )
    // left
    .add_object(
        Sphere::builder()
            .center_xyz(-0.5, 0.0, -1.0)
            .radius(0.5)
            .material(Lambertian::new(Color::from_u8(255, 0, 0)).boxed())
            .build_boxed(),
    )
    // right
    .add_object(
        Sphere::builder()
            .center_xyz(0.5, 0.0, -1.0)
            .radius(0.5)
            .material(Metal::new(Color::from_u8(0, 0, 255), 0.8).boxed())
            .build_boxed(),
    )
    .build();

    raytrascii::render::render(
        scene,
        RenderDimensions::RelativeToTermSize {
            offset_cols: 0,
            offset_rows: -2,
        },
        15,
        10,
        RenderMode::ColorAndBrightness,
    )?;

    Ok(())
}
