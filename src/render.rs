//! Output rendering

use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use terminal::Action;
use terminal::Terminal;
use terminal::Value;

use rayon::prelude::*;

use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::scene::hittable::Hittable;
use crate::scene::Scene;
use crate::scene::SceneBackground;

/// Dimensions/size of the rendered output
pub enum RenderDimensions {
    ConcreteSize { cols: u16, rows: u16 },
    TermSize,
    RelativeToTermSize { offset_cols: i32, offset_rows: i32 },
}

/// Mode of the rendered output
#[derive(Debug, PartialEq, Eq)]
pub enum RenderMode {
    Brightness,
    Color,
    ColorAndBrightness,
}

/// Main render function that composes a scene and a camera and outputs an image
pub fn render(
    term: &mut Terminal<io::Stdout>,
    scene: &Scene,
    cam: &Camera,
    dimensions: RenderDimensions,
    max_depth: usize,
    samples_per_pixel: usize,
    mode: RenderMode,
) -> terminal::error::Result<()> {
    let (cols, rows) = match dimensions {
        RenderDimensions::ConcreteSize { cols, rows } => (cols, rows),
        RenderDimensions::TermSize => {
            if let terminal::Retrieved::TerminalSize(cols, rows) = term.get(Value::TerminalSize)? {
                (cols, rows)
            } else {
                panic!("Could not get terminal size");
            }
        }
        RenderDimensions::RelativeToTermSize {
            offset_cols,
            offset_rows,
        } => {
            if let terminal::Retrieved::TerminalSize(cols, rows) = term.get(Value::TerminalSize)? {
                (
                    (cols as i32 + offset_cols) as u16,
                    (rows as i32 + offset_rows) as u16,
                )
            } else {
                panic!("Could not get terminal size");
            }
        }
    };

    let aspect_ratio = cols as f64 / (rows * 2) as f64;
    let view = cam.get_view(aspect_ratio);

    // compute the output
    let output: Vec<Option<(char, Option<terminal::Color>)>> = vec![None; (cols * rows) as usize];
    let output = Arc::new(Mutex::new(output));

    (0..rows).into_par_iter().for_each(|row| {
        (0..cols).into_par_iter().for_each(|col| {
            let mut color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (col as f64 + rand::random::<f64>()) / (cols - 1) as f64;
                let v = (row as f64 + rand::random::<f64>()) / (rows - 1) as f64;
                let ray = view.get_ray(u, v);
                color += ray_color(&ray, scene, max_depth);
            }

            let color = color.correct(2.0, samples_per_pixel);

            let brightness_char =
                if mode == RenderMode::Brightness || mode == RenderMode::ColorAndBrightness {
                    const PALETTE: [char; 69] = [
                        '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b',
                        'd', 'p', 'q', 'w', 'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X',
                        'z', 'c', 'v', 'u', 'n', 'x', 'r', 'j', 'f', 't', '/', '\\', '|', '(', ')',
                        '1', '{', '}', '[', ']', '?', '-', '_', '+', '~', '<', '>', 'i', '!', 'l',
                        'I', ';', ':', ',', '"', '^', '`', '\'', '.',
                    ];

                    let b = color.brightness();
                    let idx = ((b * PALETTE.len() as f64) as usize).clamp(0, PALETTE.len());

                    PALETTE[idx]
                } else {
                    '#'
                };

            let color: Option<terminal::Color> =
                if mode == RenderMode::Color || mode == RenderMode::ColorAndBrightness {
                    Some(color.into())
                } else {
                    None
                };

            let position = ((rows - row - 1) * cols) + col;
            let output = Arc::clone(&output);
            output.lock().unwrap()[position as usize] = Some((brightness_char, color));
        });
    });

    // clear terminal
    term.batch(Action::ClearTerminal(terminal::Clear::All))?;

    // output image
    for (ch, color) in output.lock().unwrap().iter().map(|o| o.unwrap()) {
        if let Some(color) = color {
            term.batch(Action::SetForegroundColor(color))?;
        }

        term.write_all(&[ch as u8])?;

        if color != None {
            term.batch(Action::ResetColor)?;
        }
    }

    term.flush_batch()?;

    Ok(())
}

/// Return the output color of a specific ray
fn ray_color(ray: &Ray, scene: &Scene, depth: usize) -> Color {
    if depth == 0 {
        return Color::black();
    }

    if let Some(rec) = scene.objects.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered, scene, depth - 1);
        }
        return Color::black();
    }

    match scene.background {
        SceneBackground::Solid(col) => col,

        SceneBackground::VerticalGradient { top, bottom } => {
            let t = 0.5 * (ray.dir.unit_vec().y + 1.0);
            (1.0 - t) * bottom + (t * top)
        }

        SceneBackground::HorizontalGradient { left, right } => {
            let t = 0.5 * (ray.dir.unit_vec().x + 1.0);
            (1.0 - t) * left + (t * right)
        }
    }
}
