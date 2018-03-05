extern crate image;
extern crate num_complex;
extern crate rand;

use tools::*;
use std::fs::File;
use image::ImageBuffer;

mod tools;

const MAX_RAY_DEPTH: usize = 30;

pub const BG_COLOR: &VectorF = &VectorF {
    x: 0.7,
    y: 0.7,
    z: 0.7,
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

            image.pixels[pixel_index] = trace(&scene, &ZERO_VECTOR, &ray.norm(), 0).into();
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

    let mut surface_color = *ZERO_VECTOR;
    let phit: VectorF = *origin + *ray * tnear;
    let nhit: VectorF = (phit - current_obj.unwrap().center).norm();

    let bias = 1e-4;
    let mut inside = false;
    if ray.dot(&nhit) > 0. {
        nhit = -nhit;
        inside = true;
    }

    if (current_obj.unwrap().reflection > 0. || current_obj.unwrap().transparency > 0.)
        && depth < MAX_RAY_DEPTH
    {
        let facingratio = -ray.dot(&nhit);
        let fresneleffect = mix((1. - facingratio).powf(3.), 1., 0.1);
        let refldir: VectorF = *ray - nhit * 2. * ray.dot(&nhit);
        let mut refraction: VectorF = *ZERO_VECTOR;
        let reflection: &VectorF = &trace(scene, &(phit + nhit * bias), &refldir.norm(), depth + 1);

        if current_obj.unwrap().transparency > 0. {
            let ior = 1.1f32;
            let eta = if inside { ior } else { 1. / ior };
            let cosi = -nhit.dot(ray);
            let k = 1. - eta.powf(2.) * (1. - cosi.powf(2.));
            let refrdir: VectorF = *ray * eta + nhit * (eta * cosi - k.sqrt());
            refraction = trace(scene, &(phit - nhit * bias), &refrdir.norm(), depth + 1);
        }

        surface_color = (*reflection * fresneleffect
            + refraction * (1. - fresneleffect) * current_obj.unwrap().transparency)
            * current_obj.unwrap().surface_color;
    } else {
        for (i, obj) in scene.iter().enumerate() {
            if obj.emission_color.x > 0. {
                let mut transmission: VectorF = *ONE_VECTOR;
                let light_direction = obj.center - phit;
                for (j, obj) in scene.iter().enumerate() {
                    if i != j {
                        let int = intersect(obj, &(phit + nhit * bias), &light_direction.norm());
                        if int.is_some() {
                            transmission = *ZERO_VECTOR;
                            break;
                        }
                    }
                }

                let coeff: f32 = if nhit.dot(&light_direction) > 0. {
                    nhit.dot(&light_direction)
                } else {
                    0.
                };

                surface_color +=
                    current_obj.unwrap().surface_color * transmission * coeff * obj.emission_color;
            }
        }
    }

    surface_color + current_obj.unwrap().emission_color
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
    let width = 1024;
    let height = 1024;
    let filename = "render.png";

    let mut image = Image::new(width, height);
    let mut scene: Vec<Sphere> = vec![];

    scene.push(Sphere {
        center: Vector {
            x: 10000.0,
            y: 0.0,
            z: -300.0,
        },
        radius: 10000.0,
        surface_color: *COLOR_GRAY,
        transparency: 0.,
        reflection: 0.,
        emission_color: *ZERO_VECTOR,
    });

    scene.push(Sphere {
        center: Vector {
            x: -2.0,
            y: -5.0,
            z: -5.0,
        },
        radius: 4.0,
        surface_color: *COLOR_GREEN,
        transparency: 0.0,
        reflection: 1.,
        emission_color: *ZERO_VECTOR,
    });

    scene.push(Sphere {
        center: Vector {
            x: -1.,
            y: 8.,
            z: -10.0,
        },
        radius: 2.,
        surface_color: *COLOR_BLUE,
        transparency: 0.9,
        reflection: 0.1,
        emission_color: *ZERO_VECTOR,
    });

    scene.push(Sphere {
        center: Vector {
            x: 0.,
            y: 5.,
            z: -25.,
        },
        radius: 3.,
        surface_color: *COLOR_RED,
        transparency: 0.5,
        reflection: 0.5,
        emission_color: *ZERO_VECTOR,
    });

    scene.push(Sphere {
        center: Vector {
            x: -20.0,
            y: 20.0,
            z: 10.0,
        },
        radius: 10.0,
        surface_color: *ZERO_VECTOR,
        transparency: 0.,
        reflection: 0.,
        emission_color: *LIGHT,
    });

    render(scene, &mut image);
    write_image(filename, &image);
}
