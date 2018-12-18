#[allow(dead_code)]
extern crate image;
extern crate rand;

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

    perlin::Grid::new(8, 8)
        .generate_noise(100)
        .save_image("test.png");
}
