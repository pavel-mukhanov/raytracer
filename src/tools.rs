use std::ops::Add;
use std::ops::Sub;
use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type VectorF = Vector<f32>;

impl Vector<f32> {
    fn dot(&self, v: &Vector<f32>) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn norm(self) -> Self {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    fn length(&self) -> f32 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)).sqrt()
    }
}

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, other: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Rgb<u8>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            pixels: vec![Rgb([0, 0, 0]); width * height],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector<f32>,
    pub radius: f32,
}

pub fn intersect(sphere: &Sphere, origin: &VectorF, direction: &VectorF) -> Option<(f32, f32)> {
    let l = sphere.center - *origin;

    let tca = l.dot(direction);
    if tca < 0. {
        return None;
    };

    let d2 = &l.dot(&l) - tca * tca;
    if d2 > sphere.radius.powf(2.) {
        return None;
    }
    let thc = (sphere.radius.powf(2.) - d2).sqrt();
    let t0 = tca - thc;
    let t1 = tca + thc;
    Some((t0, t1))
}

#[test]
fn test_vector() {
    let v1 = VectorF {
        x: 1.,
        y: 2.,
        z: 3.,
    };
    let v2 = VectorF {
        x: 2.,
        y: 3.,
        z: 5.,
    };
    let v3 = VectorF {
        x: 2.,
        y: 3.,
        z: 5.,
    };

    let sphere = Sphere {
        center: Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 20.0,
    };

    println!("v1.norm()\n{:#?}", v1);

    assert!(intersect(&sphere, &v2, &v3).is_none());
    assert_eq!(3.7416575, v1.length());
    assert!(v1.length() - v1.norm().dot(&v1) < 0.0001);
}
