use std::borrow::BorrowMut;

use cgmath::{Vector3, InnerSpace, Rotation3, Rotation, Deg};
use image::codecs::hdr::Rgbe8Pixel;
use math::{Rect};
use objects::{Hitable, Hitinfo, Ray, material::{Diffuse}, Material};

mod objects;
mod math;

fn main() {
    let size: Rect<u32> = Rect::new(0, 0, 2560, 1440);

    let camera = Camera::new(
    Vector3::new(-100.0,0.0,-120.0), 
    Vector3::new(0.0,45.0,0.0), 
        90.0
    );

    let samples: u8 = 16;
    let bounces: u8 = 3;

    let mut scene: Vec<Box<dyn Hitable>> = Vec::new();

    scene.push(Box::new(Sphere {
        position: Vector3::new(0.0,0.0,0.0),
        radius: 10.0,
        color: image::Rgb([255,0,0]),
        material: Box::new(Diffuse {}),
    }));
    scene.push(Box::new(Sphere {
        position: Vector3::new(-30.0,0.0,0.0),
        radius: 10.0,
        color: image::Rgb([0,255,0]),
        material: Box::new(Diffuse {}),
    }));
    scene.push(Box::new(Sphere {
        position: Vector3::new(30.0,0.0,0.0),
        radius: 10.0,
        color: image::Rgb([0,0,255]),
        material: Box::new(Diffuse {}),
    }));
    /*scene.push(Box::new(Plane {
        position: Vector3::new(0.0,-40.0,0.0),
        normal: Vector3::new(0.0,1.0,0.0),
        scale: Vector3::new(80.0,80.0,80.0),
        color: image::Rgb([220,220,220]),
    }));*/

    let image = image::ImageBuffer::from_fn(size.width, size.height, move |x,y| {

        let mut direction = Vector3::new(x as f64-(size.width as f64 /2.0),y as f64 - (size.height as f64/2.0), size.width as f64 * 2.0).normalize();
        
        // Rotate direction 
        direction = camera.rotate_vec(direction);
        //println!("{:?}", direction);
        
        let ray: Ray = Ray::new(camera.position, direction);

        let mut nearest: Option<Hitinfo> = None;

        let mut pixel_color = image::Rgb([0,0,0]);

        for object in &scene {
            let info = object.hit(&ray, &scene, bounces);
            if info.is_some() {
                if nearest.is_none() {
                    nearest = Some(*info.as_ref().unwrap())
                }
                //println!("{:?}", info.as_ref().unwrap().distance);
                if info.as_ref().unwrap().distance >= nearest.as_ref().unwrap().distance {
                    nearest = Some(*info.as_ref().unwrap());
                    let mat = info.as_ref().unwrap().material.calc_mat(info.as_ref().unwrap(), &scene, bounces);
                    let normal = nearest.unwrap().normal;
                    pixel_color = mat;
                    //pixel_color = image::Rgb([(255.0 - normal.x * 255.0) as u8, (255.0 - normal.y * 255.0) as u8, (255.0 - normal.z * 255.0) as u8]);
                }
            }
            //println!("{}", *nearest < 1000.0);
        }

        pixel_color
    });

    let path = format!("{}/out.png", std::env::current_dir().unwrap().display());
    image.save_with_format(path, image::ImageFormat::Png).unwrap();
}

// ----------

struct Camera {
    position: Vector3<f64>,
    rotation: Vector3<f64>,
    fov: f64,
}

impl Camera {
    pub fn new(position: Vector3<f64>, rotation: Vector3<f64>, fov: f64) -> Self {
        Self {
            position,
            rotation,
            fov,
        }
    }

    pub fn rotate_vec(&self, mut v: Vector3<f64>) -> Vector3<f64> {
        let rotation_matrix = cgmath::Basis3::from_angle_z(cgmath::Deg(self.rotation.z));
        v = rotation_matrix.rotate_vector(v);
        let rotation_matrix = cgmath::Basis3::from_angle_x(cgmath::Deg(self.rotation.x));
        v = rotation_matrix.rotate_vector(v);
        let rotation_matrix = cgmath::Basis3::from_angle_y(cgmath::Deg(self.rotation.y));
        v = rotation_matrix.rotate_vector(v);
        v
    }
}

// ----------

// ----------

trait Light {
    fn is_illuminted(&self, ray: &Ray, scene: &Vec<Box<dyn Hitable>>);
}

// ----------


struct Sphere {
    position: Vector3<f64>,
    radius: f64,
    color: image::Rgb<u8>,
    material: Box<dyn Material>
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, scene: &Vec<Box<dyn Hitable>>, bounce_limit: u8) -> Option<Hitinfo> {

        let oc = ray.origin - self.position;

        let a =  ray.direction.magnitude2();

        let half_b = ray.direction.dot(oc);

        let c = oc.magnitude2() - self.radius.powi(2);

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let root = (-half_b - discriminant.sqrt()) / a;

        Some(Hitinfo {
            position: ray.function(root),
            normal: (ray.function(root) - self.position) / self.radius,
            distance: root,
            color: self.color,
            scene: None,
            material: &self.material,
        })
    }
}

/*
struct Plane {
    position: Vector3<f64>,
    normal: Vector3<f64>,
    scale: Vector3<f64>,
    color: image::Rgb<u8>,
}

impl Hitable for Plane {
    fn hit(&self, ray: &Ray, scene: &Vec<Box<dyn Hitable>>) -> Option<Hitinfo> {

        let denom = self.normal.dot(ray.direction);
        if denom.abs() > 0.0001 {
            let t = (self.position - ray.origin).dot(self.normal) / denom;
            return Some(Hitinfo {
                position: ray.origin + ray.direction * t,
                normal: self.normal,
                distance: t,
                color: self.color,
                scene: None
            });
        }

        None
    }
}*/