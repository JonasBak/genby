trait Vector {
    fn slice(&self) -> &[f32];
    fn new(values: &[f32]) -> Self;

    fn len(&self) -> f32 {
        self.slice().iter().fold(0.0, |acc, x| acc + x * x).sqrt()
    }
}

fn add<T: Vector>(vec1: &T, vec2: &T) -> T {
    let values = vec1.slice();
    let new_values: Vec<f32> = vec2
        .slice()
        .iter()
        .enumerate()
        .map(|(i, val)| val + values[i])
        .collect();
    T::new(&new_values)
}

fn mul<T: Vector>(scalar: f32, vec: &T) -> T {
    let values: Vec<f32> = vec.slice().iter().map(|x| x * scalar).collect();
    T::new(&values)
}

fn dot<T: Vector>(vec1: &T, vec2: &T) -> f32 {
    let values = vec1.slice();
    vec2.slice()
        .iter()
        .enumerate()
        .map(|(i, x)| x * values[i])
        .fold(0.0, |acc, x| acc + x)
}

struct Vec3([f32; 3]);

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3([x, y, z])
    }
}

struct Vec2([f32; 2]);

impl Vec2 {
    fn new(x: f32, y: f32) -> Vec2 {
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

fn print_len(vec: &impl Vector) {
    println!("{}", vec.len());
}

fn main() {
    let a = Vec3::new(1.0, 4.0, 3.0);
    print_len(&a);

    let b = Vec2::new(3.0, 4.0);
    print_len(&b);
    print_len(&mul(2.0, &b));

    //print_len(Vector::add(&a, &a));
    print_len(&add(&a, &a));

    println!("Hello, world!");
}
