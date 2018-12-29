#[allow(dead_code)]
extern crate cfg_if;
extern crate image;
//extern crate rand;
extern crate js_sys;
extern crate wasm_bindgen;

mod cell;
mod perlin;
mod utils;
mod vec;
mod world;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

static mut current_world: Option<world::World> = None;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn create(x: f32, y: f32) {
    unsafe {
        current_world = Some(world::World::new(5, 100));
    }
}

#[wasm_bindgen]
pub fn tick(dt: f32) {
    unsafe {
        match current_world {
            Some(ref mut world) => world.update(dt),
            _ => (),
        }
    }
}

#[wasm_bindgen]
pub fn size() -> Vec<u32> {
    unsafe {
        let (x, y) = match current_world {
            Some(ref world) => world.size(),
            _ => (0, 0),
        };
        vec![x, y]
    }
}

#[wasm_bindgen]
pub fn get_pixels() -> Vec<u8> {
    unsafe {
        if let Some(ref world) = current_world {
            let (width, height) = world.size();
            let mut pixels = vec![0; (width * height * 3) as usize];
            for (i, cell) in world.cells.iter().enumerate() {
                let px = cell.to_pixel();
                pixels[3 * i] = px.0;
                pixels[3 * i + 1] = px.1;
                pixels[3 * i + 2] = px.2;
            }
            pixels
        } else {
            vec![]
        }
    }
}

#[wasm_bindgen]
pub fn get_wind_directions() -> Vec<f32> {
    unsafe {
        if let Some(ref world) = current_world {
            let (width, height) = world.size();
            let mut pixels = vec![0.0; (width * height * 2) as usize];
            for (i, cell) in world.cells.iter().enumerate() {
                pixels[2 * i] = *cell.properties.wind.0.xy().0;
                pixels[2 * i + 1] = *cell.properties.wind.0.xy().1;
            }
            pixels
        } else {
            vec![]
        }
    }
}

#[wasm_bindgen]
pub fn alter_world(center_x: u32, center_y: u32, radius: u32, d_water: f32, d_air_pressure: f32) {
    unsafe {
        if let Some(ref mut world) = current_world {
            let indices = world.select_cells((center_x, center_y), radius);
            for i in indices.iter() {
                world.cells[*i].properties = world.cells[*i]
                    .properties
                    .alter_properties(d_water, d_air_pressure);
            }
        }
    }
}
