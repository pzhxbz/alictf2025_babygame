use bevy::prelude::*;
use bevy_tweening::{lens::*, *};

use crate::GameState;

const FADE_OUT_DURATION: f32 = 3.0;
const FADE_IN_DURATION: f32 = 2.0;
const BLACK_VISIBLE: Color = Color::srgba(0.0, 0.0, 0.0, 1.0);
const BLACK_TRANSPARENT: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

fn fade_out_black_screen(mut commands: Commands) {
    let tween = Tween::new(
        EaseFunction::CubicIn,
        std::time::Duration::from_secs_f32(FADE_OUT_DURATION),
        UiBackgroundColorLens {
            start: BLACK_VISIBLE,
            end: BLACK_TRANSPARENT,
        },
    );

    commands.spawn((
        Animator::new(tween),
        ImageBundle {
            style: Style {
                width: Val::Vw(110.0),
                height: Val::Vh(110.0),
                ..default()
            },
            ..default()
        },
    ));
}

fn fade_in_black_screen(mut commands: Commands) {
    let tween = Tween::new(
        EaseFunction::CubicIn,
        std::time::Duration::from_secs_f32(FADE_IN_DURATION),
        UiBackgroundColorLens {
            start: BLACK_TRANSPARENT,
            end: BLACK_VISIBLE,
        },
    );

    commands.spawn((
        Animator::new(tween),
        ImageBundle {
            style: Style {
                width: Val::Vw(110.0),
                height: Val::Vh(110.0),
                ..default()
            },
            ..default()
        },
    ));
}

pub struct ScreenFadePlugin;

impl Plugin for ScreenFadePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Intro), (fade_out_black_screen,))
            .add_systems(OnEnter(GameState::Ending), (fade_in_black_screen,));
    }
}
