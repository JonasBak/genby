use rand::Rng;
use std::ops;
use vec;
use vec::Vector;

fn new_random_vec() -> vec::Vec2f {
    let mut rng = rand::thread_rng();
    vec::normalize(&vec::Vec2f::new(
        rng.gen::<f32>() - 0.5,
        rng.gen::<f32>() - 0.5,
    ))
}

fn interpolate(a: f32, b: f32, t: f32) -> f32 {
    let u = t * t * (3.0 - 2.0 * t);
    (1.0 - u) * a + u * b
}

fn map_range<T>(from: ops::Range<T>, to: ops::Range<T>, value: T) -> T
where
    T: PartialOrd<T>
        + ops::Sub<T, Output = T>
        + ops::Div<T, Output = T>
        + ops::Mul<T, Output = T>
        + ops::Add<T, Output = T>
        + Copy
        + Clone,
{
    (to.end - to.start) * (value - from.start) / (from.end - from.start) + to.start
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
        (x as u32, y as u32)
    }

    fn calculate_weight(&self, grid_x: u32, grid_y: u32, global_x: f32, global_y: f32) -> f32 {
        let relative_point = vec::Vec2f::new(global_x - grid_x as f32, global_y - grid_y as f32);

        vec::dot(
            &self.grid[(grid_x * self.width + grid_y) as usize],
            &relative_point,
        )
    }

    pub fn generate_noise(&self, resolution: u32) -> Noise {
        let width = resolution * (self.width - 1);
        let height = resolution * (self.height - 1);
        let size = width * height;
        let noise = (0..size)
            .map(|i| {
                let global_x = (i % width) as f32 / resolution as f32;
                let global_y = (i / width) as f32 / resolution as f32;

                let (grid_x, grid_y) = self.point_to_grid(global_x, global_y);

                let offsets = vec::Vec2f::new(global_x - grid_x as f32, global_y - grid_y as f32);

                println!(
                    "{:?}, {:?}, {:?}",
                    (grid_x, grid_y),
                    (global_x, global_y),
                    offsets
                );

                let weights: Vec<f32> = vec![
                    self.calculate_weight(grid_x, grid_y, global_x, global_y),
                    self.calculate_weight(grid_x + 1, grid_y, global_x, global_y),
                    self.calculate_weight(grid_x, grid_y + 1, global_x, global_y),
                    self.calculate_weight(grid_x + 1, grid_y + 1, global_x, global_y),
                ];

                //for (i, t) in offsets.slice().iter().enumerate() {
                //    let combined = interpolate(weights[i * 2], weights[i * 2 + 1], *t);
                //    weights.push(combined);
                //}
                //match weights.last() {
                //    Some(x) => *x,
                //    None => -1.0,
                //}
                interpolate(
                    interpolate(weights[0], weights[1], *offsets.get(0)),
                    interpolate(weights[2], weights[3], *offsets.get(0)),
                    *offsets.get(1),
                )
            })
            .collect();

        Noise {
            width: width,
            height: height,
            grid: noise,
        }
    }
}

#[derive(Debug)]
pub struct Noise {
    width: u32,
    height: u32,
    grid: Vec<f32>,
}

impl Noise {
    pub fn save_image(&self, file: &str) {
        let buffer: Vec<u8> = self
            .grid
            .iter()
            .map(|x| map_range(-1.0..1.0, 0.0..255.0, *x) as u8)
            .fold(vec![], |mut acc, x| {
                acc.push(x);
                acc.push(x);
                acc.push(x);
                acc.push(255);
                acc
            });

        image::save_buffer(
            &std::path::Path::new(file),
            &buffer,
            self.width,
            self.height,
            image::RGBA(8),
        );
    }
}
