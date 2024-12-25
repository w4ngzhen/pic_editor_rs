mod cli;
mod workspace;
mod utils;

use crate::cli::AppArgs;
use crate::workspace::Workspace;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use clap::Parser;

fn main() {
    let args = AppArgs::parse();
    let workspace = Workspace::from_args(&args);

    App::new()
        .insert_resource(workspace)
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginPass` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn setup(mut commands: Commands, workspace: Res<Workspace>, asset_server: Res<AssetServer>) {
    let default_doc = workspace.get_default_doc();
    let doc_img: Handle<Image> = asset_server.load(default_doc.src_path.clone());
    commands.spawn(Camera2d::default());
    commands
        .spawn((
            default_doc.id.clone(),
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((ImageNode {
                image: doc_img,
                ..default()
            },));
        });
}
