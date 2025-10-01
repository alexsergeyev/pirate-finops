use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn setup_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.2).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "🏴‍☠️ Pirate FinOps Treasure Hunt",
                TextStyle {
                    font_size: 48.0,
                    color: Color::srgb(1.0, 0.8, 0.2),
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                "Hunt for wasted Azure resources!",
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            }));

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(MenuButton {
                    action: MenuAction::Play,
                });
        })
        .insert(GameUI);
}

pub fn menu_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match button.action {
                    MenuAction::Play => next_state.set(crate::GameState::Playing),
                    MenuAction::Quit => std::process::exit(0),
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<GameUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.2, 0.9).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Budget: $10000",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.2, 0.8, 0.2),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::right(Val::Px(20.0)),
                    ..default()
                }),
                BudgetText,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Sprint: 0",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::right(Val::Px(20.0)),
                    ..default()
                }),
                SprintText,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Waste: 0%",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.8, 0.8, 0.2),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::right(Val::Px(20.0)),
                    ..default()
                }),
                WasteText,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Savings: $0/mo",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.2, 0.8, 0.8),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::right(Val::Px(20.0)),
                    ..default()
                }),
                SavingsText,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Burn: $0/turn",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.8, 0.4, 0.2),
                        ..default()
                    },
                ),
                BurnRateText,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Combo: 1.0x",
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(1.0, 0.8, 0.2),
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::left(Val::Px(20.0)),
                    ..default()
                }),
                ComboText,
            ));
        })
        .insert(GameUI);

    // Current tile info panel - moved higher to avoid overlap
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(90.0),
                max_width: Val::Px(800.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(70.0),
                left: Val::Percent(5.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.2, 0.1, 0.95).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Current Location: Exploring...",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.9, 0.9, 0.2),
                        ..default()
                    },
                ),
                CurrentTileText,
            ));
        })
        .insert(GameUI);

    // Controls panel
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.2, 0.9).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Move: Arrow/WASD ($100 + auto-scan) | Fix: Space (may fail on critical resources)",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                    ..default()
                },
            ));
        })
        .insert(GameUI);
}

