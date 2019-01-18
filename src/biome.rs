use cell;
use vec;

#[derive(Copy, Clone, PartialEq)]
pub enum BiomeType {
    Lake,
    Forest,
    Mountain,
    Wetland,
    Nob,
}

#[derive(Copy, Clone)]
pub struct BiomeTags {
    wet: bool,
    water: bool,
    high: bool,
    flat: bool,
    steep: bool,
}

pub fn tag_cell(cell: &cell::CellProperties) -> BiomeTags {
    let cell::CellProperties {
        height: cell::Height(height),
        water: cell::Water(water),
        gradient: cell::Gradient(gradient),
        ..
    } = cell;
    BiomeTags {
        wet: *water > 0.001,
        water: *water > 0.05,
        high: *height > 0.3,
        flat: vec::len(gradient) < 0.08,
        steep: vec::len(gradient) > 0.1,
    }
}

pub fn classify_tags(tags: BiomeTags) -> BiomeType {
    match (tags.wet, tags.water, tags.high, tags.flat, tags.steep) {
        (_, true, _, _, _) => BiomeType::Lake,
        (_, _, true, _, _) => BiomeType::Mountain,
        (true, _, _, _, _) => BiomeType::Wetland,
        (_, _, false, true, _) => BiomeType::Forest,
        _ => BiomeType::Nob,
    }
}

//pub fn classify_cell(cell: cell::CellProperties) -> BiomeType {
//    let cell::CellProperties {
//        height: cell::Height(height),
//        water: cell::Water(water),
//        gradient: cell::Gradient(gradient),
//        ..
//    } = cell;
//
//    match (height, water, vec::len(&gradient)) {
//        (_, w, _) if w > 0.05 => BiomeType::Lake,
//        (h, _, _) if h > 0.3 => BiomeType::Mountain,
//        (_, w, _) if w > 0.001 => BiomeType::Wetland,
//        (h, _, g) if h < 0.2 && g < 0.08 => BiomeType::Forest,
//        _ => BiomeType::Nob,
//    }
//}

pub fn tmp_colors(biome: BiomeType) -> (u8, u8, u8) {
    match biome {
        BiomeType::Mountain => (255, 0, 0),
        BiomeType::Lake => (0, 0, 255),
        BiomeType::Wetland => (0, 100, 0),
        BiomeType::Forest => (0, 255, 0),
        _ => (0, 0, 0),
    }
}

pub fn update_biomes(delta: f32, mut cells: Vec<cell::Cell>) -> Vec<cell::Cell> {
    let evaporation_factor = 0.000005;
    let mut water_vapor = 0.0;
    let mut n_mountain = 0;
    for mut cell in cells.iter_mut() {
        match classify_tags(cell.biome_tags) {
            BiomeType::Lake => {
                cell.properties =
                    cell.properties
                        .alter_properties(-evaporation_factor * delta, 0.0, 0.0);
                water_vapor += evaporation_factor * delta;
            }
            BiomeType::Mountain => {
                n_mountain += 1;
            }
            _ => (),
        };
    }
    let rain = water_vapor / n_mountain as f32;
    for mut cell in cells.iter_mut() {
        match classify_tags(cell.biome_tags) {
            BiomeType::Mountain => {
                cell.properties = cell.properties.alter_properties(rain, 0.0, 0.0);
                water_vapor += evaporation_factor * delta;
            }
            _ => (),
        };
    }

    cells
}
