use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;
use bevy_yarnspinner::events::DialogueCompleteEvent;
use obfstr::obfstmt;

use crate::{
    aspect::{AspectCombiner, Combiner},
    player::{input::PlayerInput, Player, PLAYER_PIVOT},
    world::camera::YSort,
    GameAssets, GameState,
};

const PLAYER_HIGHLIGHT_DISTANCE: f32 = 32.0;
const COMBINER_OFFSET: Vec3 = Vec3::new(128.0, 0.0, 0.0);

#[derive(Component, Default)]
pub struct Bed;

#[derive(Event)]
pub struct PlayerWentToBed;

fn spawn_smoke_effect(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.smoke_animations[0].clone());

    commands.spawn((
        YSort(100.0),
        animator,
        SpriteBundle {
            texture: assets.smoke_texture.clone(),
            transform: Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        TextureAtlas {
            layout: assets.smoke_layout.clone(),
            ..default()
        },
    ));
}

fn spawn_bed(
    mut commands: Commands,
    assets: Res<GameAssets>,
    combiner: Res<Combiner>,
    q_combiner: Query<&Transform, With<AspectCombiner>>,
) {
    if !combiner.all_sockets_full {
        return;
    }

    let combiner_transform = match q_combiner.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let pos = combiner_transform.translation + COMBINER_OFFSET;

    spawn_smoke_effect(&mut commands, &assets, pos);

    commands.spawn((
        Bed,
        YSort(0.0),
        Collider::cuboid(16.0, 16.0),
        SpriteBundle {
            texture: assets.bed_texture.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        },
        TextureAtlas {
            layout: assets.bed_layout.clone(),
            ..default()
        },
    ));
}

fn highlight_and_select_bed(
    player_input: Res<PlayerInput>,
    q_player: Query<&Transform, With<Player>>,
    mut q_bed: Query<(&Transform, &mut TextureAtlas), (With<Bed>, Without<Player>)>,
    mut ev_player_went_to_bed: EventWriter<PlayerWentToBed>,
    mut selected: Local<bool>,
    mut combiner: ResMut<Combiner>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let (transform, mut atlas) = match q_bed.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let index = if !*selected
        && transform
            .translation
            .truncate()
            .distance_squared(player_transform.translation.truncate() + PLAYER_PIVOT)
            <= PLAYER_HIGHLIGHT_DISTANCE.powi(2)
    {
        if player_input.select_socket {
            *selected = true;
            // println!("enc2");
            let mut flag3 = combiner.flag_vec2.clone();
            let k = vec![0x9e51e580, 0xf4496000, 0x64168eed, 0x496e55bf];
            // println!("flag_last1 {:?}", flag3);
            encrypt_xx(&mut flag3, &k);
            combiner.flag_vec3 = flag3;

            ev_player_went_to_bed.send(PlayerWentToBed);
        }
        1
    } else {
        0
    };
    atlas.index = index;
}

pub struct MapBedPlugin;

impl Plugin for MapBedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_bed.run_if(on_event::<DialogueCompleteEvent>()),
                highlight_and_select_bed,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<PlayerWentToBed>();
    }
}
#[inline(always)]
fn mx(sum: u32, y: u32, z: u32, p: u32, e: u32, k: &Vec<u32>) -> u32 {
    ((z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4))
        ^ ((sum ^ y).wrapping_add(k[(p & 3 ^ e) as usize] ^ z))
}
#[inline(always)]
fn encrypt_xx(v: &mut Vec<u32>, k: &Vec<u32>) -> Vec<u32> {
    const DELTA: u32 = 0xb72908f9;

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
