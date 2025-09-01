use std::f64::consts::PI;
use crate::common::*;
use crate::hittable::{Hittable, Interval};
use crate::ray::Ray;
use indicatif::ProgressBar;
use rand::rngs::ThreadRng;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f64,
    width: usize,
    height: usize,
    sample_per_pixel: usize,
    max_depth: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angel: f64,
    defocus_u: Vec3,
    defocus_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, width: usize) -> Self {
        let height = ((width as f64) / aspect_ratio) as usize;
        let height = height.max(1);

        let sample_per_pixel = 1024;
        let max_depth = 64;

        let defocus_angel = 0.6;
        let focus_distance = 10.0;

        let vertical_fov = 20.0;
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();

        let viewport_height = 2.0 * half_height * focus_distance;
        let viewport_width = viewport_height * (width as f64 / height as f64);

        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_at = Point3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);

        let center = look_from;

        let w = (look_from - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * - v;

        let pixel_delta_u = viewport_u / (width as f64);
        let pixel_delta_v = viewport_v / (height as f64);

        let viewport_upper_left = center
            - focus_distance * w
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_distance * (defocus_angel / 2.0 / 180.0 * PI).tan();
        let defocus_u = defocus_radius * u;
        let defocus_v = defocus_radius * v;

        Self {
            aspect_ratio,
            width,
            height,
            sample_per_pixel,
            max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angel,
            defocus_u,
            defocus_v,
        }
    }

    pub fn ray_color(&self, ray: &Ray, depth: usize, world: &impl Hittable, rng: &mut ThreadRng) -> Color {
        if depth >= self.max_depth {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(hit_record) => {
                match hit_record.material.scatter(ray, &hit_record, rng) {
                    Some((scattered, attenuation)) => {
                        attenuation.component_mul(
                            &self.ray_color(&scattered, depth + 1, world, rng)
                        )
                    },
                    None => Color::new(0.0, 0.0, 0.0)
                }
            },
            None => {
                let unit_direction = &ray.direction; // 直接就是 normalized 的
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    fn linear_to_gamma(&self, color: Color) -> Color {
        color.map(|c| if c > 0.0 { c.sqrt() } else { 0.0 })
    }

    pub fn write_color(&self, pixel_buffer: &mut [u8], color: Color) -> () {
        pixel_buffer[0] = (color[0] * 255.999) as u8;
        pixel_buffer[1] = (color[1] * 255.999) as u8;
        pixel_buffer[2] = (color[2] * 255.999) as u8;
    }

    fn get_ray(&self, i: usize, j: usize, rng: &mut ThreadRng) -> Ray {
        let ru: f64 = random_range(-0.5, 0.5, rng);
        let rv: f64 = random_range(-0.5, 0.5, rng);
        let pixel_sample = self.pixel00_loc
            + (i as f64) * self.pixel_delta_u
            + (j as f64) * self.pixel_delta_v
            + ru * self.pixel_delta_u
            + rv * self.pixel_delta_v;


        let ray_origin = if self.defocus_angel <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
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
                let mut rng = rand::rng();

                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    color += self.ray_color(
                        &self.get_ray(i, j, &mut rng),
                        0,
                        world,
                        &mut rng
                    );
                }
                let color = color / self.sample_per_pixel as f64;
                let color = self.linear_to_gamma(color);
                self.write_color(pix, color);
                bar.inc(1);
            });

        bar.finish();
        (self.width, self.height, buffer)
    }

    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Vec3 {
        let p = random_in_unit_disk(rng);
        self.center + self.defocus_u * p.x + self.defocus_v * p.y
    }
}