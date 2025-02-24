use std::time::{SystemTime, UNIX_EPOCH};

use bevy_yarnspinner::events::DialogueCompleteEvent;
use obfstr::obfstmt;
use strum_macros::{Display, EnumIter, EnumString};

use bevy::prelude::*;

use crate::{
    aspect::{Combiner, Socket},
    GameState,
};

const START_DELAY: f32 = 1.0;
// const ENDING_DELAY: f32 = 2.0;
// const GOOD_ENDING_THRESHOLD: i32 = 7;

#[derive(Reflect, Clone, PartialEq, EnumString, Display, Debug, Copy, EnumIter)]
pub enum NarratorDialogue {
    Intro,
    GoodEnding,
    BadEnding,
    CheatEnding,
}

#[derive(Event)]
pub struct TriggeredNarratorDialogue(pub NarratorDialogue);

fn determine_ending(_q_sockets: &Query<&Socket>, combiner: Res<Combiner>) -> NarratorDialogue {
    let time_bool = {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap_or_default();
        let sec = since_the_epoch.as_secs();
        sec - combiner.time1 < 60 || sec - combiner.time1 > 3600
    };

    let counter = *combiner.counter.lock();

    let mut k = vec![0u32; 4];

    if time_bool || counter < 3000 {
        return NarratorDialogue::CheatEnding;
    }

    obfstmt! {
        k[0] = 0x41661f49;
        k[1] = 0xdfc12fcf;
        k[2] = 0x1fe0f1a2;
        k[3] = 0x71168786;
    }

    let mut flag_last = combiner.flag_vec3.clone();
    // println!("flag_last2 {:?}", flag_last);
    encrypt_xx(&mut flag_last, &k);

    let final_res = vec![
        3281827746, 3820286646, 1435952682, 2903429368, 3014570085, 1231715997, 1091464774,
        49750581, 299271759, 2410055363, 1929249756, 195027653, 2148093411, 4137854613, 2846676883,
        258032211,
    ];

    // println!("flag_last3 {:?}", flag_last);
    if flag_last != final_res {
        return NarratorDialogue::BadEnding;
    }
    NarratorDialogue::GoodEnding
}
#[inline(always)]
fn mx(sum: u32, y: u32, z: u32, p: u32, e: u32, k: &Vec<u32>) -> u32 {
    ((z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4))
        ^ ((sum ^ y).wrapping_add(k[(p & 3 ^ e) as usize] ^ z))
}
#[inline(always)]
fn encrypt_xx(v: &mut Vec<u32>, k: &Vec<u32>) -> Vec<u32> {
    const DELTA: u32 = 0x98d846dc;

    for i in 0..v.len() {
        obfstmt! {
            v[i] ^= 0x42e2b468;
        }
    }

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

    for i in 0..v.len() {
        obfstmt! {
            v[i] ^= 0x71f28b88;
        }
    }

    return v.clone();
}

fn trigger_intro_dialogue(
    time: Res<Time>,
    mut ev_triggered_narrator_dialogue: EventWriter<TriggeredNarratorDialogue>,
    mut elapsed: Local<f32>,
) {
    if *elapsed > START_DELAY {
        return;
    }
    *elapsed += time.delta_seconds();

    if *elapsed > START_DELAY {
        ev_triggered_narrator_dialogue.send(TriggeredNarratorDialogue(NarratorDialogue::Intro));
    }
}

fn transition_to_gaming_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Gaming);
}

fn trigger_ending_dialogue(
    time: Res<Time>,
    q_sockets: Query<&Socket>,
    mut ev_triggered_narrator_dialogue: EventWriter<TriggeredNarratorDialogue>,
    mut elapsed: Local<f32>,
    combiner: Res<Combiner>,
) {
    const ENDING_DELAY: f32 = 2.0;
    if *elapsed > ENDING_DELAY {
        return;
    }
    *elapsed += time.delta_seconds();

    if *elapsed > ENDING_DELAY {
        ev_triggered_narrator_dialogue.send(TriggeredNarratorDialogue(determine_ending(
            &q_sockets, combiner,
        )));
    }
}

pub struct NarratorPlugin;

impl Plugin for NarratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggeredNarratorDialogue>()
            .add_systems(
                Update,
                (
                    trigger_intro_dialogue,
                    transition_to_gaming_state.run_if(on_event::<DialogueCompleteEvent>()),
                )
                    .run_if(in_state(GameState::Intro)),
            )
            .add_systems(
                Update,
                trigger_ending_dialogue.run_if(in_state(GameState::Ending)),
            );
    }
}
