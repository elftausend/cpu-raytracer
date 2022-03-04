use cgmath::{Vector3, InnerSpace};
use image::Rgb;

use super::{Hitinfo, Hitable, Ray, Material};

#[derive(Clone, Copy)]
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
        Diffuse::random_vector(-1.0, 1.0).normalize()
    }
}

impl Material for Diffuse {
    fn calc_mat(&self, info: &Hitinfo, scene: &Vec<Box<dyn Hitable>>, bounce_limit: u8) -> Rgb<u8> {
        let target = info.position + info.normal + Diffuse::random_in_unit_sphere();

        let light_direction = Vector3::new(0.0, 1.0, 0.0).normalize();

        let ray: Ray = Ray::new(info.position, target);

        let mut nearest: Option<Hitinfo> = None;

        let mut pixel_color = image::Rgb([255,255,255]);

        // This causes an overflow!!!!
        for object in scene {
            let sec = object.hit(&ray, &scene, bounce_limit-1);
            if sec.is_some() {
                if nearest.is_none() {
                    nearest = Some(*sec.as_ref().unwrap())
                }
                //println!("{:?}", info.as_ref().unwrap().distance);
                if sec.as_ref().unwrap().distance >= nearest.as_ref().unwrap().distance {
                    nearest = Some(*sec.as_ref().unwrap());
                    let d = sec.as_ref().unwrap().normal.dot(light_direction);
                    let normal = nearest.unwrap().normal;
                    //pixel_color = add_color_bias(info.color, nearest.unwrap().color, 0.5);
                    pixel_color = Rgb([
                        ((sec.as_ref().unwrap().normal.x/2.0+1.0) * 255.0) as u8, 
                        ((sec.as_ref().unwrap().normal.y/2.0+1.0) * 255.0) as u8, 
                        ((sec.as_ref().unwrap().normal.z/2.0+1.0) * 255.0) as u8
                    ]);
                    //println!("{}", d);
                    if d < 0.0001 {
                        pixel_color = add_color_bias(pixel_color, Rgb([0,0,0]), 0.2);
                    }
                    //pixel_color = image::Rgb([(255.0 - normal.x * 255.0) as u8, (255.0 - normal.y * 255.0) as u8, (255.0 - normal.z * 255.0) as u8]);
                }
            }
            //println!("{}", *nearest < 1000.0);
        }

        pixel_color
    }
}

pub fn add_color(c1: Rgb<u8>, c2: Rgb<u8>) -> Rgb<u8> {
    Rgb([((c1[0] as u32 + c2[0] as u32) / 2) as u8,
    ((c1[1] as u32 + c2[1] as u32) / 2) as u8,
    ((c1[2] as u32 + c2[2] as u32) / 2) as u8])
}

pub fn add_color_bias(c1: Rgb<u8>, c2: Rgb<u8>, bias: f64) -> Rgb<u8> {
    Rgb([((c1[0] as f64 * bias + c2[0] as f64 * (1.0 - bias))) as u8,
    ((c1[1] as f64 * bias + c2[1] as f64 * (1.0 - bias))) as u8,
    ((c1[2] as f64 * bias + c2[2] as f64 * (1.0 - bias))) as u8])
}