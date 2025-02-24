import re


def obfuscate_strings(text):
    # 正则表达式匹配所有字符串
    def replace_match(match):
        if "Cannot" in match.group(0):
            return f"obfstr!({match.group(0)})"
        return f"obfstr!({match.group(0)}).to_string()"

    # 匹配双引号和单引号的字符串
    pattern = r'(["\'])(.*?)(\1)'
    return re.sub(pattern, replace_match, text)


# 测试文本
text = """

#[automatically_derived]
#[allow(unused_variables)]
impl AssetCollection for GameAssets {
    fn create(world: &mut ::bevy::ecs::world::World) -> Self {
        let from_world_fields = ();
        world.resource_scope(
            |world,
             asset_keys: ::bevy::prelude::Mut<
                ::bevy_asset_loader::dynamic_asset::DynamicAssets,
            >| {
                GameAssets {
                    ami_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("characters/ami.png")
                    },
                    ami_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(128u32, 128u32),
                            8u32,
                            3u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    ima_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("characters/ima.png")
                    },
                    ima_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(128u32, 128u32),
                            8u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    character_animations: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        <[_]>::into_vec(
                            // #[rustc_box]
                            std::boxed::Box::new([
                                asset_server.load("characters/character.trickfilm#idle"),
                                asset_server.load("characters/character.trickfilm#walk"),
                                asset_server.load("characters/character.trickfilm#run"),
                            ]),
                        )
                    },
                    number0: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_0.png")
                    },
                    number1: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_1.png")
                    },
                    number2: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_2.png")
                    },
                    number3: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_3.png")
                    },
                    number4: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_4.png")
                    },
                    number5: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_5.png")
                    },
                    number6: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_6.png")
                    },
                    number7: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_7.png")
                    },
                    number8: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_8.png")
                    },
                    number9: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("number/digit_9.png")
                    },
                    smoke_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("effects/smoke.png")
                    },
                    smoke_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(64u32, 64u32),
                            12u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    smoke_animations: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        <[_]>::into_vec(std::boxed::Box::new([
                            asset_server.load("effects/smoke.trickfilm#main")
                        ]))
                    },
                    level: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("map/level.ldtk")
                    },
                    tutorial_switch_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("map/tutorial_switch.png")
                    },
                    tutorial_switch_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(32u32, 32u32),
                            3u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    tutorial_wall: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("map/tutorial_wall.png")
                    },
                    bed_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("map/bed.png")
                    },
                    bed_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(64u32, 64u32),
                            2u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    aspect_socket_texture_left: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("map/aspect_socket_left.png")
                    },
                    aspect_socket_texture_right: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("map/aspect_socket_right.png")
                    },
                    aspect_socket_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(64u32, 64u32),
                            2u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    aspect_combiner_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("map/aspect_combiner.png")
                    },
                    aspect_combiner_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(64u32, 64u32),
                            2u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    ui_interact_key_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/keys/interact_key.png")
                    },
                    ui_interact_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(32u32, 32u32),
                            2u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    ui_down_key_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/keys/down_key.png")
                    },
                    ui_down_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(34u32, 34u32),
                            3u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    ui_up_key_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/keys/up_key.png")
                    },
                    ui_up_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(34u32, 34u32),
                            3u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    ui_left_key_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/keys/left_key.png")
                    },
                    ui_left_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(34u32, 34u32),
                            3u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    ui_right_key_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/keys/right_key.png")
                    },
                    ui_right_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(34u32, 34u32),
                            3u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    ui_shift_key_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/keys/shift_key.png")
                    },
                    ui_shift_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect("Cannot get Assets<TextureAtlasLayout>");
                        atlases.add(TextureAtlasLayout::from_grid(
                            ::bevy::math::UVec2::new(66u32, 34u32),
                            2u32,
                            1u32,
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                            Some(::bevy::math::UVec2::new(0u32, 0u32)),
                        ))
                    },
                    ui_keys_animations: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        <[_]>::into_vec(std::boxed::Box::new([
                            asset_server.load("ui/keys/keys.trickfilm#key"),
                            asset_server.load("ui/keys/keys.trickfilm#arrow"),
                            asset_server.load("ui/keys/keys.trickfilm#shift"),
                        ]))
                    },
                    white_pixel: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/white_pixel.png")
                    },
                    dialogue_edge: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/dialogue_edge.png")
                    },
                    dialogue_continue: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/dialogue_continue.png")
                    },
                    ami_character_icon: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/ami_character_icon.png")
                    },
                    ima_character_icon: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/ima_character_icon.png")
                    },
                    vignette: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("ui/vignette.png")
                    },
                    ami_blip_sound: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("audio/ami_blip.ogg")
                    },
                    ima_blip_sound: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("audio/ima_blip.ogg")
                    },
                    koto_hit_sound: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("audio/koto.ogg")
                    },
                    footstep: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("audio/footstep.ogg")
                    },
                    select_aspect: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("audio/select_aspect.ogg")
                    },
                    deselect_aspect: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("audio/deselect_aspect.ogg")
                    },
                    main_bgm: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("audio/main_bgm.ogg")
                    },
                    ending_bgm: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("audio/ending_bgm.ogg")
                    },
                    pixel_font: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("fonts/PressStart2P.ttf")
                    },
                    silver_font: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect("Cannot get AssetServer");
                        asset_server.load("fonts/Silver.ttf")
                    },
                }
            },
        )
    }
    fn load(world: &mut ::bevy::ecs::world::World) -> Vec<::bevy::prelude::UntypedHandle> {
        let mut handles = Vec::new();
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("characters/ami.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("characters/ima.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("characters/character.trickfilm#idle")
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load_untyped("characters/character.trickfilm#walk")
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load_untyped("characters/character.trickfilm#run")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_0.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_1.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_2.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_3.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_4.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_5.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_6.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_7.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_8.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("number/digit_9.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("effects/smoke.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("effects/smoke.trickfilm#main")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("map/level.ldtk").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("map/tutorial_switch.png")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("map/tutorial_wall.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("map/bed.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("map/aspect_socket_left.png")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("map/aspect_socket_right.png")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("map/aspect_combiner.png")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("ui/keys/interact_key.png")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("ui/keys/down_key.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("ui/keys/up_key.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("ui/keys/left_key.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("ui/keys/right_key.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("ui/keys/shift_key.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("ui/keys/keys.trickfilm#key")
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load_untyped("ui/keys/keys.trickfilm#arrow")
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load_untyped("ui/keys/keys.trickfilm#shift")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("ui/white_pixel.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("ui/dialogue_edge.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("ui/dialogue_continue.png")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("ui/ami_character_icon.png")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("ui/ima_character_icon.png")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("ui/vignette.png").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("audio/ami_blip.ogg").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("audio/ima_blip.ogg").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("audio/koto.ogg").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("audio/footstep.ogg").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("audio/select_aspect.ogg")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("audio/deselect_aspect.ogg")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("audio/main_bgm.ogg").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("audio/ending_bgm.ogg").untyped());
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(
                asset_server
                    .load_untyped("fonts/PressStart2P.ttf")
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect("Cannot get AssetServer");
            handles.push(asset_server.load_untyped("fonts/Silver.ttf").untyped());
        }
        handles
    }
}
"""

# 调用函数
result = obfuscate_strings(text)
print(result)
