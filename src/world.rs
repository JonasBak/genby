use biome;
use cell;
#[allow(dead_code)]
use perlin;
use utils;

pub struct World {
    pub cells: Vec<cell::Cell>,
    pub width: u32,
    pub height: u32,
}

pub struct WorldDescription {
    pub heightmap: perlin::Noise,
    pub heatmap: perlin::Noise,
    pub waterlevel: perlin::Noise,
}

impl World {
    pub fn new(grid: u32, size: u32) -> World {
        let description = WorldDescription {
            heightmap: perlin::Grid::new(grid, grid).generate_noise(size / grid),
            heatmap: perlin::Grid::new(grid, grid).generate_noise(size / grid),
            waterlevel: perlin::Grid::new(grid, grid).generate_noise(size / grid),
        };
        World {
            cells: (0..size * size)
                .map(|i| cell::Cell::new(&description, i % size, i / size))
                .collect(),
            width: size,
            height: size,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn save_generic<T>(&self, file: &str, to_pixel: T)
    where
        T: Fn(&cell::Cell) -> (u8, u8, u8),
    {
        let buffer: Vec<u8> =
            self.cells
                .iter()
                .map(|cell| to_pixel(cell))
                .fold(vec![], |mut acc, px| {
                    acc.push(px.0);
                    acc.push(px.1);
                    acc.push(px.2);
                    acc.push(255);
                    acc
                });

        let _ = image::save_buffer(
            &std::path::Path::new(file),
            &buffer,
            self.width,
            self.height,
            image::RGBA(8),
        );
    }

    pub fn update(&mut self, delta: f32) {
        let mut updated_cells = vec![];
        let mut lake_indices = vec![];
        let mut mountain_indices = vec![];
        for i in 0..self.cells.len() {
            let p = (i as u32 % self.width, i as u32 / self.height);
            let p0 = (
                (p.0 + self.width - 1) % self.width,
                (p.1 + self.height - 1) % self.height,
            );
            let p1 = ((p.0 + 1) % self.width, (p.1 + 1) % self.height);
            let neighborhood = cell::Neighborhood {
                up: self.cells[(p1.1 * self.width + p.0) as usize].properties,
                down: self.cells[(p0.1 * self.width + p.0) as usize].properties,
                left: self.cells[(p.1 * self.width + p0.0) as usize].properties,
                right: self.cells[(p.1 * self.width + p1.0) as usize].properties,
                me: self.cells[i].properties,
            };
            updated_cells.push(self.cells[i].update(delta, &neighborhood));
            match biome::classify_tags(updated_cells[i].biome_tags) {
                biome::BiomeType::Lake => lake_indices.push(i),
                biome::BiomeType::Mountain => mountain_indices.push(i),
                _ => (),
            };
        }
        biome::update_biomes(delta, &mut updated_cells, lake_indices, mountain_indices);
        self.cells = updated_cells;
    }

    pub fn select_cells(&self, center: (u32, u32), radius: u32) -> Vec<usize> {
        let mut indices = vec![];
        for x in -(radius as i32)..(radius as i32) {
            let r_y = (((radius * radius) as i32 - x * x) as f32).sqrt() as i32;
            for y in -r_y..r_y {
                let px = (center.0 as i32 + x + self.width as i32) % (self.width as i32);
                let py = (center.1 as i32 + y + self.height as i32) % (self.height as i32);
                indices.push((py * (self.width as i32) + px) as usize);
            }
        }
        indices
    }
}
