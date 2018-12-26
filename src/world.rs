use cell;
#[allow(dead_code)]
use perlin;
use utils;

pub struct World {
    cells: Vec<cell::Cell>,
    width: u32,
    height: u32,
}

pub struct WorldDescription {
    pub heightmap: perlin::Noise,
    pub windmap: perlin::Noise,
    pub waterlevel: perlin::Noise,
}

impl World {
    pub fn new(grid: u32, size: u32) -> World {
        let description = WorldDescription {
            heightmap: perlin::Grid::new(grid, grid).generate_noise(size / grid),
            windmap: perlin::Grid::new(grid, grid).generate_noise(size / grid),
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

    pub fn save_image(&self, file: &str) {
        let buffer: Vec<u8> =
            self.cells
                .iter()
                .map(|cell| cell.to_pixel())
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

    pub fn save_windmap(&self, file: &str) {
        let buffer: Vec<u8> =
            self.cells
                .iter()
                .map(|cell| cell.to_wind_pixel())
                .fold(vec![], |mut acc, px| {
                    acc.push(px.0);
                    acc.push(px.1);
                    acc.push(0);
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
        }
        self.cells = updated_cells;
    }
}
