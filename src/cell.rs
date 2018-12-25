#[allow(dead_code)]
use utils;
use world;

#[derive(Copy, Clone)]
pub struct Gradient(f32, f32);

#[derive(Copy, Clone)]
pub struct Height(f32);

#[derive(Copy, Clone)]
pub struct Water(f32);

#[derive(Copy, Clone)]
pub struct Heat(f32);

#[derive(Copy, Clone)]
pub struct Resources(f32);

//#[derive(Copy, Clone)]
//enum Property {
//    Gradient(f32, f32),
//    Height(f32),
//    Water(f32),
//    Heat(f32),
//    Resources(f32),
//}

#[derive(Copy, Clone)]
pub struct CellProperties {
    gradient: Gradient,
    height: Height,
    water: Water,
    heat: Heat,
    resources: Resources,
}

impl CellProperties {
    fn new(description: &world::WorldDescription, x: u32, y: u32) -> CellProperties {
        let gradient = description.heightmap.get_gradient(x, y);
        let mut waterlevel =
            description.waterlevel.get(x, y) - 0.1 - description.heightmap.get(x, y);
        if waterlevel < 0.0 {
            waterlevel = 0.0;
        }
        CellProperties {
            gradient: Gradient(gradient.0, gradient.1),
            height: Height(description.heightmap.get(x, y)),
            water: Water(waterlevel),
            heat: Heat(0.0),
            resources: Resources(0.0),
        }
    }

    fn step(current: &CellProperties, delta: f32, neighborhood: &Neighborhood) -> CellProperties {
        CellProperties {
            gradient: current.gradient,
            height: current.height,
            water: update_water(delta, neighborhood),
            heat: update_heat(delta, neighborhood),
            resources: update_resources(delta, neighborhood),
        }
    }
}

pub struct Neighborhood {
    pub up: CellProperties,
    pub down: CellProperties,
    pub left: CellProperties,
    pub right: CellProperties,
    pub me: CellProperties,
}

pub struct Cell {
    pub properties: CellProperties,
}

impl Cell {
    pub fn new(description: &world::WorldDescription, x: u32, y: u32) -> Cell {
        Cell {
            properties: CellProperties::new(description, x, y),
        }
    }

    pub fn update(&self, delta: f32, neighborhood: &Neighborhood) -> Cell {
        let new_props = CellProperties::step(&self.properties, delta, neighborhood);
        Cell {
            properties: new_props,
        }
    }

    pub fn to_pixel(&self) -> (u8, u8, u8) {
        //if self.properties.water.0 > 0.01 {
        //    return (0, 0, 255);
        //}

        let w = self.properties.water.0.min(1.0);
        let h = utils::map_range(
            0.0..2.0,
            0.0..255.0,
            (self.properties.height.0 + 1.0) * (1.0 - w),
        );
        return (h as u8, h as u8, if w > 0.1 { 255 } else { h as u8 });
    }
}

fn water_diff(me: CellProperties, close: CellProperties) -> f32 {
    if close.height.0 + close.water.0 > me.height.0 + me.water.0 {
        close
            .water
            .0
            .min(close.height.0 + close.water.0 - me.height.0 - me.water.0)
    } else {
        -me.water
            .0
            .min(-close.height.0 - close.water.0 + me.height.0 + me.water.0)
    }
}

fn update_water(delta: f32, neighborhood: &Neighborhood) -> Water {
    let diff_up = water_diff(neighborhood.me, neighborhood.up);
    let diff_down = water_diff(neighborhood.me, neighborhood.down);
    let diff_left = water_diff(neighborhood.me, neighborhood.left);
    let diff_right = water_diff(neighborhood.me, neighborhood.right);

    Water(neighborhood.me.water.0 + delta * (diff_up + diff_down + diff_left + diff_right))
}

fn update_heat(delta: f32, neighborhood: &Neighborhood) -> Heat {
    Heat(0.0)
}

fn update_resources(delta: f32, neighborhood: &Neighborhood) -> Resources {
    Resources(0.0)
}
