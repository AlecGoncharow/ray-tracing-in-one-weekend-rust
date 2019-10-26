use crate::hittable::HitRecord;
use crate::random_point_in_unit_sphere;
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Material {
    // returns a scattered ray and an attenuation factor
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.point + hit.normal + random_point_in_unit_sphere();
        let scattered = Ray::new(hit.point, target - hit.point);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.make_unit_vector().reflect(&hit.normal);
        let scattered = Ray::new(hit.point, reflected);
        let attenuation = self.albedo;

        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
