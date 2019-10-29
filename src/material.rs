use crate::hittable::HitRecord;
use crate::random_point_in_unit_sphere;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec::Vec3;

use rand::Rng;

pub trait Material {
    // returns a scattered ray and an attenuation factor
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        (0.0, 0.0, 0.0).into()
    }
}

pub struct Lambertian {
    albedo: Box<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.point + hit.normal + random_point_in_unit_sphere();
        let scattered = Ray::new(hit.point, target - hit.point);
        let attenuation = self.albedo.value(0.0, 0.0, hit.point);

        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        if fuzz > 1.0 {
            Self { albedo, fuzz: 1.0 }
        } else {
            Self { albedo, fuzz }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.make_unit_vector().reflect(&hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + (self.fuzz * random_point_in_unit_sphere()),
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    let cos_flip = 1.0 - cosine;
    r0 + ((1.0 - r0) * cos_flip.powi(5))
}

pub fn slow_get_double() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = ray.direction.reflect(&hit.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(&hit.normal) > 0.0 {
            let cosine = self.ref_idx * ray.direction.dot(&hit.normal) / ray.direction.magnitude();

            (-1.0 * hit.normal, self.ref_idx, cosine)
        } else {
            let cosine = -1.0 * ray.direction.dot(&hit.normal) / ray.direction.magnitude();
            (hit.normal, 1.0 / self.ref_idx, cosine)
        };

        let refracted = ray.direction.refract(&outward_normal, ni_over_nt);

        let reflect_prob = if refracted.is_some() {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        if slow_get_double() < reflect_prob {
            Some((Ray::new(hit.point, reflected), attenuation))
        } else {
            Some((Ray::new(hit.point, refracted.unwrap()), attenuation))
        }
    }
}

pub struct DiffuseLight {
    emit: Box<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture + Send + Sync>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }
    fn emitted(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        self.emit.value(u, v, point)
    }
}
