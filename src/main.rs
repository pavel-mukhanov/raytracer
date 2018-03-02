extern crate image;
extern crate num_complex;
extern crate rand;

use tools::*;
use std::fs::File;
use image::{ImageBuffer, Rgb};

mod tools;

const MAX_RAY_DEPTH: usize = 5;

pub const BG_COLOR: &VectorF = &VectorF {
    x: 127.,
    y: 127.,
    z: 127.,
};

fn render(scene: Vec<Sphere>, image: &mut Image) {
    let inv_width = 1. / image.width as f32;
    let inv_height = 1. / image.height as f32;
    let fov = 100.;
    let aspectratio = image.width as f32 / image.height as f32;
    let angle = (std::f32::consts::PI * 0.5 * fov / 180.).tan();
    let mut pixel_index = 0;

    for y in 0..image.height {
        for x in 0..image.width {
            let ray = VectorF {
                x: (2. * ((x as f32 + 0.5) * inv_width) - 1.) * angle * aspectratio,
                y: (1. - 2. * ((y as f32 + 0.5) * inv_height)) * angle,
                z: -1.,
            };

            image.pixels[pixel_index] = trace(&scene, &ZERO_VECTOR, &ray, 0).into();
            pixel_index += 1;
        }
    }
}

fn trace(scene: &Vec<Sphere>, origin: &VectorF, ray: &VectorF, depth: usize) -> VectorF {
    let mut tnear = INFINITY;
    let mut current_obj: Option<&Sphere> = Option::None;

    for obj in scene {
        let int = intersect(&obj, &origin, &ray.norm());

        let t = match int {
            Some((t0, t1)) => if t0 < 0. {
                t1
            } else {
                t0
            },
            None => INFINITY,
        };

        if t < tnear {
            tnear = t;
            current_obj = Some(obj);
        }
    }

    if current_obj.is_none() {
        return *BG_COLOR;
    }

    let surface_color;
    let phit: VectorF = *origin + *ray * tnear; 
    let mut nhit: VectorF = phit - current_obj.unwrap().center; 
    nhit = nhit.norm();

    let bias = 1e-4; 
    if ray.dot(&nhit) > 0. {
        nhit = -nhit;
    }

    if depth < MAX_RAY_DEPTH {
        let facingratio = -ray.dot(&nhit);
        let fresneleffect = mix((1. - facingratio).powf(3.), 1., 0.1);
        let refldir: VectorF = *ray - nhit * 2. * ray.dot(&nhit);
        let reflection: &VectorF = &trace(scene, &(phit + nhit * bias), &refldir.norm(), depth + 1);

        surface_color = (*reflection * fresneleffect) * current_obj.unwrap().surface_color;
    } else {
        surface_color = current_obj.unwrap().surface_color;
    }

    surface_color
}

fn write_image(filename: &str, image: &Image) {
    let mut imgbuf = ImageBuffer::new(image.width as u32, image.height as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = x * image.width as u32 + y;
        *pixel = image.pixels[index as usize];
    }

    let fout = &mut File::create(filename).unwrap();
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
}

fn main() {
    let width = 256;
    let height = 256;
    let filename = "render.png";

    let mut image = Image::new(width, height);
    let mut scene: Vec<Sphere> = vec![];

    scene.push(Sphere {
        center: Vector {
            x: 0.0,
            y: 2.0,
            z: -10.0,
        },
        radius: 4.0,
        surface_color: *COLOR_GREEN,
    });

    scene.push(Sphere {
        center: Vector {
            x: 0.0,
            y: 5.0,
            z: -5.0,
        },
        radius: 4.0,
        surface_color: *COLOR_BLUE,
    });

    scene.push(Sphere {
        center: Vector {
            x: 10.0,
            y: 0.0,
            z: -20.0,
        },
        radius: 4.0,
        surface_color: *COLOR_RED,
    });

    render(scene, &mut image);
    write_image(filename, &image);
}
