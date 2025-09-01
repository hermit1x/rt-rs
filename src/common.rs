use rand::Rng;
use rand::rngs::ThreadRng;
use rand_distr::{Normal, Distribution};
pub use std::f64::consts::PI;

pub type Point3 = nalgebra::Vector3<f64>;
pub type Vec3 = nalgebra::Vector3<f64>;
pub type Color = nalgebra::Vector3<f64>;

pub fn random(rng: &mut ThreadRng) -> f64 {
    rng.random_range(0.0..=1.0)
}

pub fn random_range(min: f64, max: f64, rng: &mut ThreadRng) -> f64 {
    rng.random_range(min..=max)
}

pub fn random_vec3(rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(random(rng), random(rng), random(rng))
}

pub fn random_range_vec3(min: f64, max: f64, rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(random_range(min, max, rng), random_range(min, max, rng), random_range(min, max, rng))
}

pub fn random_unit_vec3(rng: &mut ThreadRng) -> Vec3 {
    let mut normal_dist = Normal::new(0.0, 1.0).unwrap();
    let x = normal_dist.sample(rng);
    let y = normal_dist.sample(rng);
    let z = normal_dist.sample(rng);
    Vec3::new(x, y, z).normalize()
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let theta = 2.0 * PI * random(rng);
    let r = random(rng).sqrt();
    Vec3::new(r * theta.cos(), r * theta.sin(), 0.0)
}

pub fn random_on_hemisphere(normal: &Vec3, rng: &mut ThreadRng) -> Vec3 {
    let in_unit_sphere = random_unit_vec3(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    }
    else {
        -in_unit_sphere
    }
}

pub fn near_zero(vec3: &Vec3) -> bool {
    vec3.norm() < 1e-8
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}