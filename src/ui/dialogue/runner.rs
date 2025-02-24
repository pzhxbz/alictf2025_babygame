use bevy::prelude::*;
use bevy_yarnspinner::{events::DialogueCompleteEvent, prelude::*};
use obfstr::{obfstmt, obfstr};
use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    aspect::{CombinedAspect, Combiner},
    npc::narrator::TriggeredNarratorDialogue,
    world::{PlayerWentToBed, TriggerFirstImaDialogue},
    GameState,
};

use super::{
    command::{trigger_ending_command, trigger_game_over_command},
    spawn::DialogueRoot,
};

const SHORT_INTRO_TIMEOUT: f32 = 3.5;
pub const IMA_FINAL_DIALOGUE: &str = "ImaFinalDialogue";
pub const IMA_FIRST_ENCOUNTER: &str = "ImaFirstEncounter";
#[allow(dead_code)]
pub const IMA_FIRST_ENCOUNTER_SHORT: &str = "ImaFirstEncounterShort";

#[derive(Resource)]
struct TimeSinceGaming(Timer);

impl Default for TimeSinceGaming {
    fn default() -> Self {
        Self(Timer::new(
            Duration::from_secs_f32(SHORT_INTRO_TIMEOUT),
            TimerMode::Once,
        ))
    }
}

#[derive(Component, Default)]
pub struct RunnerFlags {
    pub line: Option<LocalizedLine>,
}

fn my_pow(node: &str, counter: Arc<parking_lot::Mutex<u32>>, is_out: bool) -> String {
    let node_num = u32::from_str_radix(node, 10).unwrap_or(99);
    let base = format!(
        "{}{}{}\n",
        obfstr!("奇怪的人: "),
        if is_out {
            obfstr!("你已经输入了太多的数字，我觉得是时候发动魔法让你不能继续输入了").to_string()
        } else {
            "".to_string()
        },
        obfstr!(".").to_string().repeat(100)
    );
    {
        let mut c = counter.lock();
        *c += node_num;
    }
    // base.repeat(node_num as usize)
    base
}

fn spawn_runner(
    commands: &mut Commands,
    project: &Res<YarnProject>,
    node: &str,
    counter: Arc<parking_lot::Mutex<u32>>,
    is_out: bool,
) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner
        .commands_mut()
        .add_command(
            obfstr!("trigger_ending").to_string(),
            trigger_ending_command,
        )
        .add_command(obfstr!("game_over").to_string(), trigger_game_over_command);
    let v = dialogue_runner.variable_storage_mut();
    v.set(obfstr!("$haha").to_string(), node.into()).ok();
    let node2 = node.to_string();
    dialogue_runner
        .library_mut()
        .add_function(obfstr!("pow").to_string(), move || {
            my_pow(&node2, counter.clone(), is_out)
        });
    dialogue_runner.start_node(obfstr!("Combine").to_string());
    commands.spawn((dialogue_runner, RunnerFlags::default()));
}

#[inline(always)]
fn next_random(i: u32) -> u32 {
    let mut j = 0u128;
    let i = i as u128;
    obfstmt! {
        j = i * 1664525u128 + 1013904223;
        j = j ^ (j >> 16);
        j = j * 1664525u128 + 1013904223;
        j = j ^ (j >> 16);
    }
    j as u32
}

#[inline(always)]
fn calc_res(mut input: u32, i: u32) -> u8 {
    let mut xor1 = 0;
    let mut xor2 = 0;
    let mut xor3 = 0;
    let mut xor4 = 0;
    obfstmt! {
        xor1 = i & 0xff;
        xor2 = (i >> 8) & 0xff;
        xor3 = (i >> 16) & 0xff;
        xor4 = (i >> 24) & 0xff;
        input ^= xor1;
        input += xor2;
        input ^= xor3;
        input += xor4;
    }
    input as u8
}

