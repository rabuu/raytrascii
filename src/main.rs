use std::sync::{atomic, Arc};

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

    // ctrl-c handling
    let running = Arc::new(atomic::AtomicBool::new(true));
    let r = Arc::clone(&running);

    ctrlc::set_handler(move || {
        r.store(false, atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-c handler");

    // set up terminal
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::cursor::Hide)?;

    while running.load(atomic::Ordering::SeqCst) {
        raytrascii::render::render(
            &scene,
            RenderDimensions::TermSize,
            15,
            10,
            RenderMode::ColorAndBrightness,
        )?;
    }

    // reset terminal
    crossterm::execute!(stdout, crossterm::cursor::Show)?;

    Ok(())
}
