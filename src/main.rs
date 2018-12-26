#[allow(dead_code)]
extern crate image;
extern crate rand;

mod cell;
mod perlin;
mod utils;
mod vec;
mod world;

fn main() {
    let a = vec::Vec3f::new(1.0, 4.0, 3.0);
    println!("{:?}", a);

    let b = vec::Vec2f::new(3.0, 4.0);
    println!("{:?}", b);
    println!("{:?}", vec::mul(2.0, &b));

    println!("{:?}", vec::add(&a, &a));
    println!("{:?}", vec::normalize(&a));

    println!("{:?}", perlin::Grid::new(3, 3));

    // perlin::Grid::new(8, 8)
    //     .generate_noise(100)
    //     .save_image("test.png");
    let mut world = world::World::new(2, 100);
    world.save_generic("test.png", |cell| cell.to_pixel());
    world.save_generic("test-windmap.png", |cell| cell.to_wind_pixel());
    for _ in 0..1000 {
        for _ in 0..10 {
            world.update(0.15);
        }
        println!("Images updated");
        world.save_generic("test.png", |cell| cell.to_airpressure_pixel());
        world.save_generic("test-windmap.png", |cell| cell.to_wind_pixel());
    }
}
