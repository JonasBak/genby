mod vec;

extern crate rand;
use rand::Rng;

struct Grid {
    width: u32,
    height: u32,
    grid: Vec<vec::Vec2f>,
}

impl Grid {
    fn point_to_grid(&self, x: f32, y: f32) -> (u32, u32) {
        (x as u32 / self.width, y as u32 / self.height)
    }
}

fn new_random_vec() -> vec::Vec2f {
    let mut rng = rand::thread_rng();
    vec::normalize(&vec::Vec2f::new(
        rng.gen::<f32>() - 0.5,
        rng.gen::<f32>() - 0.5,
    ))
}

fn main() {
    let a = vec::Vec3f::new(1.0, 4.0, 3.0);
    println!("vec {:?}", a);

    let b = vec::Vec2f::new(3.0, 4.0);
    println!("vec {:?}", b);
    println!("vec {:?}", vec::mul(2.0, &b));

    println!("vec {:?}", vec::add(&a, &a));
    println!("vec {:?}", vec::normalize(&a));
    println!("vec {:?}", new_random_vec());
    println!("vec {:?}", new_random_vec());
}
