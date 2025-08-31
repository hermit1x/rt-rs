use rand::Rng;
use rand::rngs::ThreadRng;
use rand_distr::{Normal, Distribution};

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