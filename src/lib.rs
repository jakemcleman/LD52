mod actions;
mod loading;
mod player;
mod mainmenu;
mod ui_events;
mod sprite_anim;

use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
use crate::mainmenu::MainMenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use sprite_anim::SpriteAnimationPlugin;
use ui_events::UiEventPlugin;
use bevy_pixel_camera::{
    PixelBorderPlugin, PixelCameraPlugin
};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .insert_resource(ClearColor(Color::BLACK))
            .add_plugin(LoadingPlugin)
            .add_plugin(UiEventPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SpriteAnimationPlugin)
            .add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::BLACK,
            });

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}