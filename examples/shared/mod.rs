use bevy::prelude::*;
use bevy_scroller::ScrollerPlugin;
pub fn get_app(title: String)->App{
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
          primary_window: Some(Window {
            // present_mode: bevy::window::PresentMode::AutoNoVsync,
            title,
            ..default()
          }),
          ..default()
        }),
        ScrollerPlugin,
      ));
  #[cfg(feature = "dev")]
  {
    use bevy_editor_pls::EditorPlugin;
    use iyes_perf_ui::*;
    app
      .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
      .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
      .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
      .add_plugins(PerfUiPlugin);

    app.add_plugins(EditorPlugin::default());
  }
  app
}
