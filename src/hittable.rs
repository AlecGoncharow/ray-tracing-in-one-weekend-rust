use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct HitRecord<'a> {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<dyn Material + Send + Sync>,
}

impl<'a> HitRecord<'a> {
    fn new(
        t: f32,
        point: Vec3,
        normal: Vec3,
        material: &'a Box<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            t,
            u: 0.0,
            v: 0.0,
            point,
            normal,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub fn get_sphere_uv(point: Vec3) -> (f32, f32) {
    let phi = point.z.atan2(point.x);
    let theta = point.y.asin();
    use std::f32::consts::PI;
    (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.squared_mag();
        let b = oc.dot(&ray.direction);
        let c = oc.squared_mag() - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut rec = HitRecord::new(
                0.0,
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                &self.material,
            );

            // check - root
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.point_at_parameter(rec.t);
                rec.normal = (rec.point - self.center) * (1.0 / self.radius);
                let (u, v) = get_sphere_uv((rec.point - self.center) * (1.0 / self.radius));
                rec.u = u;
                rec.v = v;

                return Some(rec);
            }

            // check + root
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.point_at_parameter(rec.t);
                rec.normal = (rec.point - self.center) * (1.0 / self.radius);
                let (u, v) = get_sphere_uv((rec.point - self.center) * (1.0 / self.radius));
                rec.u = u;
                rec.v = v;
                return Some(rec);
            }
        }
        None
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn push(&mut self, item: Box<dyn Hittable + Send + Sync>) {
        self.list.push(item);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut to_return: Option<HitRecord> = None;
        let mut closest_found = t_max;

        for item in self.list.iter() {
            if let Some(hit) = item.hit(ray, t_min, closest_found) {
                closest_found = hit.t;
                to_return = Some(hit);
            }
        }

        to_return
    }
}
