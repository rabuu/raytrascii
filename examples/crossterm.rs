use std::io::{self, Write};

use raytrascii::brightness::Brightness;
use raytrascii::hittable::{Hittable, HittableList, Sphere};
use raytrascii::lalg::{Point3, Vec3};
use raytrascii::ray::Ray;

use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, style};
use crossterm::{ExecutableCommand, QueueableCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();

    // set up terminal
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    // canvas
    let (cols, rows) = terminal::size()?;
    let aspect_ratio = cols as f64 / (rows * 2) as f64;

    // scene
    let sphere1 = Box::new(Sphere::new(Point3::new(0.5, 0.0, -1.0), 0.3));
    let sphere2 = Box::new(Sphere::new(Point3::new(-0.5, 0.0, -1.0), 0.3));

    let scene = HittableList::new(vec![sphere1, sphere2]);

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::origin();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    // render
    for j in (0..rows).rev() {
        for i in 0..cols {
            let u = i as f64 / (cols - 1) as f64;
            let v = j as f64 / (rows - 1) as f64;

            let direction = lower_left_corner + (u * horizontal) + (v * vertical) - origin;

            let ray = Ray::new(origin, direction);

            let brightness = ray_brightness(&ray, &scene);
            stdout.queue(style::Print(brightness))?;
        }
        stdout.queue(style::Print("\n"))?;
    }
    stdout.flush()?;

    // reset terminal
    stdout.execute(cursor::Show)?;

    Ok(())
}

fn ray_brightness(ray: &Ray, scene: &HittableList) -> Brightness {
    if let Some(_rec) = scene.hit(ray, 0.0, f64::INFINITY) {
        return Brightness(0.0);
    }

    let unit_dir = ray.dir.unit_vec();
    let t = 0.5 * (unit_dir.y + 1.0);

    let b = (1.0 - t) * 0.95 + (t * 0.7);
    Brightness(b)
}