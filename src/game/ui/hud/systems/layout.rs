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
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    ..default()
                },
                ..default()
            },
            Hud {},
        )
    )
    .with_children(|parent|{
        //=== star counter ===
        parent.spawn
        (
            NodeBundle{
                style: Style{
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    width: Val::Percent(30.0),
                    height: Val::Percent(90.0),
                    ..default()
                },
                background_color: Color::rgba(157.0, 157.0, 157.0, 0.5).into(),
                ..default()
            }
        )
        .with_children(|parent|{
            //=== image ===
            parent.spawn
            (
                ImageBundle{
                    style: Style{
                        width: Val::Auto,
                        height: Val::Auto,
                        margin: UiRect::new(
                            Val::Px(8.0), 
                            Val::Px(8.0), 
                            Val::Px(8.0), 
                            Val::Px(8.0)
                        ),
                        ..default()
                    },
                    image: asset_server.load("sprites/Star.png").into(),
                    ..default()
                }
            );


            //=== score text
            parent.spawn
            (
                TextBundle{
                    text: Text{
                        sections: vec![
                            TextSection::new(
                                "Score", 
                                TextStyle{
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    color: Color::WHITE,
                                }
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    style: Style{
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
    }).id();

    return hud_entity;
}