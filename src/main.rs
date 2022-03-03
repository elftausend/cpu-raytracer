use std::borrow::BorrowMut;

use image::codecs::hdr::Rgbe8Pixel;
use winit::dpi::PhysicalSize;
fn main() {
    let size: PhysicalSize<u32> = PhysicalSize::new(1920, 1080);

    let camera = Camera::new(
    Vector3::new(0.0,0.0,-90.0), 
    Vector3::new(0.0,0.0,0.0), 
        90.0
    );

    let samples: u8 = 16;
    let bounces: u8 = 64;

    let mut scene: Vec<Box<dyn Hitable>> = Vec::new();

    scene.push(Box::new(Sphere {
        position: Vector3::new(0.0,0.0,0.0),
        radius: 10.0,
        color: image::Rgb([255,120,0]),
        reflectance: 0.0,
    }));
    scene.push(Box::new(Sphere {
        position: Vector3::new(-8.0,2.0,-10.0),
        radius: 10.0,
        color: image::Rgb([255,120,0]),
        reflectance: 0.0,
    }));
    scene.push(Box::new(Sphere {
        position: Vector3::new(3.0,3.0,-10.0),
        radius: 8.0,
        color: image::Rgb([255,120,0]),
        reflectance: 0.0,
    }));

    let mut image = image::ImageBuffer::from_fn(size.width, size.height, move |x,y| {

        let direction = Vector3::new(x as f64-(size.width as f64 /2.0),y as f64 - (size.height as f64/2.0), size.width as f64).normalized();
        let ray: Ray = Ray::new(camera.position, direction);

        let mut nearest: Option<Hitinfo> = None;

        let mut pixel_color = image::Rgb([0,0,0]);

        for object in &scene {
            let info = object.hit(&ray);
            if info.is_some() {
                if nearest.is_none() {
                    nearest = Some(*info.as_ref().unwrap())
                }
                //println!("{:?}", info.as_ref().unwrap().distance);
                if info.as_ref().unwrap().distance <= nearest.as_ref().unwrap().distance {
                    nearest = Some(*info.as_ref().unwrap());
                    let normal = info.unwrap().normal;
                    pixel_color = image::Rgb([(255.0 - normal.x * 255.0) as u8, (255.0 - normal.y * 255.0) as u8, (255.0 - normal.z * 255.0) as u8]);
                }
            }
            //println!("{}", *nearest < 1000.0);
        }

        pixel_color
    });


    let path = format!("{}/out.png", std::env::current_dir().unwrap().display());
    image.save_with_format(path, image::ImageFormat::Png).unwrap();
}

#[derive(Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,y,z
        }
    }

    fn mag(&self) -> f64 {
        (self.x*self.x+self.y*self.y+self.z*self.z).sqrt()
    }

    fn normalized(&self) -> Self {
        let mag = self.mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn normalize(&mut self) -> &Self {
        let mag = self.mag();
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
        self
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = f64;

    fn mul(self, rhs: Vector3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs, 
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self * rhs.x, 
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs, 
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl std::ops::Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        Self::Output {
            x: self / rhs.x, 
            y: self / rhs.y,
            z: self / rhs.z
        }
    }
}

// ----------

struct Camera {
    position: Vector3,
    rotation: Vector3,
    fov: f64,
}

impl Camera {
    pub fn new(position: Vector3, rotation: Vector3, fov: f64) -> Self {
        Self {
            position,
            rotation: rotation.normalized(),
            fov,
        }
    }
}

// ----------

struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn function(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
}

#[derive(Clone, Copy)]
struct Hitinfo {
    position: Vector3,
    normal: Vector3,
    distance: f64,
}

// ----------

trait Hitable {
    fn hit(&self, ray: &Ray) -> Option<Hitinfo>;
}

trait Light {
    fn is_illuminted(&self, position: &[f64; 3]);
}

// ----------

struct Sphere {
    position: Vector3,
    radius: f64,
    color: image::Rgb<u8>,
    reflectance: f64,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hitinfo> {

        let oc = ray.origin - self.position;
        let a =  ray.direction.mag().powi(2);
        let half_b = oc * ray.direction;
        let c = oc.mag().powi(2) - self.radius.powi(2);
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;

        Some(Hitinfo {
            position: ray.function(root),
            normal: (ray.function(root) - self.position) / self.radius,
            distance: root,
        })
    }
}