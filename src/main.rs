use std::borrow::BorrowMut;

use image::codecs::hdr::Rgbe8Pixel;
use winit::dpi::PhysicalSize;
fn main() {
    let size: PhysicalSize<u32> = PhysicalSize::new(1920, 1080);

    let camera = Camera::new([-10.0, 1.0, 0.0], [0.0, 0.0, 0.0], 90.0);

    let samples: u8 = 16;
    let bounces: u8 = 64;

    let mut scene: Vec<Box<dyn Hitable>> = Vec::new();

    scene.push(Box::new(Sphere {
        position: [0.0,0.0,0.0],
        radius: 1.0,
        color: image::Rgb([255,120,0]),
        reflectance: 0.0,
    }));

    let mut image = image::ImageBuffer::from_fn(size.width, size.height, move |x,y| {

        //let (sx, sy) = ((x as f64 / size.width as f64)*2.0-1.0, (y as f64 / size.width as f64)*2.0-1.0);
        //let yaw = sx * (camera.fov/2.0);
        //let pitch = sx * (camera.fov/2.0);
        
        let ray: Ray = Ray::new(camera.position, normalize([x as f64-size.width as f64 /2.0,y as f64 - size.height as f64/2.0, size.width as f64]));

        for object in &scene {
            let info = object.hit(&ray);
            if info.is_none() {
                return image::Rgb([0u8,0u8,0u8]);
            } else {
                return info.unwrap().1;
            }
        }

        image::Rgb([0u8,0u8,0u8])
    });


    let path = format!("{}/out.png", std::env::current_dir().unwrap().display());
    image.save_with_format(path, image::ImageFormat::Png).unwrap();
}

fn normalize(v: [f64;3]) -> [f64;3]{
    let (x,y,z) = (v[0],v[1],v[2]);
    let n = (x*x+y*y+z*z).sqrt();
    [x/n,y/n,z/n]
}

// ----------

struct Camera {
    position: [f64; 3],
    rotation: [f64; 3],
    fov: f64,
}

impl Camera {
    pub fn new(position: [f64;3], rotation: [f64;3], fov: f64) -> Self {
        Self {
            position,
            rotation,
            fov,
        }
    }
}

// ----------

struct Ray {
    position: [f64; 3],
    direction: [f64; 3],
}

impl Ray {
    pub fn new(position: [f64;3], direction: [f64;3]) -> Self {
        Self {
            position,
            direction: normalize(direction),
        }
    }

    pub fn r(&self, t: f64) -> [f64;3] {
        let x = self.position[0] + t * self.direction[0];
        let y = self.position[1] + t * self.direction[1];
        let z = self.position[2] + t * self.direction[2];
        [x,y,z]
    }
}

struct Hitinfo {
    position: [f64; 3],
    direction: [f64; 3],
    distance: f64,
}

// ----------

trait Hitable {
    fn hit(&self, ray: &Ray) -> Option<(f64, image::Rgb<u8>, Hitinfo)>;
}

trait Light {
    fn is_illuminted(&self, position: &[f64; 3]);
}

// ----------

struct Sphere {
    position: [f64;3],
    radius: f64,
    color: image::Rgb<u8>,
    reflectance: f64,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<(f64, image::Rgb<u8>, Hitinfo)> {
        let (rx,ry,rz) = (ray.position[0],ray.position[1],ray.position[2]);
        let (cx,cy,cz) = (self.position[0],self.position[1],self.position[2]);
        let (ux,uy,uz) = (ray.direction[0],ray.direction[1],ray.direction[2]);

        let t = [rx-cx,ry-cy,rz-cz];
        let v = t[0]*ux+t[1]*uy+t[2]*uz;
        let c = ((rx-cx).exp2() + (ry-cy).exp2() + (rz-cz).exp2()).sqrt();

        let d2 = self.radius*self.radius - (c*c - v*v);

        if d2 < 0.0 { return None };

        let k = v-(d2).sqrt();

        let intercetion = [rx+ux*k,ry+uy*k,rz+uz*k];

        Some((self.reflectance, self.color, Hitinfo {
            position: intercetion,
            direction: normalize([intercetion[0]-cx, intercetion[1]-cy, intercetion[2]-cz]),
            distance: k,
        }))
    }
}