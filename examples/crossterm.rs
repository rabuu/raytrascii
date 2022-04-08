use std::io::{self, Write};

use raytrascii::camera::Camera;
use raytrascii::color::Color;
use raytrascii::hittable::{Hittable, HittableList, Sphere};
use raytrascii::lalg::{Point3, Vec3};
use raytrascii::material::Lambertian;
use raytrascii::ray::Ray;

use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, queue, style};
use crossterm::{ExecutableCommand, QueueableCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();

    // set up terminal
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    // canvas
    let (cols, rows) = terminal::size()?;
    let aspect_ratio = cols as f64 / (rows * 2) as f64;
    let max_depth = 20;
    let samples_per_pixel = 10;

    // scene
    let sphere1 = Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Color::from_u8(153, 40, 8))),
    ));
    let sphere2 = Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Color::from_u8(0, 154, 23))),
    ));

    let scene = HittableList::new(vec![sphere1, sphere2]);

    // camera
    let lookfrom = Point3::origin();
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 90.0;

    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio);

    // render
    for j in (0..rows - 2).rev() {
        for i in 0..cols {
            let mut color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (cols - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (rows - 1) as f64;
                let ray = cam.get_ray(u, v);
                color += ray_color(&ray, &scene, max_depth);
            }

            let color = color.correct(2.0, samples_per_pixel);
            let style_col = style::Color::Rgb {
                r: (color.r * 255.0) as u8,
                g: (color.g * 255.0) as u8,
                b: (color.b * 255.0) as u8,
            };
            queue!(
                stdout,
                style::SetForegroundColor(style_col),
                style::Print('#'),
                style::ResetColor,
            )?;
        }
        stdout.queue(style::Print("\n"))?;
    }
    stdout.flush()?;

    // reset terminal
    stdout.execute(cursor::Show)?;

    Ok(())
}

fn ray_color(ray: &Ray, scene: &HittableList, depth: usize) -> Color {
    if depth == 0 {
        return Color::black();
    }

    if let Some(rec) = scene.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered, scene, depth - 1);
        }
        return Color::black();
    }

    let unit_dir = ray.dir.unit_vec();
    let t = 0.5 * (unit_dir.y + 1.0);

    let start_color_vec = Color::white();
    let end_color_vec = Color::from_u8(125, 175, 255);

    (1.0 - t) * start_color_vec + (t * end_color_vec)
}
