use biome;
#[allow(dead_code)]
use utils;
use vec;
use vec::Vector;
use world;

#[derive(Copy, Clone)]
pub struct Height(pub f32);

#[derive(Copy, Clone)]
pub struct Gradient(pub vec::Vec2f);

#[derive(Copy, Clone)]
pub struct AirPressure(pub f32);

#[derive(Copy, Clone)]
pub struct Wind(pub vec::Vec2f);

#[derive(Copy, Clone)]
pub struct Water(pub f32);

#[derive(Copy, Clone)]
pub struct Heat(pub f32);

#[derive(Copy, Clone)]
pub struct Resources(pub f32);

pub struct Neighborhood {
    pub up: CellProperties,
    pub down: CellProperties,
    pub left: CellProperties,
    pub right: CellProperties,
    pub me: CellProperties,
}

impl Neighborhood {
    pub fn get_gradient(&self) -> vec::Vec2f {
        vec::Vec2f::new(
            (self.right.height.0 - self.left.height.0) / 2.0,
            (self.up.height.0 - self.down.height.0) / 2.0,
        )
    }
}

pub struct Cell {
    pub properties: CellProperties,
    pub biome_tags: biome::BiomeTags,
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new(description: &world::WorldDescription, x: u32, y: u32) -> Cell {
        let properties = CellProperties::new(description, x, y);
        Cell {
            properties: properties,
            biome_tags: biome::tag_cell(&properties),
            x: x,
            y: y,
        }
    }

    pub fn update(&self, delta: f32, neighborhood: &Neighborhood) -> Cell {
        let new_props = CellProperties::step(&self.properties, delta, neighborhood);
        Cell {
            properties: new_props,
            biome_tags: biome::tag_cell(&new_props),
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Copy, Clone)]
pub struct CellProperties {
    pub height: Height,
    pub gradient: Gradient,
    pub air_pressure: AirPressure,
    pub wind: Wind,
    pub water: Water,
    pub heat: Heat,
    pub resources: Resources,
}

impl CellProperties {
    fn new(description: &world::WorldDescription, x: u32, y: u32) -> CellProperties {
        let waterlevel =
            (description.waterlevel.get(x, y) - 0.1 - description.heightmap.get(x, y)).max(0.0);
        CellProperties {
            height: Height(description.heightmap.get(x, y)),
            gradient: Gradient(vec::Vec2f::new(0.0, 0.0)),
            air_pressure: AirPressure(1.0),
            wind: Wind(vec::Vec2f::new(0.0, 0.0)),
            water: Water(waterlevel),
            heat: Heat(description.heightmap.get(x, y)),
            resources: Resources(0.0),
        }
    }

    pub fn total_height(&self) -> f32 {
        self.height.0 + self.water.0
    }

    fn step(current: &CellProperties, delta: f32, neighborhood: &Neighborhood) -> CellProperties {
        CellProperties {
            height: current.height,
            gradient: Gradient(neighborhood.get_gradient()),
            air_pressure: update_air_pressure(delta, neighborhood),
            wind: update_wind(delta, neighborhood),
            water: update_water(delta, neighborhood),
            heat: update_heat(delta, neighborhood),
            resources: update_resources(delta, neighborhood),
        }
    }

