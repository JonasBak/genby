extern crate rand;

#[allow(dead_code)]
mod perlin;
mod vec;

fn main() {
    let a = vec::Vec3f::new(1.0, 4.0, 3.0);
    println!("{:?}", a);

    let b = vec::Vec2f::new(3.0, 4.0);
    println!("{:?}", b);
    println!("{:?}", vec::mul(2.0, &b));

    println!("{:?}", vec::add(&a, &a));
    println!("{:?}", vec::normalize(&a));

    println!("{:?}", perlin::Grid::new(3, 3));

    println!("{:?}", perlin::Grid::new(2, 2).generate_noise(2));
}
