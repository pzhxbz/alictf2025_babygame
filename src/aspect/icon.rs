use bevy::{
    color::palettes::css::{GRAY, WHITE},
    prelude::*,
};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween};

use crate::{world::camera::YSortChild, GameAssets};

use super::{
    combiner::Combiner,
    socket::{AspectIcon, Socket},
    Number,
};

pub const DEFAULT_ICON_POSITION: Vec2 = Vec2::new(0.0, 16.0);
pub const HIGHLIGHTED_ICON_POSITION: Vec2 = Vec2::new(0.0, 24.0);
const DEHIGHLIGHTED_ICON_POSITION: Vec2 = Vec2::new(0.0, 8.0);
const REPOSITION_TIME: f32 = 0.2;

pub fn icon_texture(assets: &Res<GameAssets>, aspect: &Number) -> Handle<Image> {
    match aspect {
        Number::Zero => assets.number0.clone(),
        Number::One => assets.number1.clone(),
        Number::Two => assets.number2.clone(),
        Number::Three => assets.number3.clone(),
        Number::Four => assets.number4.clone(),
        Number::Five => assets.number5.clone(),
        Number::Six => assets.number6.clone(),
        Number::Seven => assets.number7.clone(),
        Number::Eight => assets.number8.clone(),
        Number::Nine => assets.number9.clone(),
        Number::Combine(_) => assets.combine.clone(),
    }
}

fn set_icon_properties(
    commands: &mut Commands,
    q_icons: &mut Query<(Entity, &Transform, &mut Sprite), With<AspectIcon>>,
    children: &Children,
    pos: Vec2,
    tint: Color,
) {
    for child in children.iter() {
        let (entity, transform, mut sprite) = match q_icons.get_mut(*child) {
            Ok(r) => r,
            Err(_) => continue,
        };

        sprite.color = tint;

        let tween = Tween::new(
            EaseFunction::CubicOut,
            std::time::Duration::from_secs_f32(REPOSITION_TIME),
            TransformPositionLens {
                start: transform.translation,
                end: pos.extend(0.0),
            },
        );
        commands.entity(entity).insert(Animator::new(tween));
    }
}

fn set_icons_properties(
    mut commands: Commands,
    combiner: Res<Combiner>,
    q_sockets: Query<(&Children, &Socket)>,
    mut q_icons: Query<(Entity, &Transform, &mut Sprite), With<AspectIcon>>,
) {
    for (children, socket) in &q_sockets {
        let (pos, tint) = if combiner.all_sockets_full
            || socket.on_top
                && combiner.left_aspect.is_some()
                && combiner.left_aspect != Some(socket.aspect)
            || !socket.on_top
                && combiner.right_aspect.is_some()
                && combiner.right_aspect != Some(socket.aspect)
        {
            (DEHIGHLIGHTED_ICON_POSITION, GRAY)
        } else if socket.on_top && combiner.left_aspect == Some(socket.aspect)
            || !socket.on_top && combiner.right_aspect == Some(socket.aspect)
        {
            (HIGHLIGHTED_ICON_POSITION, WHITE)
        } else {
            (DEFAULT_ICON_POSITION, WHITE)
        };
        set_icon_properties(&mut commands, &mut q_icons, children, pos, tint.into());
    }
}

fn update_icon_ysorts(mut q_icons: Query<(&Transform, &mut YSortChild), With<AspectIcon>>) {
    for (transform, mut ysort) in &mut q_icons {
        *ysort = YSortChild(transform.translation.y + 1.0);
    }
}

pub struct AspectIconPlugin;

impl Plugin for AspectIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (set_icons_properties, update_icon_ysorts));
    }
}
