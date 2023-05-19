use bevy_app::App;

pub trait IsSpritePipeline {
    fn prepare_render_app(render_app: &mut App);
    fn add_self_to(render_app: &mut App);
}

pub mod old {
    use bevy_app::App;
    use bevy_core_pipeline::core_2d::Transparent2d;
    use bevy_ecs::schedule::IntoSystemConfigs;
    use bevy_render::render_phase::AddRenderCommand;
    use bevy_render::render_resource::SpecializedRenderPipelines;
    use bevy_render::{ExtractSchedule, Render, RenderSet};

    use crate::render::old::{
        extract_sprite_events, extract_sprites, queue_sprites, DrawSprite, ExtractedSprites,
        ImageBindGroups, SpriteAssetEvents, SpriteMeta, SpritePipeline,
    };
    use crate::usability_trait::IsSpritePipeline;
    use crate::{queue_material2d_meshes, ColorMaterial, SpriteSystem};

    impl IsSpritePipeline for SpritePipeline {
        fn prepare_render_app(render_app: &mut App) {
            render_app
                .init_resource::<ImageBindGroups>()
                .init_resource::<SpecializedRenderPipelines<SpritePipeline>>()
                .init_resource::<SpriteMeta>()
                .init_resource::<ExtractedSprites>()
                .init_resource::<SpriteAssetEvents>()
                .add_render_command::<Transparent2d, DrawSprite>()
                .add_systems(
                    ExtractSchedule,
                    (
                        extract_sprites.in_set(SpriteSystem::ExtractSprites),
                        extract_sprite_events,
                    ),
                )
                .add_systems(
                    Render,
                    queue_sprites
                        .in_set(RenderSet::Queue)
                        .ambiguous_with(queue_material2d_meshes::<ColorMaterial>),
                );
        }

        fn add_self_to(render_app: &mut App) {
            render_app.init_resource::<SpritePipeline>();
        }
    }
}

pub mod new {
    use bevy_app::App;
    use bevy_core_pipeline::core_2d::Transparent2d;
    use bevy_ecs::schedule::IntoSystemConfigs;
    use bevy_render::render_phase::AddRenderCommand;
    use bevy_render::render_resource::SpecializedRenderPipelines;
    use bevy_render::{ExtractSchedule, Render, RenderSet};

    use crate::render::new::{
        extract_sprite_events, extract_sprites, queue_sprites, DrawSprite, ExtractedSprites,
        ImageBindGroups, SpriteAssetEvents, SpriteMeta, SpritePipelineNew,
    };
    use crate::usability_trait::IsSpritePipeline;
    use crate::{queue_material2d_meshes, ColorMaterial, SpriteSystem};

    impl IsSpritePipeline for SpritePipelineNew {
        fn prepare_render_app(render_app: &mut App) {
            render_app
                .init_resource::<ImageBindGroups>()
                .init_resource::<SpecializedRenderPipelines<SpritePipelineNew>>()
                .init_resource::<SpriteMeta>()
                .init_resource::<ExtractedSprites>()
                .init_resource::<SpriteAssetEvents>()
                .add_render_command::<Transparent2d, DrawSprite>()
                .add_systems(
                    ExtractSchedule,
                    (
                        extract_sprites.in_set(SpriteSystem::ExtractSprites),
                        extract_sprite_events,
                    ),
                )
                .add_systems(
                    Render,
                    queue_sprites
                        .in_set(RenderSet::Queue)
                        .ambiguous_with(queue_material2d_meshes::<ColorMaterial>),
                );
        }

        fn add_self_to(render_app: &mut App) {
            render_app.init_resource::<SpritePipelineNew>();
        }
    }
}
