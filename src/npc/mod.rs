pub mod narrator;

use std::str::FromStr;

use obfstr::obfstr;
use strum_macros::Display;

use bevy::prelude::*;

use crate::GameAssets;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(narrator::NarratorPlugin);
    }
}

#[derive(Clone, Copy, Display, PartialEq, Default)]
pub enum NpcDialogue {
    #[default]
    Ami,
    Ima,
}

impl FromStr for NpcDialogue {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == obfstr!("奇怪的人") || s == obfstr!("Ima") {
            Ok(NpcDialogue::Ima)
        } else if s == obfstr!("你自己") || s == obfstr!("Ami") {
            Ok(NpcDialogue::Ami)
        } else {
            Err(strum::ParseError::VariantNotFound)
        }
    }
}

pub fn npc_character_icon(assets: &Res<GameAssets>, npc: &NpcDialogue) -> Handle<Image> {
    match npc {
        NpcDialogue::Ami => assets.ami_character_icon.clone(),
        NpcDialogue::Ima => assets.ima_character_icon.clone(),
    }
}
