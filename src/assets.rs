use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_kira_audio::AudioSource;
use bevy_trickfilm::prelude::*;
use obfstr::obfstr;
// #[derive(AssetCollection, Resource)]
#[derive(Resource)]
pub struct GameAssets {
    // --- CHARACTERS ---
    // #[asset(path = "characters/ami.png")]
    pub ami_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 128, tile_size_y = 128, columns = 8, rows = 3))]
    pub ami_layout: Handle<TextureAtlasLayout>,

    // #[asset(path = "characters/ima.png")]
    pub ima_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 128, tile_size_y = 128, columns = 8, rows = 1))]
    pub ima_layout: Handle<TextureAtlasLayout>,

    // #[asset(
    //     paths(
    //         "characters/character.trickfilm#idle",
    //         "characters/character.trickfilm#walk",
    //         "characters/character.trickfilm#run",
    //     ),
    //     collection(typed)
    // )]
    pub character_animations: Vec<Handle<AnimationClip2D>>,

    // --- NUMBER ---
    // #[asset(path = "number/digit_0.png")]
    pub number0: Handle<Image>,
    // #[asset(path = "number/digit_1.png")]
    pub number1: Handle<Image>,
    // #[asset(path = "number/digit_2.png")]
    pub number2: Handle<Image>,
    // #[asset(path = "number/digit_3.png")]
    pub number3: Handle<Image>,
    // #[asset(path = "number/digit_4.png")]
    pub number4: Handle<Image>,
    // #[asset(path = "number/digit_5.png")]
    pub number5: Handle<Image>,
    // #[asset(path = "number/digit_6.png")]
    pub number6: Handle<Image>,
    // #[asset(path = "number/digit_7.png")]
    pub number7: Handle<Image>,
    // #[asset(path = "number/digit_8.png")]
    pub number8: Handle<Image>,
    // #[asset(path = "number/digit_9.png")]
    pub number9: Handle<Image>,

    pub combine: Handle<Image>,

    // --- EFFECTS ---
    // #[asset(path = "effects/smoke.png")]
    pub smoke_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 12, rows = 1))]
    pub smoke_layout: Handle<TextureAtlasLayout>,

    // #[asset(paths("effects/smoke.trickfilm#main",), collection(typed))]
    pub smoke_animations: Vec<Handle<AnimationClip2D>>,

    // --- MAP ---
    // #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,

    // #[asset(path = "map/tutorial_switch.png")]
    pub tutorial_switch_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 3, rows = 1))]
    pub tutorial_switch_layout: Handle<TextureAtlasLayout>,
    // #[asset(path = "map/tutorial_wall.png")]
    pub tutorial_wall: Handle<Image>,

    // #[asset(path = "map/bed.png")]
    pub bed_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 2, rows = 1))]
    pub bed_layout: Handle<TextureAtlasLayout>,

    // #[asset(path = "map/aspect_socket_left.png")]
    pub aspect_socket_texture_left: Handle<Image>,
    // #[asset(path = "map/aspect_socket_right.png")]
    pub aspect_socket_texture_right: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 2, rows = 1))]
    pub aspect_socket_layout: Handle<TextureAtlasLayout>,
    // #[asset(path = "map/aspect_combiner.png")]
    pub aspect_combiner_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 2, rows = 1))]
    pub aspect_combiner_layout: Handle<TextureAtlasLayout>,

    // #[asset(path = "ui/keys/interact_key.png")]
    pub ui_interact_key_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 2, rows = 1))]
    pub ui_interact_key_layout: Handle<TextureAtlasLayout>,

    // #[asset(path = "ui/keys/down_key.png")]
    pub ui_down_key_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_down_key_layout: Handle<TextureAtlasLayout>,
    // #[asset(path = "ui/keys/up_key.png")]
    pub ui_up_key_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_up_key_layout: Handle<TextureAtlasLayout>,
    // #[asset(path = "ui/keys/left_key.png")]
    pub ui_left_key_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_left_key_layout: Handle<TextureAtlasLayout>,
    // #[asset(path = "ui/keys/right_key.png")]
    pub ui_right_key_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 1))]
    pub ui_right_key_layout: Handle<TextureAtlasLayout>,
    // #[asset(path = "ui/keys/shift_key.png")]
    pub ui_shift_key_texture: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 66, tile_size_y = 34, columns = 2, rows = 1))]
    pub ui_shift_key_layout: Handle<TextureAtlasLayout>,
    // #[asset(
    //     paths(
    //         "ui/keys/keys.trickfilm#key",
    //         "ui/keys/keys.trickfilm#arrow",
    //         "ui/keys/keys.trickfilm#shift"
    //     ),
    //     collection(typed)
    // )]
    pub ui_keys_animations: Vec<Handle<AnimationClip2D>>,

    // --- UI ---
    // #[asset(path = "ui/white_pixel.png")]
    pub white_pixel: Handle<Image>,

    // #[asset(path = "ui/dialogue_edge.png")]
    pub dialogue_edge: Handle<Image>,
    // #[asset(path = "ui/dialogue_continue.png")]
    pub dialogue_continue: Handle<Image>,

    // #[asset(path = "ui/ami_character_icon.png")]
    pub ami_character_icon: Handle<Image>,
    // #[asset(path = "ui/ima_character_icon.png")]
    pub ima_character_icon: Handle<Image>,

    // #[asset(path = "ui/vignette.png")]
    pub vignette: Handle<Image>,

    // --- AUDIO ---
    // #[asset(path = "audio/ami_blip.ogg")]
    pub ami_blip_sound: Handle<AudioSource>,
    // #[asset(path = "audio/ima_blip.ogg")]
    pub ima_blip_sound: Handle<AudioSource>,

    // #[asset(path = "audio/koto.ogg")]
    pub koto_hit_sound: Handle<AudioSource>,
    // #[asset(path = "audio/footstep.ogg")]
    pub footstep: Handle<AudioSource>,
    // #[asset(path = "audio/select_aspect.ogg")]
    pub select_aspect: Handle<AudioSource>,
    // #[asset(path = "audio/deselect_aspect.ogg")]
    pub deselect_aspect: Handle<AudioSource>,

    // #[asset(path = "audio/main_bgm.ogg")]
    pub main_bgm: Handle<AudioSource>,
    // #[asset(path = "audio/ending_bgm.ogg")]
    pub ending_bgm: Handle<AudioSource>,

    // --- FONT ---
    // #[asset(path = "fonts/PressStart2P.ttf")]
    pub pixel_font: Handle<Font>,
    // #[asset(path = "fonts/Silver.ttf")]
    pub silver_font: Handle<Font>,
}

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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("characters/ami.png").to_string())
                    },
                    ami_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("characters/ima.png").to_string())
                    },
                    ima_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        <[_]>::into_vec(
                            // #[rustc_box]
                            std::boxed::Box::new([
                                asset_server.load(
                                    obfstr!("characters/character.trickfilm#idle").to_string(),
                                ),
                                asset_server.load(
                                    obfstr!("characters/character.trickfilm#walk").to_string(),
                                ),
                                asset_server.load(
                                    obfstr!("characters/character.trickfilm#run").to_string(),
                                ),
                            ]),
                        )
                    },
                    number0: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_0.png").to_string())
                    },
                    number1: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_1.png").to_string())
                    },
                    number2: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_2.png").to_string())
                    },
                    number3: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_3.png").to_string())
                    },
                    number4: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_4.png").to_string())
                    },
                    number5: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_5.png").to_string())
                    },
                    number6: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_6.png").to_string())
                    },
                    number7: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_7.png").to_string())
                    },
                    number8: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_8.png").to_string())
                    },
                    number9: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_9.png").to_string())
                    },
                    combine: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("number/digit_x.png").to_string())
                    },
                    smoke_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("effects/smoke.png").to_string())
                    },
                    smoke_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        <[_]>::into_vec(std::boxed::Box::new([
                            asset_server.load(obfstr!("effects/smoke.trickfilm#main").to_string())
                        ]))
                    },
                    level: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("fake://asd2.ldtk").to_string())
                    },
                    tutorial_switch_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("map/tutorial_switch.png").to_string())
                    },
                    tutorial_switch_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("map/tutorial_wall.png").to_string())
                    },
                    bed_texture: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("map/bed.png").to_string())
                    },
                    bed_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("map/aspect_socket_left.png").to_string())
                    },
                    aspect_socket_texture_right: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("map/aspect_socket_right.png").to_string())
                    },
                    aspect_socket_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("map/aspect_combiner.png").to_string())
                    },
                    aspect_combiner_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/keys/interact_key.png").to_string())
                    },
                    ui_interact_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/keys/down_key.png").to_string())
                    },
                    ui_down_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/keys/up_key.png").to_string())
                    },
                    ui_up_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/keys/left_key.png").to_string())
                    },
                    ui_left_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/keys/right_key.png").to_string())
                    },
                    ui_right_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/keys/shift_key.png").to_string())
                    },
                    ui_shift_key_layout: {
                        let mut atlases = world
                                .get_resource_mut::<
                                    ::bevy::asset::Assets<::bevy::sprite::TextureAtlasLayout>,
                                >()
                                .expect(obfstr!("Cannot get Assets<TextureAtlasLayout>"));
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
                            .expect(obfstr!("Cannot get AssetServer"));
                        <[_]>::into_vec(std::boxed::Box::new([
                            asset_server.load(obfstr!("ui/keys/keys.trickfilm#key").to_string()),
                            asset_server.load(obfstr!("ui/keys/keys.trickfilm#arrow").to_string()),
                            asset_server.load(obfstr!("ui/keys/keys.trickfilm#shift").to_string()),
                        ]))
                    },
                    white_pixel: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/white_pixel.png").to_string())
                    },
                    dialogue_edge: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/dialogue_edge.png").to_string())
                    },
                    dialogue_continue: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/dialogue_continue.png").to_string())
                    },
                    ami_character_icon: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/ami_character_icon.png").to_string())
                    },
                    ima_character_icon: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/ima_character_icon.png").to_string())
                    },
                    vignette: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("ui/vignette.png").to_string())
                    },
                    ami_blip_sound: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("audio/ami_blip.ogg").to_string())
                    },
                    ima_blip_sound: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("audio/ima_blip.ogg").to_string())
                    },
                    koto_hit_sound: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("audio/koto.ogg").to_string())
                    },
                    footstep: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("audio/footstep.ogg").to_string())
                    },
                    select_aspect: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("audio/select_aspect.ogg").to_string())
                    },
                    deselect_aspect: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("audio/deselect_aspect.ogg").to_string())
                    },
                    main_bgm: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("audio/main_bgm.ogg").to_string())
                    },
                    ending_bgm: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("audio/ending_bgm.ogg").to_string())
                    },
                    pixel_font: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("fonts/PressStart2P.ttf").to_string())
                    },
                    silver_font: {
                        let asset_server = world
                            .get_resource::<::bevy::asset::AssetServer>()
                            .expect(obfstr!("Cannot get AssetServer"));
                        asset_server.load(obfstr!("fonts/Silver.ttf").to_string())
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
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("characters/ami.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("characters/ima.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("characters/character.trickfilm#idle").to_string())
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load_untyped(obfstr!("characters/character.trickfilm#walk").to_string())
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load_untyped(obfstr!("characters/character.trickfilm#run").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_0.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_1.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_2.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_3.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_4.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_5.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_6.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_7.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_8.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("number/digit_9.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("effects/smoke.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("effects/smoke.trickfilm#main").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("map/level.ldtk").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("map/tutorial_switch.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("map/tutorial_wall.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("map/bed.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("map/aspect_socket_left.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("map/aspect_socket_right.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("map/aspect_combiner.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/interact_key.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/down_key.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/up_key.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/left_key.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/right_key.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/shift_key.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/keys.trickfilm#key").to_string())
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/keys.trickfilm#arrow").to_string())
                    .untyped(),
            );
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/keys/keys.trickfilm#shift").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/white_pixel.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/dialogue_edge.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/dialogue_continue.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/ami_character_icon.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/ima_character_icon.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("ui/vignette.png").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("audio/ami_blip.ogg").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("audio/ima_blip.ogg").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("audio/koto.ogg").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("audio/footstep.ogg").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("audio/select_aspect.ogg").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("audio/deselect_aspect.ogg").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("audio/main_bgm.ogg").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("audio/ending_bgm.ogg").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("fonts/PressStart2P.ttf").to_string())
                    .untyped(),
            );
        }
        {
            let asset_server = world
                .get_resource::<::bevy::prelude::AssetServer>()
                .expect(obfstr!("Cannot get AssetServer"));
            handles.push(
                asset_server
                    .load_untyped(obfstr!("fonts/Silver.ttf").to_string())
                    .untyped(),
            );
        }
        handles
    }
}
