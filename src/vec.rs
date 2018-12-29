use std::ops;
use wasm_bindgen::prelude::*;

pub trait Vector<T> {
    fn new(values: &[T]) -> Self;

    fn slice(&self) -> &[T];

    fn get(&self, i: usize) -> &T {
        &self.slice()[i]
    }
}

pub fn add<T, E>(vec1: &T, vec2: &T) -> T
where
    E: ops::Add<Output = E> + Copy + Clone,
    T: Vector<E>,
{
    let values = vec1.slice();
    let new_values: Vec<E> = vec2
        .slice()
        .iter()
        .enumerate()
        .map(|(i, val)| val.add(values[i]))
        .collect();
    T::new(&new_values)
}

pub fn diff<T, E>(vec1: &T, vec2: &T) -> T
where
    E: ops::Sub<Output = E> + Copy + Clone,
    T: Vector<E>,
{
    let values = vec2.slice();
    let new_values: Vec<E> = vec1
        .slice()
        .iter()
        .enumerate()
        .map(|(i, val)| val.sub(values[i]))
        .collect();
    T::new(&new_values)
}

pub fn mul<T, E, S>(scalar: S, vec: &T) -> T
where
    S: Copy + Clone,
    E: ops::Mul<S, Output = E> + Copy + Clone,
    T: Vector<E>,
{
    let values: Vec<E> = vec.slice().iter().map(|x| x.mul(scalar)).collect();
    T::new(&values)
}

pub fn dot<T: Vector<f32>>(vec1: &T, vec2: &T) -> f32 {
    let values = vec1.slice();
    vec2.slice()
        .iter()
        .enumerate()
        .map(|(i, x)| x * values[i])
        .fold(0.0, |acc, x| acc + x)
}

pub fn len<T>(vec: &T) -> f32
where
    T: Vector<f32>,
{
    vec.slice().iter().fold(0.0, |acc, x| acc + x * x).sqrt()
}

pub fn normalize<T: Vector<f32>>(vec: &T) -> T {
    let length = len(vec);
    if length == 0.0 {
        T::new(vec.slice())
    } else {
        mul(1.0 / length, &vec)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T>([T; 3]);

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3([x, y, z])
    }

    pub fn xyz(&self) -> (&T, &T, &T) {
        (&self.0[0], &self.0[1], &self.0[2])
    }
}

impl<T: Copy + Clone> Vector<T> for Vec3<T> {
    fn new(values: &[T]) -> Vec3<T> {
        Vec3::new(values[0], values[1], values[2])
    }

    fn slice(&self) -> &[T] {
        &self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T>([T; 2]);

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2([x, y])
    }

    pub fn xy(&self) -> (&T, &T) {
        (&self.0[0], &self.0[1])
    }
}

impl<T: Copy + Clone> Vector<T> for Vec2<T> {
    fn new(values: &[T]) -> Vec2<T> {
        Vec2::new(values[0], values[1])
    }

    fn slice(&self) -> &[T] {
        &self.0
    }
}

//#[wasm_bindgen]
//#[derive(Debug, Copy, Clone)]
//pub struct WasmVec2f {
//    values: [f32; 2],
//}
//
//impl WasmVec2f {
//    pub fn new(x: f32, y: f32) -> WasmVec2f {
//        WasmVec2f { values: [x, y] }
//    }
//
//    pub fn xy(&self) -> (&f32, &f32) {
//        (&self.values[0], &self.values[1])
//    }
//}
//
//impl Vector<f32> for WasmVec2f {
//    fn new(values: &[f32]) -> WasmVec2f {
//        WasmVec2f::new(values[0], values[1])
//    }
//
//    fn slice(&self) -> &[f32] {
//        &self.values
//    }
//}

pub type Vec3f = Vec3<f32>;
pub type Vec2f = Vec2<f32>;
//pub type Vec2f = WasmVec2f;
