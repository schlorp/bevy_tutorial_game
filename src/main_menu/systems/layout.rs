use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;



pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>){
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>){
    if let Ok(main_menu_entity) = main_menu_query.get_single(){
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
        NodeBundle {
            style: Style{
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(10.0),
                column_gap: Val::Px(10.0),
                ..default()
            },
            background_color: Color::MIDNIGHT_BLUE.into(),
            ..default()
        },
        MainMenu {},
    )
    )
    
    .with_children(|parent|{
        // === Title ===
        parent.spawn
        (
            NodeBundle{
                style: Style{
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Auto,
                    height: Val::Px(120.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            }
        )
            .with_children(|parent|{
                //===left image===
                parent.spawn
                (
                    ImageBundle{
                        style: Style{
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::new(
                                Val::Px(8.0), 
                                Val::Px(8.0), 
                                Val::Px(8.0), 
                                Val::Px(8.0)
                            ),
                            ..default()
                        },
                        image: asset_server.load("sprites/SpaRood.png").into(),
                        ..default()
                    }
                );


                //===text===
                parent.spawn
                (
                    TextBundle{
                        text: Text{
                            sections: vec![
                                TextSection::new(
                                    "Balls Game", 
                                    get_title_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );


                //===right image===
                parent.spawn
                (
                    ImageBundle{
                        style: Style{
                            width: Val::Px(64.0),
                            height: Val::Px(64.0),
                            margin: UiRect::new(
                                Val::Px(8.0), 
                                Val::Px(8.0), 
                                Val::Px(8.0), 
                                Val::Px(8.0)
                            ),
                            ..default()
                        },
                        image: asset_server.load("sprites/SpaRood.png").into(),
                        ..default()
                    }
                );
            });


        // === Play ===
        parent.spawn(
            (
                ButtonBundle{
                    style: Style{
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton {}
            )
        )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle{
                        text: Text{
                            sections: vec![
                                TextSection::new(
                                    "Play", 
                                    get_button_text_style(&asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });


        // === Quit ===
        parent.spawn(
            (
                ButtonBundle{
                    style: Style{
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton {}
            )
        )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle{
                        text: Text{
                            sections: vec![
                                TextSection::new(
                                    "Quit", 
                                    get_button_text_style(&asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });
    })
    .id();

    return main_menu_entity;
}