#[allow(dead_code)]
use utils;
use vec;
use vec::Vector;
use world;

#[derive(Copy, Clone)]
pub struct Gradient(vec::Vec2f);

#[derive(Copy, Clone)]
pub struct Height(f32);

#[derive(Copy, Clone)]
pub struct AirPressure(f32);

#[derive(Copy, Clone)]
pub struct Wind(vec::Vec2f);

#[derive(Copy, Clone)]
pub struct Water(f32);

#[derive(Copy, Clone)]
pub struct Heat(f32);

#[derive(Copy, Clone)]
pub struct Resources(f32);

#[derive(Copy, Clone)]
pub struct CellProperties {
    gradient: Gradient,
    height: Height,
    air_pressure: AirPressure,
    wind: Wind,
    water: Water,
    heat: Heat,
    resources: Resources,
}

impl CellProperties {
    fn new(description: &world::WorldDescription, x: u32, y: u32) -> CellProperties {
        let mut waterlevel =
            description.waterlevel.get(x, y) - 0.1 - description.heightmap.get(x, y);
        if waterlevel < 0.0 {
            waterlevel = 0.0;
        }
        CellProperties {
            gradient: Gradient(description.heightmap.get_gradient(x, y)),
            height: Height(description.heightmap.get(x, y)),
            air_pressure: AirPressure(description.windmap.get(x, y)),
            wind: Wind(description.windmap.get_gradient(x, y)), //Wind(vec::Vec2f::new(0.0, 0.0)),
            water: Water(waterlevel),
            heat: Heat(0.0),
            resources: Resources(0.0),
        }
    }

    fn total_height(&self) -> f32 {
        self.height.0 + self.water.0
    }

    fn step(current: &CellProperties, delta: f32, neighborhood: &Neighborhood) -> CellProperties {
        CellProperties {
            gradient: current.gradient,
            height: current.height,
            air_pressure: update_air_pressure(delta, neighborhood),
            wind: update_wind(delta, neighborhood),
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

    pub fn to_wind_pixel(&self) -> (u8, u8, u8) {
        let wind = vec::normalize(&self.properties.wind.0);
        let r = wind.xy().0 * 0.5 + 0.5;
        let g = wind.xy().1 * 0.5 + 0.5;

        ((r * 255.0) as u8, (g * 255.0) as u8, 0)
    }
}

fn update_air_pressure(delta: f32, neighborhood: &Neighborhood) -> AirPressure {
    neighborhood.me.air_pressure
}

fn update_wind(delta: f32, neighborhood: &Neighborhood) -> Wind {
    let diff_left = -neighborhood.me.wind.0.get(0) + neighborhood.left.wind.0.get(0);
    let diff_right = -neighborhood.me.wind.0.get(0) + neighborhood.right.wind.0.get(0);

    let diff_up = -neighborhood.me.wind.0.get(1) + neighborhood.up.wind.0.get(1);
    let diff_down = -neighborhood.me.wind.0.get(1) + neighborhood.down.wind.0.get(1);

    Wind(vec::Vec2f::new(
        neighborhood.me.wind.0.get(0) + (diff_left + diff_right) * delta,
        neighborhood.me.wind.0.get(1) + (diff_up + diff_down) * delta,
    ))
}

fn water_diff(me: CellProperties, close: CellProperties) -> f32 {
    if close.total_height() > me.total_height() {
        close.water.0.min(close.total_height() - me.total_height())
    } else {
        -me.water.0.min(-close.total_height() + me.total_height())
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
