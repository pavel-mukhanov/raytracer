extern crate image;
extern crate num_complex;
extern crate rand;

use tools::*;
use std::fs::File;
use image::{ImageBuffer, Rgb};

mod tools;

fn render(scene: Vec<Sphere>, pixels: &mut Vec<Rgb<u8>>, imgx:u32, imgy:u32) {
    let inv_width = 1. / imgx as f32;
    let inv_height = 1. / imgy as f32;
    let fov = 100.;
    let aspectratio = imgx as f32 / imgy as f32;
    let angle = (std::f32::consts::PI * 0.5 * fov / 180.).tan();
    let mut pixel_index = 0;

    for y in 0..imgy {
        for x in 0..imgx {
            let ray = VectorF {
                x: (2. * ((x as f32 + 0.5) * inv_width) - 1.) * angle * aspectratio,
                y: (1. - 2. * ((y as f32 + 0.5) * inv_height)) * angle,
                z: -1.,
            };

            let int = intersect(scene[0].clone(), VectorF {x:0., y:0., z:0.}, ray.norm());

            pixels[pixel_index] = image::Rgb([0, if int.is_ok() {127} else {0}, 0]);
            pixel_index += 1;
        }
    }
}

fn write_image(pixels: Vec<Rgb<u8>>, imgx: u32, imgy: u32) {
    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = x * imgx as u32 + y;
        *pixel = pixels[index as usize];
    }

    let fout = &mut File::create("render.png").unwrap();
    image::ImageRgb8(imgbuf).save(fout, image::PNG).unwrap();
}

fn main() {
    let imgx = 256;
    let imgy = 256;
    let mut pixels = vec![Rgb([0, 0, 0]); imgx * imgy];
    let mut scene: Vec<Sphere> = vec![];

    scene.push(Sphere {
        center: Vector {
            x: 0.0,
            y: 0.0,
            z: -10.0,
        },
        radius: 4.0,
    });

    render(scene, &mut pixels, imgx as u32, imgy as u32);

    write_image(pixels, imgx as u32, imgy as u32);
}
