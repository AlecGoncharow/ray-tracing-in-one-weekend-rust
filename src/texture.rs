use crate::vec::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3;
}

pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _point: Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    odd_texture: Box<dyn Texture + Send + Sync>,
    even_texture: Box<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new(
        odd_texture: Box<dyn Texture + Send + Sync>,
        even_texture: Box<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            odd_texture,
            even_texture,
        }
    }
}

/*
pub struct CheckerTexture {
    odd_texture: ConstantTexture,
    even_texture: ConstantTexture,
}

impl CheckerTexture {
    pub fn new(odd_texture: ConstantTexture, even_texture: ConstantTexture) -> Self {
        Self {
            odd_texture,
            even_texture,
        }
    }
}
*/

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();

        if sines < 0.0 {
            self.odd_texture.value(u, v, point)
        } else {
            self.even_texture.value(u, v, point)
        }
    }
}