    pub fn alter_properties(
        &self,
        d_water: f32,
        d_air_pressure: f32,
        d_height: f32,
    ) -> CellProperties {
        CellProperties {
            water: Water((self.water.0 + d_water).max(0.0)),
            air_pressure: AirPressure((self.air_pressure.0 + d_air_pressure).max(0.0)),
            height: Height((self.height.0 + d_height).min(1.0).max(-1.0)),
            ..*self
        }
    }
}

fn update_air_pressure(delta: f32, neighborhood: &Neighborhood) -> AirPressure {
    let air_propagation_factor = 0.5;

    let diff_down = neighborhood.down.wind.0.get(1);
    let diff_up = -neighborhood.up.wind.0.get(1);
    let diff_left = neighborhood.left.wind.0.get(0);
    let diff_right = -neighborhood.right.wind.0.get(0);

    AirPressure(
        neighborhood.me.air_pressure.0
            + delta * air_propagation_factor * (diff_down + diff_up + diff_left + diff_right),
    )
}

fn air_pressure_diff(me: CellProperties, close: CellProperties) -> f32 {
    let gravity_factor = 0.5;

    if close.total_height() * gravity_factor + close.air_pressure.0
        > me.total_height() * gravity_factor + me.air_pressure.0
    {
        close.air_pressure.0.min(
            (close.total_height() * gravity_factor + close.air_pressure.0
                - me.total_height() * gravity_factor
                - me.air_pressure.0)
                .max(0.0),
        )
    } else {
        -me.air_pressure.0.min(
            (-close.total_height() * gravity_factor - close.air_pressure.0
                + me.total_height() * gravity_factor
                + me.air_pressure.0)
                .max(0.0),
        )
    }
}

fn update_wind(delta: f32, neighborhood: &Neighborhood) -> Wind {
    let wind_propagation_factor = 0.25;

    let diff_up = air_pressure_diff(neighborhood.me, neighborhood.up);
    let diff_down = air_pressure_diff(neighborhood.me, neighborhood.down);
    let diff_left = air_pressure_diff(neighborhood.me, neighborhood.left);
    let diff_right = air_pressure_diff(neighborhood.me, neighborhood.right);

    let (current_x, current_y) = neighborhood.me.wind.0.xy();

    Wind(vec::Vec2f::new(
        current_x + delta * wind_propagation_factor * (diff_left - diff_right - current_x),
        current_y + delta * wind_propagation_factor * (diff_down - diff_up - current_y),
    ))
}

fn water_diff(me: CellProperties, close: CellProperties, wind: f32) -> f32 {
    let wind_factor = 0.1;

    if close.total_height() + wind_factor * wind > me.total_height() {
        close
            .water
            .0
            .min((close.total_height() + wind_factor * wind - me.total_height()).max(0.0))
    } else {
        -me.water
            .0
            .min((-close.total_height() - wind_factor * wind + me.total_height()).max(0.0))
    }
}

fn update_water(delta: f32, neighborhood: &Neighborhood) -> Water {
    let water_propagation_factor = 1.0;

    let (wind_x, wind_y) = neighborhood.me.wind.0.xy();

    let diff_up = water_diff(
        neighborhood.me,
        neighborhood.up,
        -neighborhood.up.wind.0.xy().1 - wind_y,
    );
    let diff_down = water_diff(
        neighborhood.me,
        neighborhood.down,
        *neighborhood.down.wind.0.xy().1 + wind_y,
    );
    let diff_left = water_diff(
        neighborhood.me,
        neighborhood.left,
        *neighborhood.left.wind.0.xy().0 + wind_x,
    );
    let diff_right = water_diff(
        neighborhood.me,
        neighborhood.right,
        -neighborhood.right.wind.0.xy().0 - wind_x,
    );

    Water(
        (neighborhood.me.water.0
            + delta * water_propagation_factor * (diff_up + diff_down + diff_left + diff_right))
            .max(0.0),
    )
}

fn update_heat(delta: f32, neighborhood: &Neighborhood) -> Heat {
    let heat_propagation_factor = 1.0;
    let gravity_factor = 0.2;
    let wind_factor = 1.0;

    let diff_down = neighborhood.down.heat.0 + neighborhood.down.total_height() * gravity_factor;
    let diff_up = neighborhood.up.heat.0 + neighborhood.up.total_height() * gravity_factor;
    let diff_left = neighborhood.left.heat.0 + neighborhood.left.total_height() * gravity_factor;
    let diff_right = neighborhood.right.heat.0 + neighborhood.right.total_height() * gravity_factor;

    Heat(
        neighborhood.me.heat.0
            + delta
                * heat_propagation_factor
                * (diff_down + diff_up + diff_left + diff_right
                    - 4.0
                        * (neighborhood.me.heat.0
                            + neighborhood.me.total_height() * gravity_factor)),
    )
}

fn update_resources(delta: f32, neighborhood: &Neighborhood) -> Resources {
    Resources(0.0)
}
