mod render;

use bevy::{
    prelude::*,
    render::{
        sync_world::SyncToRenderWorld,
        view::{NoFrustumCulling, RenderVisibleEntities, VisibilityClass},
    },
};

pub struct InfiniteGridPlugin;

impl Plugin for InfiniteGridPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        render::render_app_builder(app);
    }
}

#[derive(Component, Default)]
pub struct InfiniteGrid;

#[derive(Component, Copy, Clone)]
pub struct InfiniteGridSettings {
    pub x_axis_color: Color,
    pub z_axis_color: Color,
    pub minor_line_color: Color,
    pub major_line_color: Color,
    pub fadeout_distance: f32,
    pub dot_fadeout_strength: f32,
    pub scale: f32,
}

impl Default for InfiniteGridSettings {
    fn default() -> Self {
        Self {
            x_axis_color: Color::srgb(1.0, 0.2, 0.2),
            z_axis_color: Color::srgb(0.2, 0.2, 1.0),
            minor_line_color: Color::srgb(0.1, 0.1, 0.1),
            major_line_color: Color::srgb(0.25, 0.25, 0.25),
            fadeout_distance: 100.,
            dot_fadeout_strength: 0.25,
            scale: 1.,
        }
    }
}

#[derive(Bundle)]
pub struct InfiniteGridBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub settings: InfiniteGridSettings,
    pub grid: InfiniteGrid,
    pub visibility: Visibility,
    pub view_visibility: ViewVisibility,
    pub visibility_class: VisibilityClass,
    pub inherited_visibility: InheritedVisibility,
    pub shadow_casters: RenderVisibleEntities,
    pub no_frustum_culling: NoFrustumCulling,
    pub sync_to_render_world: SyncToRenderWorld,
}

impl Default for InfiniteGridBundle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            global_transform: Default::default(),
            settings: Default::default(),
            grid: Default::default(),
            visibility: Default::default(),
            view_visibility: Default::default(),
            visibility_class: VisibilityClass([std::any::TypeId::of::<Mesh3d>()].into()),
            inherited_visibility: Default::default(),
            shadow_casters: Default::default(),
            no_frustum_culling: Default::default(),
            sync_to_render_world: Default::default(),
        }
    }
}
