use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

mod hittable;
use hittable::Hittable;
use hittable::HittableList;
use hittable::Sphere;

mod texture;
use texture::CheckerTexture;
use texture::ConstantTexture;

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
    colors: Arc<Mutex<Vec<Color>>>,
}

impl Output {
    fn write(&self) -> std::io::Result<()> {
        let mut file = File::create("out.ppm")?;

        let header = format!("P3\n{} {}\n255\n", self.cols, self.rows);

        file.write_all(header.as_bytes())?;

        self.colors.lock().unwrap().iter().for_each(|color| {
            let row = format!("{} {} {}\n", color.r, color.g, color.b);
            file.write_all(row.as_bytes()).expect("color machine broke");
        });

        Ok(())
    }
}

#[derive(Clone)]
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

fn _sphere_cube_scene() -> HittableList {
    let mut list = HittableList::new();
    let mut rng = rand::thread_rng();
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Box::new(Metal::new(Vec3::new(0.5, 0.5, 0.5), 0.01)),
    )));

    for x in -2..3 {
        for y in 0..6 {
            for z in -2..3 {
                let center = Vec3::new(x as f32, y as f32 + 2.5, z as f32);
                let choose_mat = rng.gen::<f32>();
                if choose_mat < 0.3 {
                    // diffuse
                    list.push(Box::new(Sphere::new(
                        center,
                        0.5,
                        Box::new(Lambertian::new(Box::new(ConstantTexture::new(
                            (
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            )
                                .into(),
                        )))),
                    )));
                } else if choose_mat < 0.6 {
                    // metal
                    list.push(Box::new(Sphere::new(
                        center,
                        0.5,
                        Box::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    )));
                } else {
                    // glass
                    list.push(Box::new(Sphere::new(
                        center,
                        0.5,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    list
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut list = HittableList::new();
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -0.0),
        1000.0,
        Box::new(Lambertian::new(Box::new(CheckerTexture::new(
            Box::new(ConstantTexture::new((0.2, 0.3, 0.1).into())),
            Box::new(ConstantTexture::new((0.9, 0.9, 0.9).into())),
        )))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if choose_mat < 0.8 {
                // diffuse
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Box::new(Lambertian::new(Box::new(ConstantTexture::new(
                        (
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )
                            .into(),
                    )))),
                )));
            } else if choose_mat < 0.95 {
                // metal
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Box::new(Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                        ),
                        0.5 * rng.gen::<f32>(),
                    )),
                )));
            } else {
                // glass
                list.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Box::new(Dielectric::new(1.5)),
                )));
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Box::new(ConstantTexture::new(
            (0.4, 0.2, 0.1).into(),
        )))),
    )));

    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));

    /*
     * this makes glass sphere into a hollow bubble
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        -0.99,
        Box::new(Dielectric::new(1.5)),
    )));
    */
    list
}

struct OrderedColorVec {
    index: u32,
    colors: Arc<Mutex<Vec<Color>>>,
}

fn main() -> std::io::Result<()> {
    let out = Arc::new(Output {
        rows: 800,
        cols: 1200,
        colors: Arc::new(Mutex::new(vec![])),
    });
    let num_samples = 100;

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (look_from - look_at).magnitude();
    let aperature = 0.1;

    let camera = Arc::new(camera::Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        out.cols as f32 / out.rows as f32,
        aperature,
        dist_to_focus,
    ));

    // let world = Arc::new(random_scene());
    let world = Arc::new(random_scene());

    let mut threads = vec![];
    let color_vecs: Arc<Mutex<Vec<OrderedColorVec>>> = Arc::new(Mutex::new(vec![]));

    for i in 0..8 {
        let world = Arc::clone(&world);
        let camera = Arc::clone(&camera);
        let out = Arc::clone(&out);
        let colors = OrderedColorVec {
            index: i,
            colors: Arc::new(Mutex::new(vec![])),
        };
        let color_vecs = Arc::clone(&color_vecs);
        threads.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let start_row = i * out.rows / 8;
            let end_row = start_row + out.rows / 8;

            for j in start_row..end_row {
                println!("row: {:?}", j);
                for i in 0..out.cols {
                    let mut sampled_color_sum = Vec3::new(0.0, 0.0, 0.0);

                    for _ in 0..num_samples {
                        let u = (i as f32 + rng.gen::<f32>()) / out.cols as f32;
                        let v = (out.rows as f32 - (j as f32 + rng.gen::<f32>())) / out.rows as f32;
                        let ray = camera.get_ray(u, v);
                        sampled_color_sum += color(ray, Box::new(&*world), 0)
                    }

                    let unsum = sampled_color_sum * (1.0 / num_samples as f32);

                    let color = Color::from_normalized_vec3(unsum.gamma_two());
                    colors.colors.lock().unwrap().push(color);
                }
            }

            println!("thread done");
            color_vecs.lock().unwrap().push(colors);
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    color_vecs
        .lock()
        .unwrap()
        .sort_by(|a, b| a.index.cmp(&b.index));

    for color_vec in color_vecs.lock().unwrap().iter() {
        println!("adding index: {:?} colors", color_vec.index);
        for color in color_vec.colors.lock().unwrap().iter() {
            out.colors.lock().unwrap().push(color.clone());
        }
    }

    out.write()
}
