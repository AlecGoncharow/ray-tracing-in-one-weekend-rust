use crate::ray::Ray;
use crate::vec::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    fn new(t: f32, point: Vec3, normal: Vec3) -> Self {
        Self { t, point, normal }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
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
            let mut rec = HitRecord::new(0.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));

            // check - root
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.point_at_parameter(rec.t);
                rec.normal = (rec.point - self.center) * (1.0 / self.radius);
                return Some(rec);
            }

            // check + root
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.point_at_parameter(rec.t);
                rec.normal = (rec.point - self.center) * (1.0 / self.radius);
                return Some(rec);
            }
        }
        None
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn push(&mut self, item: Box<dyn Hittable>) {
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
