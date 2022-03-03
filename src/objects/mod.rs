use cgmath::{Vector3, InnerSpace};

pub mod material;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn function(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}

#[derive(Clone, Copy)]
pub struct Hitinfo<'a> {
    pub position: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub distance: f64,
    pub color: image::Rgb<u8>,
    pub scene: Option<&'a Vec<Box<dyn Hitable>>>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, scene: &Vec<Box<dyn Hitable>>, bounce_limit: u8) -> Option<Hitinfo>;
}