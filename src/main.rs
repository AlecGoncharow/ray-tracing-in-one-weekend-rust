use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

mod hittable;
use hittable::Hittable;
use hittable::HittableList;
use hittable::Sphere;

mod camera;
mod material;
use material::Dielectric;
use material::Lambertian;
use material::Metal;

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

fn color(ray: Ray, world: Box<&dyn Hittable>, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(&ray, 0.001, std::f32::MAX) {
        if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
            if depth >= 50 {
                return Vec3::new(0.0, 0.0, 0.0);
            }
            return attenuation.make_comp_mul(&color(scattered, world, depth + 1));
        } else {
            // absorbed
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = ray.direction.make_unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::new(1.1, 1.1, 1.1);
    let mut rng = rand::thread_rng();
    while point.squared_mag() >= 1.0 {
        point = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);
    }

    point
}

fn main() -> std::io::Result<()> {
    let mut out = Output {
        rows: 250,
        cols: 500,
        colors: vec![],
    };
    let num_samples = 100;
    let camera = camera::Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        out.cols as f32 / out.rows as f32,
    );

    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.2)),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));

    world.push(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));

    for j in 0..out.rows {
        for i in 0..out.cols {
            let mut sampled_color_sum = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..num_samples {
                let u = (i as f32 + rng.gen::<f32>()) / out.cols as f32;
                let v = (out.rows as f32 - (j as f32 + rng.gen::<f32>())) / out.rows as f32;
                let ray = camera.get_ray(u, v);
                sampled_color_sum += color(ray, Box::new(&world), 0)
            }

            let unsum = sampled_color_sum * (1.0 / num_samples as f32);

            let color = Color::from_normalized_vec3(unsum.gamma_two());
            out.colors.push(color);
        }
    }

    out.write()
}
