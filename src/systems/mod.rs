use bevy::{
    a11y::AccessibilityPlugin,
    animation::AnimationPlugin,
    app::{PluginGroup, PluginGroupBuilder},
    asset::AssetPlugin,
    audio::AudioPlugin,
    core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin},
    core_pipeline::CorePipelinePlugin,
    diagnostic::DiagnosticsPlugin,
    gilrs::GilrsPlugin,
    gizmos::GizmoPlugin,
    gltf::GltfPlugin,
    hierarchy::HierarchyPlugin,
    input::InputPlugin,
    log::LogPlugin,
    pbr::PbrPlugin,
    render::{pipelined_rendering::PipelinedRenderingPlugin, texture::ImagePlugin, RenderPlugin},
    scene::ScenePlugin,
    sprite::SpritePlugin,
    text::TextPlugin,
    time::TimePlugin,
    transform::TransformPlugin,
    ui::UiPlugin,
    window::WindowPlugin,
    winit::WinitPlugin,
};

pub mod camera;
pub mod player;

pub struct BasePlugins;

impl PluginGroup for BasePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LogPlugin::default())
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin)
            .add(FrameCountPlugin)
            .add(TimePlugin)
            .add(TransformPlugin)
            .add(HierarchyPlugin)
            .add(DiagnosticsPlugin)
            .add(InputPlugin)
            .add(WindowPlugin::default())
            .add(AccessibilityPlugin)
            .add(AssetPlugin::default())
            .add(ScenePlugin)
            .add(WinitPlugin::default())
            .add(RenderPlugin::default())
            .add(ImagePlugin::default())
            .add(PipelinedRenderingPlugin)
            .add(CorePipelinePlugin)
            .add(SpritePlugin)
            .add(TextPlugin)
            .add(UiPlugin)
            .add(PbrPlugin::default())
            .add(GltfPlugin::default())
            .add(AudioPlugin::default())
            .add(GilrsPlugin)
            .add(AnimationPlugin)
            .add(GizmoPlugin)
    }
}
