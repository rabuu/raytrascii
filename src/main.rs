use std::sync::{atomic, Arc};

use raytrascii::{
    camera::Camera,
    color::Color,
    render::{RenderDimensions, RenderMode},
    scene::{
        hittable::Sphere,
        material::{Lambertian, Metal},
        Scene, SceneBackground,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ctrl-c handling
    let running = Arc::new(atomic::AtomicBool::new(true));
    let r = Arc::clone(&running);

    ctrlc::set_handler(move || {
        r.store(false, atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-c handler");

    // create scene
    let scene = Scene::builder(SceneBackground::Solid(Color::white()))
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

    // create camera
    let mut cam = Camera::default();

    // set up terminal
    let mut term = terminal::stdout();
    term.act(terminal::Action::HideCursor)?;

    while running.load(atomic::Ordering::SeqCst) {
        raytrascii::render::render(
            &mut term,
            &scene,
            &cam,
            RenderDimensions::TermSize,
            15,
            10,
            RenderMode::ColorAndBrightness,
        )?;

        cam.move_focused(0.02, 0.0, 0.0);
    }

    // reset terminal
    term.act(terminal::Action::ShowCursor)?;
    term.act(terminal::Action::ClearTerminal(terminal::Clear::All))?;

    Ok(())
}
