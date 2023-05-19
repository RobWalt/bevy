#![allow(clippy::type_complexity)]

use crate::render::usability_trait::IsSpritePipeline;
mod bundle;
mod dynamic_texture_atlas_builder;
mod mesh2d;
mod render;
mod sprite;
mod texture_atlas;
mod texture_atlas_builder;

pub mod collide_aabb;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        bundle::{SpriteBundle, SpriteSheetBundle},
        sprite::Sprite,
        texture_atlas::{TextureAtlas, TextureAtlasSprite},
        ColorMaterial, ColorMesh2dBundle, TextureAtlasBuilder,
    };
}

pub use bundle::*;
pub use dynamic_texture_atlas_builder::*;
pub use mesh2d::*;
pub use render::*;
pub use sprite::*;
pub use texture_atlas::*;
pub use texture_atlas_builder::*;

use bevy_app::prelude::*;
use bevy_asset::{AddAsset, Assets, Handle, HandleUntyped};
use bevy_ecs::prelude::*;
use bevy_reflect::TypeUuid;
use bevy_render::{
    mesh::Mesh,
    primitives::Aabb,
    render_resource::Shader,
    texture::Image,
    view::{NoFrustumCulling, VisibilitySystems},
    RenderApp,
};

#[derive(Default)]
pub struct SpritePlugin;

pub const SPRITE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2763343953151597127);
pub const SPRITE_SHADER_HANDLE_NEW: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2763343953151597128);

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpriteSystem {
    ExtractSprites,
}

type Pipeline = SpritePipelineNew;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        let sprite_shader = Shader::from_wgsl(include_str!("render/sprite.wgsl"));
        shaders.set_untracked(SPRITE_SHADER_HANDLE, sprite_shader);
        let sprite_shader_new = Shader::from_wgsl(include_str!("render/sprite_new.wgsl"));
        shaders.set_untracked(SPRITE_SHADER_HANDLE_NEW, sprite_shader_new);
        app.add_asset::<TextureAtlas>()
            .register_asset_reflect::<TextureAtlas>()
            .register_type::<Sprite>()
            .register_type::<TextureAtlasSprite>()
            .register_type::<Anchor>()
            .register_type::<Mesh2dHandle>()
            .add_plugin(Mesh2dRenderPlugin)
            .add_plugin(ColorMaterialPlugin)
            .add_systems(
                PostUpdate,
                calculate_bounds_2d.in_set(VisibilitySystems::CalculateBounds),
            );

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            Pipeline::prepare_render_app(render_app);
        }
    }

    fn finish(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            Pipeline::add_self_to(render_app);
        }
    }
}

pub fn calculate_bounds_2d(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    atlases: Res<Assets<TextureAtlas>>,
    meshes_without_aabb: Query<(Entity, &Mesh2dHandle), (Without<Aabb>, Without<NoFrustumCulling>)>,
    sprites_without_aabb: Query<
        (Entity, &Sprite, &Handle<Image>),
        (Without<Aabb>, Without<NoFrustumCulling>),
    >,
    atlases_without_aabb: Query<
        (Entity, &TextureAtlasSprite, &Handle<TextureAtlas>),
        (Without<Aabb>, Without<NoFrustumCulling>),
    >,
) {
    for (entity, mesh_handle) in &meshes_without_aabb {
        if let Some(mesh) = meshes.get(&mesh_handle.0) {
            if let Some(aabb) = mesh.compute_aabb() {
                commands.entity(entity).insert(aabb);
            }
        }
    }
    for (entity, sprite, texture_handle) in &sprites_without_aabb {
        if let Some(size) = sprite
            .custom_size
            .or_else(|| images.get(texture_handle).map(|image| image.size()))
        {
            let aabb = Aabb {
                center: (-sprite.anchor.as_vec() * size).extend(0.0).into(),
                half_extents: (0.5 * size).extend(0.0).into(),
            };
            commands.entity(entity).insert(aabb);
        }
    }
    for (entity, atlas_sprite, atlas_handle) in &atlases_without_aabb {
        if let Some(size) = atlas_sprite.custom_size.or_else(|| {
            atlases
                .get(atlas_handle)
                .and_then(|atlas| atlas.textures.get(atlas_sprite.index))
                .map(|rect| (rect.min - rect.max).abs())
        }) {
            let aabb = Aabb {
                center: (-atlas_sprite.anchor.as_vec() * size).extend(0.0).into(),
                half_extents: (0.5 * size).extend(0.0).into(),
            };
            commands.entity(entity).insert(aabb);
        }
    }
}
