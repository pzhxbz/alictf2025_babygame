use std::{sync::Arc, time::Duration};

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformScaleLens},
    Animator, EaseFunction, Tracks, Tween,
};
use obfstr::obfstmt;

use crate::{
    aspect::icon::{DEFAULT_ICON_POSITION, HIGHLIGHTED_ICON_POSITION},
    audio::PlaySound,
    player::input::PlayerInput,
    GameAssets, GameState,
};

use super::{
    icon::icon_texture,
    socket::{CombinerIcon, Socket},
    AspectCombiner, Number,
};

#[derive(Event)]
pub struct CombinedAspect;

#[derive(Resource, Default, Debug)]
pub struct Combiner {
    pub left_aspect: Option<Number>,
    pub right_aspect: Option<Number>,
    pub current_combination: Option<Number>,
    pub last_combined_aspect: Number,
    pub all_sockets_full: bool,
    pub flag_vec: Vec<u8>,
    pub flag_vec2: Vec<u32>,
    pub flag_vec3: Vec<u32>,
    pub time1: u64,
    pub status: u32,
    pub counter: Arc<parking_lot::Mutex<u32>>,
}

pub fn is_socket_combination_possible(combiner: &Res<Combiner>, _socket: &Socket) -> bool {
    // RELEASE NEED
    if combiner.flag_vec.len() >= 20 {
        false
    } else {
        true
    }
    // true
}

pub fn aspect_combinations(left_aspect: &Number, right_aspect: &Number) -> Number {
    fn match_aspects(left_aspect: &Number, right_aspect: &Number) -> Number {
        let left = left_aspect.to_int();
        let right = right_aspect.to_int();
        Number::Combine(left * 10 + right)
    }

    match_aspects(left_aspect, right_aspect)
}

fn select_aspects(
    assets: Res<GameAssets>,
    player_input: Res<PlayerInput>,
    mut combiner: ResMut<Combiner>,
    q_sockets: Query<(&TextureAtlas, &Socket)>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    if !player_input.select_socket {
        return;
    }

    let mut left_aspect = combiner.left_aspect;
    let mut right_aspect = combiner.right_aspect;
    for (atlas, socket) in &q_sockets {
        if atlas.index == 0 {
            continue;
        }

        if socket.on_top {
            left_aspect = if combiner.left_aspect != Some(socket.aspect) {
                ev_play_sound.send(PlaySound {
                    clip: assets.select_aspect.clone(),
                    ..default()
                });
                Some(socket.aspect)
            } else {
                ev_play_sound.send(PlaySound {
                    clip: assets.deselect_aspect.clone(),
                    ..default()
                });
                None
            };
        } else {
            right_aspect = if combiner.right_aspect != Some(socket.aspect) {
                ev_play_sound.send(PlaySound {
                    clip: assets.select_aspect.clone(),
                    ..default()
                });
                Some(socket.aspect)
            } else {
                ev_play_sound.send(PlaySound {
                    clip: assets.deselect_aspect.clone(),
                    ..default()
                });
                None
            };
        }
    }

    combiner.left_aspect = left_aspect;
    combiner.right_aspect = right_aspect;
}

fn show_combiner_icon(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut combiner: ResMut<Combiner>,
    mut q_combiner_icon: Query<(Entity, &mut Handle<Image>), With<CombinerIcon>>,
    mut visible: Local<bool>,
) {
    let (entity, mut texture) = match q_combiner_icon.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if let (Some(left_aspect), Some(right_aspect)) = (combiner.left_aspect, combiner.right_aspect) {
        let combined_aspect = aspect_combinations(&left_aspect, &right_aspect);
        combiner.current_combination = Some(combined_aspect);
        *texture = icon_texture(&assets, &combined_aspect);

        if !*visible {
            *visible = true;

            let seq = Tracks::new([
                Tween::new(
                    EaseFunction::QuarticOut,
                    Duration::from_secs_f32(0.2),
                    TransformPositionLens {
                        start: DEFAULT_ICON_POSITION.extend(0.0),
                        end: HIGHLIGHTED_ICON_POSITION.extend(0.0),
                    },
                ),
                Tween::new(
                    EaseFunction::QuarticOut,
                    Duration::from_secs_f32(0.2),
                    TransformScaleLens {
                        start: Vec3::splat(0.0),
                        end: Vec3::splat(1.0),
                    },
                ),
            ]);
            commands.entity(entity).insert(Animator::new(seq));
        }
    } else if *visible {
        *visible = false;

        let seq = Tracks::new([
            Tween::new(
                EaseFunction::QuarticOut,
                Duration::from_secs_f32(0.2),
                TransformPositionLens {
                    start: HIGHLIGHTED_ICON_POSITION.extend(0.0),
                    end: DEFAULT_ICON_POSITION.extend(0.0),
                },
            ),
            Tween::new(
                EaseFunction::QuarticOut,
                Duration::from_secs_f32(0.2),
                TransformScaleLens {
                    start: Vec3::splat(1.0),
                    end: Vec3::splat(0.0),
                },
            ),
        ]);
        commands.entity(entity).insert(Animator::new(seq));
    }
}

