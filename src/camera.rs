use crate::ray::Ray;
use crate::vec::Vec3;
use rand::Rng;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

fn random_point_in_unit_disk() -> Vec3 {
    let mut point = Vec3::new(1.1, 1.1, 1.1);
    let mut rng = rand::thread_rng();
    while point.squared_mag() >= 1.0 {
        point = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 0.0);
    }

    point
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let lens_radius = aperture / 2.0;

        let origin = look_from;
        let w = (look_from - look_at).make_unit_vector();
        let u = (v_up.cross(&w)).make_unit_vector();
        let v = w.cross(&u);

        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_point_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