fn spawn_dialogue_runner(
    mut commands: Commands,
    project: Res<YarnProject>,
    mut combiner: ResMut<Combiner>,
) {
    let c = combiner.counter.clone();
    spawn_runner(
        &mut commands,
        &project,
        &combiner.last_combined_aspect.to_str(),
        c,
        combiner.flag_vec.len() >= 19,
    );
    let last_u32 = combiner.last_combined_aspect.to_int();
    combiner.status = next_random(combiner.status);
    let status = combiner.status;

    if combiner.flag_vec.is_empty() {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap_or_default();
        combiner.time1 = since_the_epoch.as_secs();
    }

    // if last_u32 == 93 {
    //     let test_vec = vec![
    //         93, 34, 19, 34, 55, 54, 77, 77, 69, 96, 42, 94, 71, 91, 38, 1, 50, 54, 60, 81, 62, 31,
    //         27, 68, 9, 96, 29, 72, 75, 90, 14, 9, 91, 22, 93, 95, 67, 16, 82, 21, 79, 42, 96, 85,
    //         0, 95, 52, 11, 44, 85, 79, 86, 21, 52, 51, 31, 54, 20, 26, 11, 97, 49, 23, 65,
    //     ];
    //     // println!("test");
    //     let mut status = status;
    //     for i in test_vec {
    //         // print!("{} ", status);
    //         combiner.flag_vec.push(calc_res(i, status));
    //         status = next_random(status);
    //     }
    //     // println!("test1 {:?}", combiner.flag_vec);
    //     *combiner.counter.lock() = 4000;
    //     return;
    // }

    combiner.flag_vec.push(calc_res(last_u32, status));
}

fn spawn_runner_ori(commands: &mut Commands, project: &Res<YarnProject>, node: &str) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner
        .commands_mut()
        .add_command(
            obfstr!("trigger_ending").to_string(),
            trigger_ending_command,
        )
        .add_command(obfstr!("game_over").to_string(), trigger_game_over_command);
    dialogue_runner.start_node(node);
    commands.spawn((dialogue_runner, RunnerFlags::default()));
}

fn spawn_narrator_dialogue(
    mut commands: Commands,
    project: Res<YarnProject>,
    mut ev_triggered_narrator_dialogue: EventReader<TriggeredNarratorDialogue>,
) {
    for ev in ev_triggered_narrator_dialogue.read() {
        spawn_runner_ori(&mut commands, &project, &ev.0.to_string());
    }
}

fn spawn_ima_first_encounter(
    mut commands: Commands,
    _time_since_gaming: Res<TimeSinceGaming>,
    project: Res<YarnProject>,
) {
    let node: String = obfstr!(IMA_FIRST_ENCOUNTER).to_string();
    spawn_runner_ori(&mut commands, &project, &node);
}

fn spawn_ima_final_dialogue(mut commands: Commands, project: Res<YarnProject>) {
    spawn_runner_ori(&mut commands, &project, obfstr!(IMA_FINAL_DIALOGUE));
}

fn despawn_dialogue(
    mut commands: Commands,
    q_dialogue_root: Query<Entity, With<DialogueRoot>>,
    mut ev_dialogue_completed: EventReader<DialogueCompleteEvent>,
) {
    for ev in ev_dialogue_completed.read() {
        for entity in &q_dialogue_root {
            commands.entity(entity).despawn_recursive();
        }
        if let Some(r) = commands.get_entity(ev.source) {
            r.despawn_recursive();
        }
    }
}

fn tick_time_since_gaming(time: Res<Time>, mut time_since_gaming: ResMut<TimeSinceGaming>) {
    time_since_gaming.0.tick(time.delta());
}

pub struct DialogueRunnerPlugin;

impl Plugin for DialogueRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_dialogue_runner.run_if(on_event::<CombinedAspect>()),
                spawn_narrator_dialogue,
                spawn_ima_first_encounter.run_if(on_event::<TriggerFirstImaDialogue>()),
                spawn_ima_final_dialogue.run_if(on_event::<PlayerWentToBed>()),
                despawn_dialogue,
            )
                .run_if(not(in_state(GameState::AssetLoading))),
        )
        .add_systems(
            Update,
            tick_time_since_gaming.run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<TimeSinceGaming>();
    }
}
