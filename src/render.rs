use std::io;
use std::io::Write;

use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use crossterm::{cursor, style, terminal};

use crate::camera::Camera;
use crate::color::Color;
use crate::lalg::{Point3, Vec3};
use crate::ray::Ray;
use crate::scene::hittable::Hittable;
use crate::scene::Scene;

pub enum RenderDimensions {
    ConcreteSize { cols: u16, rows: u16 },
    TermSize,
    RelativeToTermSize { offset_cols: i32, offset_rows: i32 },
}

#[derive(Debug, PartialEq, Eq)]
pub enum RenderMode {
    Brightness,
    Color,
    ColorAndBrightness,
}

pub fn render(
    scene: Scene,
    dimensions: RenderDimensions,
    max_depth: usize,
    samples_per_pixel: usize,
    mode: RenderMode,
) -> io::Result<()> {
    let mut stdout = io::stdout();

    let (cols, rows) = match dimensions {
        RenderDimensions::ConcreteSize { cols, rows } => (cols, rows),
        RenderDimensions::TermSize => terminal::size()?,
        RenderDimensions::RelativeToTermSize {
            offset_cols,
            offset_rows,
        } => {
            let (cols, rows) = terminal::size()?;
            (
                (cols as i32 + offset_cols) as u16,
                (rows as i32 + offset_rows) as u16,
            )
        }
    };

    let aspect_ratio = cols as f64 / (rows * 2) as f64;

    // set up camera
    let lookfrom = Point3::origin();
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 90.0;
    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio);

    // set up terminal
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    for row in (0..rows).rev() {
        for col in 0..cols {
            let mut color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (col as f64 + rand::random::<f64>()) / (cols - 1) as f64;
                let v = (row as f64 + rand::random::<f64>()) / (rows - 1) as f64;
                let ray = cam.get_ray(u, v);
                color += ray_color(&ray, &scene, max_depth);
            }

            let color = color.correct(2.0, samples_per_pixel);

            let brightness_char =
                if mode == RenderMode::Brightness || mode == RenderMode::ColorAndBrightness {
                    let b = color.brightness();
                    '@'
                } else {
                    '#'
                };

            if mode == RenderMode::Color || mode == RenderMode::ColorAndBrightness {
                crossterm::queue!(stdout, style::SetForegroundColor(color.into()))?;
            }

            crossterm::queue!(stdout, style::Print(brightness_char), style::ResetColor)?;
        }
        stdout.queue(style::Print('\n'))?;
    }
    stdout.flush()?;

    // reset terminal
    stdout.execute(cursor::Show)?;

    Ok(())
}

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

    let t = 0.5 * (ray.dir.unit_vec().y + 1.0);

    let start_col = Color::white();
    let end_col = Color::new(0.7, 0.5, 0.8);

    (1.0 - t) * start_col + (t * end_col)
}