fn select_combined_aspect(
    player_input: Res<PlayerInput>,
    mut combiner: ResMut<Combiner>,
    q_combiner: Query<&TextureAtlas, With<AspectCombiner>>,
    mut ev_combined_aspect: EventWriter<CombinedAspect>,
) {
    if !player_input.select_socket {
        return;
    }
    let atlas = match q_combiner.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if atlas.index != 1 {
        return;
    }

    let (left_aspect, right_aspect) =
        if let (Some(l_aspect), Some(r_aspect)) = (combiner.left_aspect, combiner.right_aspect) {
            (l_aspect, r_aspect)
        } else {
            return;
        };

    let combined_aspect = aspect_combinations(&left_aspect, &right_aspect);

    combiner.last_combined_aspect = combined_aspect;
    combiner.left_aspect = None;
    combiner.right_aspect = None;
    ev_combined_aspect.send(CombinedAspect);
}

fn check_all_aspects_full(mut combiner: ResMut<Combiner>, _q_sockets: Query<&Socket>) {
    combiner.all_sockets_full = combiner.flag_vec.len() >= 64;
    if combiner.all_sockets_full && combiner.flag_vec2.is_empty() {
        let mut flag2 = vec![];
        for i in 0..16 {
            let t = u32::from_le_bytes([
                combiner.flag_vec[i * 4],
                combiner.flag_vec[i * 4 + 1],
                combiner.flag_vec[i * 4 + 2],
                combiner.flag_vec[i * 4 + 3],
            ]);
            flag2.push(t);
        }

        let k = vec![0xaf657662, 0xfc6f144b, 0x22ab2b6c, 0x367d2dcb];
        encrypt_xx(&mut flag2, &k);
        // println!("flag_last0 {:?}", flag2);
        combiner.flag_vec2 = flag2;
    }
}
#[inline(always)]
fn mx(sum: u32, y: u32, z: u32, p: u32, e: u32, k: &Vec<u32>) -> u32 {
    ((z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4))
        ^ ((sum ^ y).wrapping_add(k[(p & 3 ^ e) as usize] ^ z))
}

#[inline(always)]
fn encrypt_xx(v: &mut Vec<u32>, k: &Vec<u32>) -> Vec<u32> {
    const DELTA: u32 = 0x6bc6121d;

    let length: u32 = v.len() as u32;
    let n: u32 = length - 1;
    let key: Vec<u32> = k.to_vec();
    let mut e: u32 = 0;
    let mut y: u32 = 0;
    let mut z = v[n as usize];
    let mut sum: u32 = 0;
    let mut q: u32 = 6 + 52 / length;
    while q > 0 {
        obfstmt! {
            sum = sum.wrapping_add(DELTA);
            e = sum >> 2 & 3;
        }
        for p in 0..n {
            obfstmt! {
                y = v[(p as usize) + 1];
                v[p as usize] = v[p as usize].wrapping_add(mx(sum, y, z, p as u32, e, &key));
                z = v[p as usize];
            }
        }
        obfstmt! {
            y = v[0];
            v[n as usize] = v[n as usize].wrapping_add(mx(sum, y, z, n, e, &key));
            z = v[n as usize];
            q = q - 1;
        }
    }
    return v.clone();
}
pub struct AspectCombinerPlugin;

impl Plugin for AspectCombinerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                select_aspects,
                show_combiner_icon,
                select_combined_aspect,
                check_all_aspects_full,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<Combiner>()
        .add_event::<CombinedAspect>();
    }
}
