use std::{
    sync::{atomic, Arc},
    time::Duration,
};

use terminal::{Action, Clear, Event, KeyCode, KeyModifiers, Retrieved, Value};

use raytrascii::{
    camera::{
        direction::{MoveDirection, RotationDirection},
        Camera,
    },
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
    term.act(Action::HideCursor)?;
    term.act(Action::EnableRawMode)?;
    term.act(Action::EnableMouseCapture)?;

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

        const MOVEMENT_SPEED: f64 = 0.03;

        if let Retrieved::Event(Some(event)) =
            term.get(Value::Event(Some(Duration::from_millis(50))))?
        {
            match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                            cam.move_relative(MoveDirection::Up, MOVEMENT_SPEED);
                        } else {
                            cam.move_relative(MoveDirection::Forward, MOVEMENT_SPEED);
                        }
                    }
                    KeyCode::Down => {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                            cam.move_relative(MoveDirection::Down, MOVEMENT_SPEED);
                        } else {
                            cam.move_relative(MoveDirection::Backward, MOVEMENT_SPEED);
                        }
                    }
                    KeyCode::Left => {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                            cam.rotate(RotationDirection::Left, MOVEMENT_SPEED);
                        } else {
                            cam.move_relative(MoveDirection::Left, MOVEMENT_SPEED);
                        }
                    }
                    KeyCode::Right => {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                            cam.rotate(RotationDirection::Right, MOVEMENT_SPEED);
                        } else {
                            cam.move_relative(MoveDirection::Right, MOVEMENT_SPEED);
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    // reset terminal
    term.act(Action::ClearTerminal(Clear::All))?;
    term.act(Action::DisableMouseCapture)?;
    term.act(Action::DisableRawMode)?;
    term.act(Action::ShowCursor)?;

    Ok(())
}
