use std::fs::File;
use std::io::prelude::*;

mod ray;
use ray::Ray;
mod vec;
use vec::Vec3;

struct Output {
    rows: u32,
    cols: u32,
    colors: Vec<Color>,
}

impl Output {
    fn write(&self) -> std::io::Result<()> {
        let mut file = File::create("out.ppm")?;

        let header = format!("P3\n{} {}\n255\n", self.cols, self.rows);

        file.write_all(header.as_bytes())?;

        self.colors.iter().for_each(|color| {
            let row = format!("{} {} {}\n", color.r, color.g, color.b);
            file.write_all(row.as_bytes()).expect("color machine broke");
        });

        Ok(())
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn from_vec3(vec: Vec3) -> Self {
        Self::new(vec.x as u8, vec.y as u8, vec.z as u8)
    }

    fn from_normalized_vec3(vec: Vec3) -> Self {
        Self::from_vec3(vec * 255.99)
    }
}

fn color(ray: Ray) -> Color {
    let unit_direction = ray.direction.make_unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    let temp_color = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    Color::from_normalized_vec3(temp_color)
}

fn main() -> std::io::Result<()> {
    let mut out = Output {
        rows: 250,
        cols: 500,
        colors: vec![],
    };

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, -0.0);

    for j in 0..out.rows {
        for i in 0..out.cols {
            let u = i as f32 / out.cols as f32;
            let v = (out.rows - j) as f32 / out.cols as f32;

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let color = color(ray);

            out.colors.push(color);
        }
    }

    out.write()
}
