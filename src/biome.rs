use cell;
use vec;

pub enum BiomeType {
    Lake,
    Forest,
    Mountain,
    Wetland,
    Nob,
}

pub fn classify_cell(cell: cell::CellProperties) -> BiomeType {
    let cell::CellProperties {
        height: cell::Height(height),
        water: cell::Water(water),
        gradient: cell::Gradient(gradient),
        ..
    } = cell;

    match (height, water, vec::len(&gradient)) {
        (_, w, _) if w > 0.05 => BiomeType::Lake,
        (_, w, _) if w > 0.001 => BiomeType::Wetland,
        (h, _, _) if h > 0.3 => BiomeType::Mountain,
        (h, _, g) if h < 0.2 && g < 0.08 => BiomeType::Forest,
        _ => BiomeType::Nob,
    }
}

pub fn tmp_colors(biome: BiomeType) -> (u8, u8, u8) {
    match biome {
        BiomeType::Mountain => (255, 0, 0),
        BiomeType::Lake => (0, 0, 255),
        BiomeType::Wetland => (0, 100, 0),
        BiomeType::Forest => (0, 255, 0),
        _ => (0, 0, 0),
    }
}
