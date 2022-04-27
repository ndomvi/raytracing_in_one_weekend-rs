use crate::Point;
use rand::{distributions::Uniform, prelude::*};

#[inline]
pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Point {
    let distr = Uniform::new_inclusive(-1.0, 1.0);

    loop {
        let v = Point::new(distr.sample(rng), distr.sample(rng), distr.sample(rng));

        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

#[inline]
pub fn random_unit_vector(rng: &mut SmallRng) -> Point {
    random_in_unit_sphere(rng).normalize()
}

#[inline]
pub fn random_in_unit_disk(rng: &mut SmallRng) -> Point {
    let distr = Uniform::new_inclusive(-1.0, 1.0);

    loop {
        let v = Point::new(distr.sample(rng), distr.sample(rng), 0.0);

        if v.length_squared() < 1.0 {
            return v;
        }
    }
}
