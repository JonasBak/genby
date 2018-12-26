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
        let w = self.properties.water.0.min(1.0);
        let h = utils::map_range(
            0.0..2.0,
            0.0..255.0,
            (self.properties.height.0 + 1.0) * (1.0 - w),
        );
        return (h as u8, h as u8, if w > 0.1 { 255 } else { h as u8 });
    }

    pub fn to_wind_pixel(&self) -> (u8, u8, u8) {
        let (x, y) = self.properties.wind.0.xy();

        let wind_x = (x * 10.0).max(-1.0).min(1.0);
        let wind_y = (y * 10.0).max(-1.0).min(1.0);

        (
            (wind_x * 255.0 / 2.0 + 255.0 / 2.0) as u8,
            (wind_y * 255.0 / 2.0 + 255.0 / 2.0) as u8,
            0,
        )
    }

    pub fn to_airpressure_pixel(&self) -> (u8, u8, u8) {
        let mut pressure = self.properties.air_pressure.0 * 0.5;

        pressure = pressure.max(0.0).min(1.0);

        (
            (pressure * 255.0) as u8,
            (pressure * 255.0) as u8,
            (pressure * 255.0) as u8,
        )
    }
}

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
            air_pressure: AirPressure(description.windmap.get(x, y) + 1.0),
            wind: Wind(vec::Vec2f::new(0.0, 0.0)),
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

fn update_air_pressure(delta: f32, neighborhood: &Neighborhood) -> AirPressure {
    let diff_down = neighborhood.down.wind.0.get(1);
    let diff_up = -neighborhood.up.wind.0.get(1);
    let diff_left = neighborhood.left.wind.0.get(0);
    let diff_right = -neighborhood.right.wind.0.get(0);
    // let diff_down = neighborhood.down.air_pressure.0 - neighborhood.me.air_pressure.0;
    // let diff_up = neighborhood.up.air_pressure.0 - neighborhood.me.air_pressure.0;
    // let diff_left = neighborhood.left.air_pressure.0 - neighborhood.me.air_pressure.0;
    // let diff_right = neighborhood.right.air_pressure.0 - neighborhood.me.air_pressure.0;

    AirPressure(
        neighborhood.me.air_pressure.0
            + delta / 2.0 * (diff_down + diff_up + diff_left + diff_right),
    )
}

fn update_wind(delta: f32, neighborhood: &Neighborhood) -> Wind {
    let diff_down = neighborhood.down.air_pressure.0 - neighborhood.me.air_pressure.0;
    let diff_up = neighborhood.up.air_pressure.0 - neighborhood.me.air_pressure.0;
    let diff_left = neighborhood.left.air_pressure.0 - neighborhood.me.air_pressure.0;
    let diff_right = neighborhood.right.air_pressure.0 - neighborhood.me.air_pressure.0;

    let (current_x, current_y) = neighborhood.me.wind.0.xy();

    Wind(vec::Vec2f::new(
        current_x + delta * (diff_left - diff_right - current_x),
        current_y + delta * (diff_down - diff_up - current_y),
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
