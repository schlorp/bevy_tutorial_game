use bevy::prelude::*;

use crate::ui::game_over_menu::components::*;

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOverMenu>>
){
    if let Ok(menu_entity) = menu_query.get_single(){
        commands.entity(menu_entity).despawn_recursive();
    }
}

pub fn build_game_over_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity{
    //=== game over menu ===
    let game_over_menu_entity = commands.spawn
    ((
        NodeBundle{
            style: Style{
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        GameOverMenu{}
    ))


        //=== game over menu container ===
        .with_children(|parent|{
            parent.spawn
            (
                NodeBundle{
                    style: Style{
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(35.0),
                        height: Val::Percent(80.0),
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                }
            )
                .with_children(|parent|{


                    //=== Game Over Title ===
                    parent.spawn
                    (
                        NodeBundle{
                            style: Style{
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                width: Val::Px(400.0),
                                height: Val::Px(80.0),
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        }
                    )
                        .with_children(|parent|{
                            //=== Game Over Text ===
                            parent.spawn
                            (
                                TextBundle{
                                    text: Text{
                                        sections: vec![
                                            TextSection::new(
                                                "Game Over", 
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
                                    ..default()
                                }
                            );
                        });


                    //=== score text ===
                    parent.spawn
                            (
                                TextBundle{
                                    text: Text{
                                        sections: vec![
                                            TextSection::new(
                                                "Your Score: ", 
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
                                    ..default()
                                }
                            );


                    //=== Restart Button ===
                    parent.spawn
                    ((
                        ButtonBundle{
                            style: Style{
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        },
                        RestartButton {},
                    ))
                        .with_children(|parent|{
                            parent.spawn
                            (
                                TextBundle{
                                    text: Text{
                                        sections: vec![
                                            TextSection::new(
                                                "Restart", 
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
                                    ..default()
                                }
                            );
                        });
                });


                    //=== main menu button
                    parent.spawn
                        ((
                            ButtonBundle{
                                style: Style{
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    ..default()
                                },
                                background_color: Color::BLACK.into(),
                                ..default()
                            },
                            MainMenuButton {}
                        ))
                            .with_children(|parent|{
                                parent.spawn
                                (
                                    TextBundle{
                                        text: Text{
                                            sections: vec![
                                                TextSection::new(
                                                    "Main Menu", 
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
                                        ..default()
                                    }
                                );
                            });

                    //=== quit button ===
                    parent.spawn
                        ((
                            ButtonBundle{
                                style: Style{
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    ..default()
                                },
                                background_color: Color::BLACK.into(),
                                ..default()
                            },
                            QuitButton {}
                        ))
                            .with_children(|parent|{
                                parent.spawn
                                (
                                    TextBundle{
                                        text: Text{
                                            sections: vec![
                                                TextSection::new(
                                                    "Quit", 
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
                                        ..default()
                                    },
                                );
                            });
        })
    .id();

    return game_over_menu_entity;
}