use rand::Rng;
use vec;

fn new_random_vec() -> vec::Vec2f {
    let mut rng = rand::thread_rng();
    vec::normalize(&vec::Vec2f::new(
        rng.gen::<f32>() - 0.5,
        rng.gen::<f32>() - 0.5,
    ))
}

#[derive(Debug)]
pub struct Grid {
    width: u32,
    height: u32,
    grid: Vec<vec::Vec2f>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        Grid {
            width: width,
            height: height,
            grid: (0..width * height).map(|_| new_random_vec()).collect(),
        }
    }

    pub fn point_to_grid(&self, x: f32, y: f32) -> (u32, u32) {
        (x as u32 / self.width, y as u32 / self.height)
    }
}
