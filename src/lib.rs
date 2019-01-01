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
pub fn get_pixels(draw_height: bool, draw_water: bool, draw_air_pressure: bool) -> Vec<u8> {
    unsafe {
        if let Some(ref world) = current_world {
            let (width, height) = world.size();
            let mut props = vec![0; (width * height * 3) as usize];
            for (i, cell) in world.cells.iter().enumerate() {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                if draw_height {
                    let h = (cell.properties.height.0 + 1.0) * 255.0 / 2.0;
                    r = h as u8;
                    g = h as u8;
                    b = h as u8;
                }

                if draw_water {
                    let w = cell.properties.water.0;
                    r = (r as f32 * (1.0 - w)).max(0.0) as u8;
                    g = (g as f32 * (1.0 - w)).max(0.0) as u8;
                    if w > 0.001 {
                        b = 255;
                    }
                }

                if draw_air_pressure {
                    let mut p = cell.properties.air_pressure.0 / 2.0;
                    p = p * p * p;
                    r = (r as f32 * (1.0 - p)).max(0.0).min(255.0) as u8;
                    g = (g as f32 * (1.0 - p) + p * 255.0).min(255.0).max(0.0) as u8;
                    b = (b as f32 * (1.0 - p)).max(0.0).min(255.0) as u8;
                }

                props[3 * i] = r;
                props[3 * i + 1] = g;
                props[3 * i + 2] = b;
            }
            props
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
            let mut directions = vec![0.0; (width * height * 2) as usize];
            for (i, cell) in world.cells.iter().enumerate() {
                directions[2 * i] = *cell.properties.wind.0.xy().0;
                directions[2 * i + 1] = *cell.properties.wind.0.xy().1;
            }
            directions
        } else {
            vec![]
        }
    }
}

#[wasm_bindgen]
pub fn alter_world(
    center_x: u32,
    center_y: u32,
    radius: u32,
    d_water: f32,
    d_air_pressure: f32,
    d_height: f32,
) {
    unsafe {
        if let Some(ref mut world) = current_world {
            let indices = world.select_cells((center_x, center_y), radius);
            for i in indices.iter() {
                world.cells[*i].properties =
                    world.cells[*i]
                        .properties
                        .alter_properties(d_water, d_air_pressure, d_height);
            }
        }
    }
}
