mod vec;

extern crate rand;
use rand::Rng;

fn new_random_vec() -> vec::Vec2 {
    let mut rng = rand::thread_rng();
    vec::normalize(&vec::Vec2::new(rng.gen(), rng.gen()))
}

fn main() {
    let a = vec::Vec3::new(1.0, 4.0, 3.0);
    println!("vec {:?}", a);

    let b = vec::Vec2::new(3.0, 4.0);
    println!("vec {:?}", b);
    println!("vec {:?}", vec::mul(2.0, &b));

    println!("vec {:?}", vec::add(&a, &a));
    println!("vec {:?}", vec::normalize(&a));
}
