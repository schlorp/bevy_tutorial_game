use bevy::prelude::*;

use crate::ui::hud::components::*;

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>){
    let _hud_entity = build_hud(&mut commands, &asset_server);
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<Hud>>){
    if let Ok(hud_entity) = hud_query.get_single(){
        commands.entity(hud_entity).despawn_recursive();
    }
}

pub fn build_hud(
    commands: &mut Commands,
    asset_server: &Res<AssetServer> 
) -> Entity {
    let hud_entity = commands.spawn
    (
        (
            NodeBundle{
                style: Style{
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 255.0, 0.15).into(),
                ..default()
            },
            Hud {},
        )
    )
    .with_children(|parent|{
        
    }).id();

    return hud_entity;
}