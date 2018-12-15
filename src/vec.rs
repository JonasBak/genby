pub trait Vector {
    fn slice(&self) -> &[f32];
    fn new(values: &[f32]) -> Self;

    fn len(&self) -> f32 {
        self.slice().iter().fold(0.0, |acc, x| acc + x * x).sqrt()
    }
}

pub fn add<T: Vector>(vec1: &T, vec2: &T) -> T {
    let values = vec1.slice();
    let new_values: Vec<f32> = vec2
        .slice()
        .iter()
        .enumerate()
        .map(|(i, val)| val + values[i])
        .collect();
    T::new(&new_values)
}

pub fn mul<T: Vector>(scalar: f32, vec: &T) -> T {
    let values: Vec<f32> = vec.slice().iter().map(|x| x * scalar).collect();
    T::new(&values)
}

pub fn dot<T: Vector>(vec1: &T, vec2: &T) -> f32 {
    let values = vec1.slice();
    vec2.slice()
        .iter()
        .enumerate()
        .map(|(i, x)| x * values[i])
        .fold(0.0, |acc, x| acc + x)
}

pub fn normalize<T: Vector>(vec: &T) -> T {
    mul(1.0 / vec.len(), &vec)
}

pub struct Vec3([f32; 3]);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3([x, y, z])
    }
}

pub struct Vec2([f32; 2]);

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2([x, y])
    }
}

impl Vector for Vec3 {
    fn slice(&self) -> &[f32] {
        &self.0
    }

    fn new(values: &[f32]) -> Vec3 {
        Vec3::new(values[0], values[1], values[2])
    }
}

impl Vector for Vec2 {
    fn slice(&self) -> &[f32] {
        &self.0
    }

    fn new(values: &[f32]) -> Vec2 {
        Vec2::new(values[0], values[1])
    }
}

pub fn print_len(vec: &impl Vector) {
    println!("{}", vec.len());
}
