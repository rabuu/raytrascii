use raytrascii::{
    color::Color,
    render::{RenderDimensions, RenderMode},
    scene::{
        hittable::Sphere,
        material::{Dielectric, Lambertian, Metal},
        Scene,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = Scene::builder()
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
                .radius(0.2)
                .material(Lambertian::new(Color::from_u8(255, 0, 0)).boxed())
                .build_boxed(),
        )
        // right
        .add_object(
            Sphere::builder()
                .center_xyz(0.5, 0.0, -1.0)
                .radius(0.2)
                .material(Metal::new(Color::from_u8(0, 0, 255), 0.8).boxed())
                .build_boxed(),
        )
        // middle
        .add_object(
            Sphere::builder()
                .center_xyz(0.0, 0.0, -1.0)
                .radius(0.2)
                .material(Dielectric::new(2.0).boxed())
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
        RenderMode::Color,
    )?;

    Ok(())
}