pub fn update_ui(
    game_data: Res<GameData>,
    tiles: Query<(&TileState, &ResourceType, &Tile)>,
    mut budget_query: Query<&mut Text, (With<BudgetText>, Without<SprintText>, Without<WasteText>, Without<SavingsText>, Without<BurnRateText>, Without<ComboText>, Without<CurrentTileText>)>,
    mut sprint_query: Query<&mut Text, (With<SprintText>, Without<BudgetText>, Without<WasteText>, Without<SavingsText>, Without<BurnRateText>, Without<ComboText>, Without<CurrentTileText>)>,
    mut waste_query: Query<&mut Text, (With<WasteText>, Without<BudgetText>, Without<SprintText>, Without<SavingsText>, Without<BurnRateText>, Without<ComboText>, Without<CurrentTileText>)>,
    mut savings_query: Query<&mut Text, (With<SavingsText>, Without<BudgetText>, Without<SprintText>, Without<WasteText>, Without<BurnRateText>, Without<ComboText>, Without<CurrentTileText>)>,
    mut burn_query: Query<&mut Text, (With<BurnRateText>, Without<BudgetText>, Without<SprintText>, Without<WasteText>, Without<SavingsText>, Without<ComboText>, Without<CurrentTileText>)>,
    mut combo_query: Query<&mut Text, (With<ComboText>, Without<BudgetText>, Without<SprintText>, Without<WasteText>, Without<SavingsText>, Without<BurnRateText>, Without<CurrentTileText>)>,
    mut current_tile_query: Query<&mut Text, (With<CurrentTileText>, Without<BudgetText>, Without<SprintText>, Without<WasteText>, Without<SavingsText>, Without<BurnRateText>, Without<ComboText>)>,
) {
    if let Ok(mut text) = budget_query.get_single_mut() {
        text.sections[0].value = format!("Budget: ${}", game_data.budget);
        text.sections[0].style.color = if game_data.budget < 2000 {
            Color::srgb(0.8, 0.2, 0.2)
        } else if game_data.budget < 5000 {
            Color::srgb(0.8, 0.8, 0.2)
        } else {
            Color::srgb(0.2, 0.8, 0.2)
        };
    }

    if let Ok(mut text) = sprint_query.get_single_mut() {
        text.sections[0].value = format!("Sprint: {}", game_data.sprint);
    }

    if let Ok(mut text) = waste_query.get_single_mut() {
        let waste_pct = game_data.waste_percentage();
        text.sections[0].value = format!("Waste: {:.1}%", waste_pct);
        text.sections[0].style.color = if waste_pct > 30.0 {
            Color::srgb(0.8, 0.2, 0.2)
        } else if waste_pct > 15.0 {
            Color::srgb(0.8, 0.8, 0.2)
        } else {
            Color::srgb(0.2, 0.8, 0.2)
        };
    }

    if let Ok(mut text) = savings_query.get_single_mut() {
        text.sections[0].value = format!("Savings: ${}/mo", game_data.monthly_savings);
    }

    if let Ok(mut text) = burn_query.get_single_mut() {
        let burn_rate = game_data.calculate_burn_rate();
        text.sections[0].value = format!("Burn: ${}/turn", burn_rate);
        text.sections[0].style.color = if burn_rate > 800 {
            Color::srgb(0.8, 0.2, 0.2)
        } else if burn_rate > 500 {
            Color::srgb(0.8, 0.4, 0.2)
        } else {
            Color::srgb(0.8, 0.6, 0.2)
        };
    }

    if let Ok(mut text) = combo_query.get_single_mut() {
        let combo_text = if game_data.combo_multiplier > 1.0 {
            format!("Combo: {:.1}x! +${}", game_data.combo_multiplier, game_data.adjacency_bonus)
        } else {
            format!("Combo: {:.1}x", game_data.combo_multiplier)
        };
        text.sections[0].value = combo_text;

        // Color based on combo level
        let combo_color = if game_data.combo_multiplier >= 1.5 {
            Color::srgb(1.0, 0.2, 0.2)  // Red for high combo
        } else if game_data.combo_multiplier > 1.0 {
            Color::srgb(1.0, 0.8, 0.2)  // Yellow for active combo
        } else {
            Color::srgb(0.7, 0.7, 0.7)  // Gray for no combo
        };
        text.sections[0].style.color = combo_color;
    }

    // Update current tile info
    if let Ok(mut text) = current_tile_query.get_single_mut() {
        let mut current_tile_info = "Current Location: Unknown".to_string();

        // Find the tile the player is on
        for (tile_state, resource_type, tile) in tiles.iter() {
            if tile.x == game_data.player_x && tile.y == game_data.player_y {
                if tile_state.revealed {
                    if matches!(resource_type, ResourceType::Empty) {
                        current_tile_info = "Current Location: Clear waters - nothing to fix".to_string();
                    } else {
                        let hint = resource_type.hint();
                        let (min_savings, max_savings) = resource_type.savings_range();
                        let fix_time = resource_type.fix_sprints();

                        if tile_state.fixed {
                            current_tile_info = format!("✓ Fixed: {} (Saving ${}-${}/mo)", hint, min_savings, max_savings);
                        } else if tile_state.fixing_sprints_left > 0 {
                            current_tile_info = format!("⚙ Fixing: {} ({} sprints left)", hint, tile_state.fixing_sprints_left);
                        } else {
                            current_tile_info = format!("⚠ {}: Fix {} sprint{} → Save ${}-${}/mo",
                                hint, fix_time, if fix_time > 1 { "s" } else { "" }, min_savings, max_savings);
                        }
                    }
                } else {
                    current_tile_info = "Current Location: Unexplored territory".to_string();
                }
                break;
            }
        }

        text.sections[0].value = current_tile_info;
    }
}

pub fn setup_game_over(mut commands: Commands, game_data: Res<GameData>) {
    let (title, color) = if game_data.game_won {
        ("🏴‍☠️ Arr! Ye Plundered the Treasure!", Color::srgb(0.8, 0.7, 0.2))
    } else {
        ("☠️ Walk the Plank! Yer Coffers Be Empty!", Color::srgb(0.8, 0.2, 0.2))
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.2, 0.95).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                title,
                TextStyle {
                    font_size: 48.0,
                    color,
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                format!("Final Budget: ${}", game_data.budget),
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }));

            parent.spawn(TextBundle::from_section(
                format!("Monthly Savings: ${}", game_data.monthly_savings),
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            }));

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Set Sail Again",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                })
                .insert(MenuButton {
                    action: MenuAction::Play,
                });
        })
        .insert(GameOverUI);
}

pub fn game_over_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match button.action {
                    MenuAction::Play => next_state.set(crate::GameState::Playing),
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}