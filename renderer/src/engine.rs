mod message_handler;
mod scene;

pub fn run_bevy_engine(canvas_id: &str, receiver: crate::message::Receiver) {
    let selector = format!("#{}", canvas_id);

    bevy::prelude::App::new()
        .add_plugins(plugins(canvas_id))
        .insert_non_send_resource(message_handler::MessageHandler::new(receiver))
        .add_systems(bevy::app::Startup, scene::setup)
        .add_systems(bevy::app::Update, message_handler::handle_messages)
        .run();
}

fn plugins(canvas_id: &str) -> bevy::app::PluginGroupBuilder {
    use bevy::prelude::PluginGroup;

    /* Window, rendered to the canvas */
    let selector = format!("#{}", canvas_id);
    let window_plugin = bevy::prelude::WindowPlugin {
        primary_window: Some(bevy::window::Window {
            canvas: Some(selector),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..Default::default()
        }),
        ..Default::default()
    };

    /* Asset plugin loads assets from the web */
    let asset_plugin = bevy::prelude::AssetPlugin {
        file_path: "assets".into(),
        ..Default::default()
    };

    bevy::DefaultPlugins.set(window_plugin).set(asset_plugin)
}
