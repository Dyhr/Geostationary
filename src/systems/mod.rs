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
    window::{Window, WindowPlugin},
    winit::{WakeUp, WinitPlugin},
};

pub mod camera;
pub mod menu;
pub mod player;

pub struct BasePlugins {
    pub window_title: String,
}

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
            .add(WindowPlugin {
                primary_window: Some(Window {
                    title: self.window_title,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .add(AccessibilityPlugin)
            .add(AssetPlugin::default())
            .add(ScenePlugin)
            .add(WinitPlugin::<WakeUp>::default())
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

        // TODO this will be needed for headless builds
        // .add(ScheduleRunnerPlugin::default())
    }
}
