use crate::common::*;
use crate::hittable::{Hittable, Interval};
use crate::ray::Ray;
use rand::Rng;
use indicatif::ProgressBar;
use rand::rngs::ThreadRng;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f64,
    width: usize,
    height: usize,
    sample_per_pixel: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, width: usize) -> Self {
        let height = ((width as f64) / aspect_ratio) as usize;
        let height = height.max(1);

        let sample_per_pixel = 16;

        let focal_length = 1.0f64;
        let viewport_height = 2.0f64;
        let viewport_width = viewport_height * (width as f64 / height as f64);

        let center = Point3::new(0.0, 0.0, 0.0);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (width as f64);
        let pixel_delta_v = viewport_v / (height as f64);

        let viewport_upper_left = center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            width,
            height,
            sample_per_pixel,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
        match world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(hit_record) => {
                let n = &hit_record.normal.normalize();
                0.5 * Color::new(n[0] + 1.0, n[1] + 1.0, n[2] + 1.0)
            },
            None => {
                let unit_direction = &ray.direction.normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
    
    pub fn write_color(&self, pixel_buffer: &mut [u8], color: Color) -> () {
        pixel_buffer[0] = (color[0] * 255.999) as u8;
        pixel_buffer[1] = (color[1] * 255.999) as u8;
        pixel_buffer[2] = (color[2] * 255.999) as u8;
    }

    fn get_ray(&self, i: usize, j: usize, rng: &mut ThreadRng) -> Ray {
        let ru: f64 = rng.gen_range(-1.0..=1.0);
        let rv: f64 = rng.gen_range(-1.0..=1.0);
        let pixel_sample = self.pixel00_loc
            + (i as f64) * self.pixel_delta_u
            + (j as f64) * self.pixel_delta_v
            + 0.5 * ru * self.pixel_delta_u
            + 0.5 * rv * self.pixel_delta_v;
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    pub fn render(&self, world: &(impl Hittable + Sync)) -> (usize, usize, Vec<u8>) {
        let size = self.width
            .checked_mul(self.height)
            .and_then(|px| px.checked_mul(3))
            .expect("width*height*3 overflowed");
        let mut buffer: Vec<u8> = vec![0u8; size];

        let bar = ProgressBar::new((self.width * self.height) as u64);

        // Parallelize by zipping pixel indices with mutable 3-byte chunks
        let width = self.width;
        buffer
            .par_chunks_mut(3)
            .enumerate()
            .for_each(|(idx, pix)| {
                let j = idx / width;
                let i = idx % width;
                let mut rng = rand::thread_rng();

                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    color += Self::ray_color(&self.get_ray(i, j, &mut rng), world);
                }
                let color = color / self.sample_per_pixel as f64;

                self.write_color(pix, color);
                bar.inc(1);
            });

        bar.finish();
        (self.width, self.height, buffer)
    }
}