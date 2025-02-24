#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod aspect;
mod assets;
mod audio;
mod fake_res;
mod get_level;
mod npc;
mod player;
mod ui;
mod utils;
mod world;

pub use assets::GameAssets;
pub type GameRng = rand_xoshiro::Xoshiro256PlusPlus;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode, WindowResolution};
use bevy_yarnspinner::prelude::*;

use bevy_asset_loader::prelude::*;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::Animation2DPlugin;
use bevy_tweening::*;
use obfstr::obfstr;

const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Intro,
    Gaming,
    Ending,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins((
            crate::fake_res::WebAssetPlugin::default(),
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: false,
                        canvas: Some("#game-canvas".to_string()),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
            ParticleSystemPlugin,
            Animation2DPlugin,
            TweeningPlugin,
            YarnSpinnerPlugin::with_yarn_source(YarnFile::new(obfstr!("test1.yarn"), yarn_other()))
                .add_yarn_source(YarnFile::new(obfstr!("test2.yarn"), yarn_aspects()))
                .with_development_file_generation(DevelopmentFileGeneration::None),
        ))
        .insert_resource(Msaa::Off)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Intro)
                .load_collection::<GameAssets>(),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins((
            world::WorldPlugin,
            audio::GameAudioPlugin,
            player::PlayerPlugin,
            utils::UtilsPlugin,
            aspect::AspectPlugin,
            ui::UiPlugin,
            npc::NpcPlugin,
        ))
        .run();
}

fn yarn_other() -> String {
    obfstr!(
        r##"title: Intro
---

你说的对，但是《这道逆向》是由出题人自主研发的一款全新开放世界冒险游戏。
游戏发生在一个被称作「rust」的幻想世界，在这里，被神选中的人将被授予「盗版ida 9.0」，导引F5之力。
你将扮演一位名为「选手」的神秘角色，在自由的旅行中邂逅不到性格各异、能力独特的同伴们。
后面忘了，总之靠你自己击败强敌，找回失散的亲人——同时，逐步发掘「flag」的真相。

===
title: ImaFirstEncounter
---

...
奇怪的人: 欢迎
你自己: 你tm是谁，这是哪？
奇怪的人: 这当然是一道逆向题。
奇怪的人: 后面你即将验证flag的地方。
你自己: 别jb废话了好吗，放我过去。
奇怪的人: 别急，出题人让我在这用对话拖你时间，哈哈哈哈。
奇怪的人: 你每次进来都会和我进行不能跳过的对话。
奇怪的人: 顺便，这题居然有符号，太凉心了把，毕竟没有符号出题人自己也做不来。

===
title: ImaFinalDialogue
---

...
奇怪的人: 看来你已经完成你的输入了，让我们看看结果如何。
...
...
<<trigger_ending>>

===
title: GoodEnding
---

你觉得对吗，我觉得对了。提交结果aliyunctf 花括号 md5(你的输入) 反花括号
<<game_over>>

===
title: BadEnding
---

你觉得对吗，我觉得不太对。
<<game_over>>

===
title: CheatEnding
---

你觉得对吗，我觉得你开了。
<<game_over>>

===
"##
    )
    .to_string()
}

fn yarn_aspects() -> String {
    obfstr!(
        r##"title: Combine
---
<<declare $haha = "0">>
奇怪的人: 你输入了{$haha}。
{pow()}
===
"##
    )
    .to_string()
}
