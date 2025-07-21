use bevy::prelude::*;

#[derive(Default, Reflect, Clone)]
#[reflect(Default)]
pub enum WallType {
    #[default]
    Stone,
}

impl std::fmt::Display for WallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallType::Stone => write!(f, "Stone"),
        }
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component, Default)]
pub struct WallProperties {
    pub name: String,
    pub wall_type: WallType,
}
