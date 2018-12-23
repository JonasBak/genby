#[allow(dead_code)]
use perlin;
use utils;

enum Property {
    Gradient(f32, f32),
    Height(f32),
    Water(f32),
    Heat(f32),
    Resources(f32),
}

struct CellProperties {
    gradient: Property,
    height: Property,
    water: Property,
    heat: Property,
    resources: Property,
}

impl CellProperties {
    fn new(description: &WorldDescription, x: u32, y: u32) -> CellProperties {
        let gradient = description.heightmap.get_gradient(x, y);
        let mut waterlevel = description.waterlevel.get(x, y) - 0.1;
        if waterlevel < 0.0 {
            waterlevel = 0.0;
        }
        CellProperties {
            gradient: Property::Gradient(gradient.0, gradient.1),
            height: Property::Height(description.heightmap.get(x, y)),
            water: Property::Water(waterlevel),
            heat: Property::Heat(0.0),
            resources: Property::Resources(0.0),
        }
    }

    fn step(self, delta: f32, neighborhood: Neighborhood) -> CellProperties {
        CellProperties {
            gradient: self.gradient,
            height: self.height,
            water: update_property(self.water, delta, &neighborhood),
            heat: update_property(self.heat, delta, &neighborhood),
            resources: update_property(self.resources, delta, &neighborhood),
        }
    }
}

fn update_property(prop: Property, delta: f32, neighborhood: &Neighborhood) -> Property {
    match prop {
        Property::Water(level) => {
            Property::Water(level + delta * update_water(level, neighborhood))
        }
        _ => prop,
    }
}

struct Neighborhood {
    up: CellProperties,
    down: CellProperties,
    left: CellProperties,
    right: CellProperties,
    own: CellProperties,
}

struct Cell {
    properties: CellProperties,
}

impl Cell {
    fn new(description: &WorldDescription, x: u32, y: u32) -> Cell {
        Cell {
            properties: CellProperties::new(description, x, y),
        }
    }

    fn to_pixel(&self) -> (u8, u8, u8) {
        match self.properties.water {
            Property::Water(level) if level > 0.0 => return (0, 0, 255),
            _ => (),
        };
        match self.properties.height {
            Property::Height(height) => {
                let h = utils::map_range(-1.0..1.0, 0.0..255.0, height) as u8;
                return (h, h, h);
            }
            _ => (),
        };
        (0, 0, 0)
    }
}

fn update_water(level: f32, neighborhood: &Neighborhood) -> f32 {
    level
}

pub struct World {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
}

struct WorldDescription {
    heightmap: perlin::Noise,
    waterlevel: perlin::Noise,
}

impl World {
    pub fn new(size: u32) -> World {
        let description = WorldDescription {
            heightmap: perlin::Grid::new(5, 5).generate_noise(size / 5),
            waterlevel: perlin::Grid::new(5, 5).generate_noise(size / 5),
        };
        World {
            cells: (0..size * size)
                .map(|i| Cell::new(&description, i % size, i / size))
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
}
