use cgmath::{Vector3, InnerSpace};

use super::{Hitinfo, Hitable, Ray};

pub trait Material {
    fn calc_mat(&self, info: &Hitinfo, scene: &Vec<Box<dyn Hitable>>, bounce_limit: u8) -> Option<Hitinfo>;
}

pub struct Diffuse {

}

impl Diffuse {
    fn random_uni_vector() -> Vector3<f64> {
        Vector3::new(rand::random::<f64>()*2.0-1.0, rand::random::<f64>()*2.0-1.0, rand::random::<f64>()*2.0-1.0)
    }

    fn random_vector(min: f64, max: f64) -> Vector3<f64> {
        Vector3::new(rand::random::<f64>()*max+min, rand::random::<f64>()*max+min, rand::random::<f64>()*max+min)
    }

    fn random_in_unit_sphere() -> Vector3<f64> {
        loop {
            let random_vector = Diffuse::random_uni_vector();
            if random_vector.magnitude2() >= 1.0 {
                continue;
            }
            return random_vector;
        }
    }
}

impl Material for Diffuse {
    fn calc_mat(&self, info: &Hitinfo, scene: &Vec<Box<dyn Hitable>>, bounce_limit: u8) -> Option<Hitinfo> {
        let target = info.position + info.normal + Diffuse::random_in_unit_sphere();

        let ray: Ray = Ray::new(info.position, target);

        let mut nearest: Option<Hitinfo> = None;

        let mut pixel_color = image::Rgb([0,0,0]);

        // This causes an overflow!!!!
        for object in scene {
            let info = object.hit(&ray, &scene, bounce_limit-1);
            if info.is_some() {
                if nearest.is_none() {
                    nearest = Some(*info.as_ref().unwrap())
                }
                //println!("{:?}", info.as_ref().unwrap().distance);
                if info.as_ref().unwrap().distance >= nearest.as_ref().unwrap().distance {
                    nearest = Some(*info.as_ref().unwrap());
                    let normal = nearest.unwrap().normal;
                    pixel_color = nearest.unwrap().color;
                    //pixel_color = image::Rgb([(255.0 - normal.x * 255.0) as u8, (255.0 - normal.y * 255.0) as u8, (255.0 - normal.z * 255.0) as u8]);
                }
            }
            //println!("{}", *nearest < 1000.0);
        }

        None
    }
}