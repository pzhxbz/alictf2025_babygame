mod combiner;
mod icon;
mod name_text;
mod socket;

use std::str::FromStr;

pub use combiner::{CombinedAspect, Combiner};
use obfstr::obfstr;
pub use socket::Socket;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use strum_macros::{Display, EnumIter, EnumString};

pub struct AspectPlugin;

impl Plugin for AspectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            socket::AspectSocketPlugin,
            combiner::AspectCombinerPlugin,
            icon::AspectIconPlugin,
            name_text::AspectNameTextPlugin,
        ))
        .register_ldtk_entity::<AspectBundle>(obfstr!("AspectSocket"))
        .register_ldtk_entity::<CombinerBundle>(obfstr!("CombinerSocket"));
    }
}

#[derive(Default, Reflect, Clone, PartialEq, EnumString, Display, Debug, Copy, EnumIter)]
pub enum Number {
    #[default]
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Combine(u32),
}

impl Number {
    pub fn to_int(&self) -> u32 {
        match self {
            Number::Zero => 0,
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
            Number::Combine(i) => *i,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Number::Zero => obfstr!("0").to_string(),
            Number::One => obfstr!("1").to_string(),
            Number::Two => obfstr!("2").to_string(),
            Number::Three => obfstr!("3").to_string(),
            Number::Four => obfstr!("4").to_string(),
            Number::Five => obfstr!("5").to_string(),
            Number::Six => obfstr!("6").to_string(),
            Number::Seven => obfstr!("7").to_string(),
            Number::Eight => obfstr!("8").to_string(),
            Number::Nine => obfstr!("9").to_string(),
            Number::Combine(i) => i.to_string(),
        }
    }
}

#[derive(Default, Component)]
pub struct AspectSocketInitiater {
    aspect: Number,
    on_top: bool,
}

impl AspectSocketInitiater {
    fn from_field(entity_instance: &EntityInstance) -> Self {
        let aspect = match entity_instance.get_enum_field(obfstr!("aspect")) {
            Ok(r) => Number::from_str(r).unwrap_or_default(),
            Err(_) => Number::default(),
        };
        let on_top = match entity_instance.get_bool_field(obfstr!("on_top")) {
            Ok(r) => r.to_owned(),
            Err(err) => {
                error!("{}{}", obfstr!("counld not find field, "), err);
                false
            }
        };
        Self { aspect, on_top }
    }
}

#[derive(Component, Default)]
pub struct AspectCombinerInitiater;

impl AspectCombinerInitiater {
    fn from_field(_entity_instance: &EntityInstance) -> Self {
        Self
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct AspectBundle {
    #[with(AspectSocketInitiater::from_field)]
    aspect_initiater: AspectSocketInitiater,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

#[derive(Default, Bundle, LdtkEntity)]
struct CombinerBundle {
    #[with(AspectCombinerInitiater::from_field)]
    aspect_combiner: AspectCombinerInitiater,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

#[derive(Component, Default)]
pub struct AspectCombiner;
