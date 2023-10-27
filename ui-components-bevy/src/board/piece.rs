use bevy::{prelude::*, sprite::Anchor};

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(
                "[some piece]",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    font: asset_server.load("/Agbalumo-Regular.ttf"),
                },
            )],
            alignment: TextAlignment::Center,
            linebreak_behavior: bevy::text::BreakLineOn::AnyCharacter,
        },
        text_anchor: Anchor::Center,
        ..Default::default()
    });
}

pub fn update() {}
