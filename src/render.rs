use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use itertools::Itertools;
use rayon::prelude::*;

use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use crossterm::{style, terminal};

use crate::camera::Camera;
use crate::color::Color;
use crate::lalg::{Point3, Vec3};
use crate::ray::Ray;
use crate::scene::hittable::Hittable;
use crate::scene::Scene;
use crate::scene::SceneBackground;

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
    scene: &Scene,
    dimensions: RenderDimensions,
    max_depth: usize,
    samples_per_pixel: usize,
    mode: RenderMode,
) -> io::Result<()> {
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

    // compute the image
    let image = Arc::new(Mutex::new(HashMap::new()));

    (0..rows).into_par_iter().for_each(|row| {
        (0..cols).into_par_iter().for_each(|col| {
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

            let color: Option<style::Color> =
                if mode == RenderMode::Color || mode == RenderMode::ColorAndBrightness {
                    Some(color.into())
                } else {
                    None
                };

            let position = ((rows - row) * cols) + col;

            let img = Arc::clone(&image);

            img.lock()
                .unwrap()
                .insert(position, (brightness_char, color));
        });
    });

    // output
    let mut stdout = io::stdout();

    // clear terminal
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // output image
    for (_, (ch, color)) in image.lock().unwrap().iter().sorted_by_key(|(&pos, _)| pos) {
        if let Some(color) = color {
            stdout.queue(style::SetForegroundColor(*color))?;
        }

        stdout.queue(style::Print(ch))?;

        if *color != None {
            stdout.queue(style::ResetColor)?;
        }
    }

    stdout.flush()?;

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

    match scene.background {
        SceneBackground::Solid(col) => return col,

        SceneBackground::VerticalGradient { top, bottom } => {
            let t = 0.5 * (ray.dir.unit_vec().y + 1.0);
            return (1.0 - t) * bottom + (t * top);
        }

        SceneBackground::HorizontalGradient { left, right } => {
            let t = 0.5 * (ray.dir.unit_vec().x + 1.0);
            return (1.0 - t) * left + (t * right);
        }
    }
}
