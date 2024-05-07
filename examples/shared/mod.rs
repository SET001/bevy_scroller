use bevy::prelude::*;
use bevy_scroller::ScrollerPlugin;
use iyes_perf_ui::*;

pub fn get_app(title: String) -> App {
  let mut app = App::new();
  app
    .add_plugins((
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          // present_mode: bevy::window::PresentMode::AutoNoVsync,
          title: format!("BEVY_SCROLLER example: {}", title),
          ..default()
        }),
        ..default()
      }),
      ScrollerPlugin,
    ))
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
    .add_plugins(PerfUiPlugin)
    .add_systems(Startup, default_start);
  #[cfg(feature = "dev")]
  {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugins(EditorPlugin::default())
  };
  app
}

fn default_start(mut commands: Commands) {
  commands.spawn(PerfUiCompleteBundle::default());
}
