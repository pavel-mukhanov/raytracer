extern crate image;
extern crate num_complex;
extern crate rand;

use tools::*;
use std::fs::File;
use image::{ImageBuffer, Rgb};

mod tools;

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

            let int = intersect(
                &scene[0],
                &VectorF {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                &ray.norm(),
            );

            image.pixels[pixel_index] = image::Rgb([0, if int.is_some() { 127 } else { 0 }, 0]);
            pixel_index += 1;
        }
    }
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
            y: 0.0,
            z: -10.0,
        },
        radius: 4.0,
    });

    render(scene, &mut image);
    write_image(filename, &image);
}
