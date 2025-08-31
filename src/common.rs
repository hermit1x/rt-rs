use rand::Rng;
use rand::rngs::ThreadRng;

pub type Point3 = nalgebra::Vector3<f64>;
pub type Vec3 = nalgebra::Vector3<f64>;
pub type Color = nalgebra::Vector3<f64>;

pub fn random(rnd: &mut ThreadRng) -> f64 {
    rnd.gen_range(0.0..=1.0)
}

pub fn random_range(min: f64, max: f64, rnd: &mut ThreadRng) -> f64 {
    rnd.gen_range(min..=max)
}

pub fn random_vec3(rnd: &mut ThreadRng) -> Vec3 {
    Vec3::new(random(rnd), random(rnd), random(rnd))
}

pub fn random_range_vec3(min: f64, max: f64, rnd: &mut ThreadRng) -> Vec3 {
    Vec3::new(random_range(min, max, rnd), random_range(min, max, rnd), random_range(min, max, rnd))
}

pub fn random_unit_vec3(rnd: &mut ThreadRng) -> Vec3 {
    let mut v = random_range_vec3(-1.0, 1.0, rnd);
    while v.norm() > 1.0 {
        v = random_range_vec3(-1.0, 1.0, rnd);
    }
    v.normalize()
}

pub fn random_on_hemisphere(normal: &Vec3, rnd: &mut ThreadRng) -> Vec3 {
    let in_unit_sphere = random_unit_vec3(rnd);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    }
    else {
        -in_unit_sphere
    }
}