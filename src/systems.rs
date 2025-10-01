use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

// Helper function to check if resources are related (for combo system)
fn are_resources_related(a: &ResourceType, b: &ResourceType) -> bool {
    use ResourceType::*;
    match (a, b) {
        // Compute resources
        (IdleVM, IdleGPUCluster) | (IdleGPUCluster, IdleVM) => true,
        (IdleVM, MissedReservation) | (MissedReservation, IdleVM) => true,

        // Storage resources
        (OrphanedDisk, StaleSnapshot) | (StaleSnapshot, OrphanedDisk) => true,
        (OrphanedDisk, ExpiredBackups) | (ExpiredBackups, OrphanedDisk) => true,
        (EmptyStorageAccount, OrphanedDisk) | (OrphanedDisk, EmptyStorageAccount) => true,

        // Database resources
        (UnusedSQLDatabase, OverprovisionedCosmosDB) | (OverprovisionedCosmosDB, UnusedSQLDatabase) => true,

        // Container/OpenShift resources
        (OverprovisionedOpenShift, OversizedAppService) | (OversizedAppService, OverprovisionedOpenShift) => true,
        (OverprovisionedOpenShift, RedundantLoadBalancer) | (RedundantLoadBalancer, OverprovisionedOpenShift) => true,

        // Networking resources
        (UnusedPublicIP, RedundantLoadBalancer) | (RedundantLoadBalancer, UnusedPublicIP) => true,
        (UnusedPublicIP, IdleCDNEndpoint) | (IdleCDNEndpoint, UnusedPublicIP) => true,

        // Logging
        (LogIngestionBloat, ExpiredBackups) | (ExpiredBackups, LogIngestionBloat) => true,

        _ => false,
    }
}

// Count adjacent fixed tiles
fn count_adjacent_fixed_from_positions(
    fixed_positions: &[(u8, u8)],
    x: u8,
    y: u8,
    grid_size: u8,
) -> usize {
    let mut count = 0;
    let adjacent = [
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ];

    for &(px, py) in &adjacent {
        if px < grid_size && py < grid_size {
            if fixed_positions.contains(&(px, py)) {
                count += 1;
            }
        }
    }
    count
}

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_data: ResMut<GameData>,
) {
    // Reset game data for a fresh start - always use Normal difficulty
    *game_data = GameData::new(Difficulty::Normal);

    // Create ocean background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.05, 0.15, 0.25), // Deep ocean blue
            custom_size: Some(Vec2::new(1200.0, 900.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        ..default()
    });

    // Add grid overlay for better visibility
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.2, 0.3, 0.4, 0.3), // Semi-transparent grid background
            custom_size: Some(Vec2::new(
                game_data.grid_size as f32 * 64.0 + 20.0,
                game_data.grid_size as f32 * 64.0 + 20.0
            )),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    });

    let grid = generate_random_grid(game_data.grid_size);
    let tile_size = 64.0;
    let grid_offset_x = -(game_data.grid_size as f32 * tile_size) / 2.0 + tile_size / 2.0;
    let grid_offset_y = -(game_data.grid_size as f32 * tile_size) / 2.0 + tile_size / 2.0;

    // Create paths between tiles
    for x in 0..game_data.grid_size {
        for y in 0..game_data.grid_size {
            let world_x = grid_offset_x + (x as f32 * tile_size);
            let world_y = grid_offset_y + (y as f32 * tile_size);

            // Grid lines for better visibility
            if x < game_data.grid_size - 1 {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.4, 0.5, 0.6, 0.3), // Semi-transparent grid line
                        custom_size: Some(Vec2::new(tile_size, 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x + tile_size/2.0, world_y, -1.0),
                    ..default()
                });
            }

            if y < game_data.grid_size - 1 {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.4, 0.5, 0.6, 0.3), // Semi-transparent grid line
                        custom_size: Some(Vec2::new(2.0, tile_size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x, world_y + tile_size/2.0, -1.0),
                    ..default()
                });
            }
        }
    }

    // Spawn player at center
    let player_world_x = grid_offset_x + (game_data.player_x as f32 * tile_size);
    let player_world_y = grid_offset_y + (game_data.player_y as f32 * tile_size);

    // Load player ship sprite
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/player.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(tile_size * 0.8, tile_size * 0.8)),
                ..default()
            },
            transform: Transform::from_xyz(player_world_x, player_world_y, 10.0),
            ..default()
        },
        Player {
            x: game_data.player_x,
            y: game_data.player_y,
        },
        PlayerMarker,
    ));

    for x in 0..game_data.grid_size {
        for y in 0..game_data.grid_size {
            let world_x = grid_offset_x + (x as f32 * tile_size);
            let world_y = grid_offset_y + (y as f32 * tile_size);

            let resource_type = grid[x as usize][y as usize];

            // Base tile - foggy/unknown area
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.15, 0.15, 0.2, 0.95), // Dark fog of war
                        custom_size: Some(Vec2::new(tile_size - 2.0, tile_size - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_x, world_y, 0.0),
                    ..default()
                },
                Tile { x, y },
                TileState::default(),
                resource_type,
            ));

            // Calculate initial total waste from ALL resources on the map
            if !matches!(resource_type, ResourceType::Empty) {
                game_data.total_waste += resource_type.waste_cost();
            }
        }
    }

    // Total waste now includes ALL waste on map, not just revealed
}

pub fn handle_player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<PlayerMarker>>,
    mut game_data: ResMut<GameData>,
) {
    let Ok((mut transform, mut player)) = player_query.get_single_mut() else {
        return;
    };

    let mut moved = false;
    let mut new_x = player.x;
    let mut new_y = player.y;
    let mut target_rotation = transform.rotation; // Keep current rotation by default

    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        if player.y < game_data.grid_size - 1 {
            new_y += 1;
            moved = true;
            target_rotation = Quat::from_rotation_z(std::f32::consts::PI); // Face down visually when going up
        }
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        if player.y > 0 {
            new_y -= 1;
            moved = true;
            target_rotation = Quat::from_rotation_z(0.0); // Face up visually when going down
        }
    }
    if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        if player.x > 0 {
            new_x -= 1;
            moved = true;
            target_rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.0); // Face right visually when going left
        }
    }
    if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
        if player.x < game_data.grid_size - 1 {
            new_x += 1;
            moved = true;
            target_rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.0); // Face left visually when going right
        }
    }

    if moved {
        // Update player position
        player.x = new_x;
        player.y = new_y;
        game_data.player_x = new_x;
        game_data.player_y = new_y;

        // Update visual position
        let tile_size = 64.0;
        let grid_offset_x = -(game_data.grid_size as f32 * tile_size) / 2.0 + tile_size / 2.0;
        let grid_offset_y = -(game_data.grid_size as f32 * tile_size) / 2.0 + tile_size / 2.0;

        transform.translation.x = grid_offset_x + (new_x as f32 * tile_size);
        transform.translation.y = grid_offset_y + (new_y as f32 * tile_size);

        // Apply rotation directly
        transform.rotation = target_rotation;

        // Fixed movement cost for Normal difficulty
        game_data.budget -= 50;
        if game_data.budget <= 0 {
            game_data.game_lost = true;
        }
    }
}

// Auto-reveal system that scans ahead automatically
pub fn auto_reveal_current_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tiles: Query<(Entity, &mut Sprite, &mut TileState, &ResourceType, &Tile)>,
    mut game_data: ResMut<GameData>,
    player_query: Query<&Transform, With<PlayerMarker>>,
) {
    // Get player's movement direction based on ship rotation
    let direction = if let Ok(transform) = player_query.get_single() {
        let angle = transform.rotation.to_euler(bevy::math::EulerRot::ZYX).0;

        if (angle - std::f32::consts::PI).abs() < 0.1 || (angle + std::f32::consts::PI).abs() < 0.1 {
            // Rotation PI or -PI = ship moved UP (north) in grid
            (0, 1)  // Scan north (ahead in movement direction)
        } else if angle.abs() < 0.1 {
            // Rotation 0 = ship moved DOWN (south) in grid
            (0, -1)  // Scan south (ahead in movement direction)
        } else if (angle - std::f32::consts::PI / 2.0).abs() < 0.1 {
            // Rotation PI/2 = ship moved RIGHT (east) in grid
            (1, 0)  // Scan east (ahead in movement direction)
        } else if (angle + std::f32::consts::PI / 2.0).abs() < 0.1 {
            // Rotation -PI/2 = ship moved LEFT (west) in grid
            (-1, 0)  // Scan west (ahead in movement direction)
        } else {
            (0, 0)  // Default
        }
    } else {
        return;
    };

    // Calculate scan positions ONCE before the loop
    let player_x = game_data.player_x as i32;
    let player_y = game_data.player_y as i32;

    // Center tile in front
    let distance = 1;
    let center_x = player_x + direction.0 * distance;
    let center_y = player_y + direction.1 * distance;


    // Calculate left and right tiles in the scan arc
    let (left_x, left_y, right_x, right_y) = match direction {
        (0, 1) => {
            // Moving UP - scan up-left, up-center, up-right
            (center_x - 1, center_y, center_x + 1, center_y)
        }
        (0, -1) => {
            // Moving DOWN - scan down-left, down-center, down-right
            (center_x - 1, center_y, center_x + 1, center_y)
        }
        (1, 0) => {
            // Moving RIGHT - scan right-up, right-center, right-down
            (center_x, center_y + 1, center_x, center_y - 1)
        }
        (-1, 0) => {
            // Moving LEFT - scan left-up, left-center, left-down
            (center_x, center_y + 1, center_x, center_y - 1)
        }
        _ => (center_x, center_y, center_x, center_y), // Shouldn't happen
    };

    // Auto-reveal current tile AND scan 3 tiles ahead in facing direction
    for (entity, mut sprite, mut tile_state, resource_type, tile) in tiles.iter_mut() {
        let tile_x = tile.x as i32;
        let tile_y = tile.y as i32;

        let mut should_reveal = false;

        // Always reveal tile player is on
        if tile_x == player_x && tile_y == player_y {
            should_reveal = true;
        }

        // Check if tile is in scan area
        if (tile_x == center_x && tile_y == center_y) ||
           (tile_x == left_x && tile_y == left_y) ||
           (tile_x == right_x && tile_y == right_y) {
            should_reveal = true;
        }

        if should_reveal && !tile_state.revealed {
            tile_state.revealed = true;
            game_data.tiles_revealed_count += 1;  // Track exploration

            // Reveal tile - change to island/terrain appearance
            if matches!(resource_type, ResourceType::Empty) {
                sprite.color = Color::srgb(0.3, 0.5, 0.7); // Water for empty tiles
                sprite.custom_size = Some(Vec2::new(64.0, 64.0));
            } else {
                sprite.color = Color::srgb(0.8, 0.7, 0.5); // Sandy terrain for resources
                sprite.custom_size = Some(Vec2::new(60.0, 60.0)); // Slightly smaller to show border

                // Check if it's a critical resource
                if matches!(resource_type,
                    ResourceType::IdleGPUCluster |
                    ResourceType::OverprovisionedOpenShift |
                    ResourceType::MissedReservation
                ) {
                    info!("⚠️ Critical resource found: {}", resource_type.hint());
                }
            }

            // Add visual indicator for the resource type
            if !matches!(resource_type, ResourceType::Empty) {
                let tile_size = 64.0;
                let grid_offset_x = -(game_data.grid_size as f32 * tile_size) / 2.0 + tile_size / 2.0;
                let grid_offset_y = -(game_data.grid_size as f32 * tile_size) / 2.0 + tile_size / 2.0;
                let world_x = grid_offset_x + (tile.x as f32 * tile_size);
                let world_y = grid_offset_y + (tile.y as f32 * tile_size);

                // Load the appropriate sprite for this resource type
                let sprite_path = resource_type.get_sprite_path();
                if !sprite_path.is_empty() {
                    commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load(sprite_path.to_string()),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(30.0, 30.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(world_x, world_y, 5.0),
                            ..default()
                        },
                        TileIcon { parent: entity },
                    ));
                }
            }
        }
    }
}

pub fn process_tile_fix(
    mut tiles: Query<(&mut TileState, &ResourceType, &Tile)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    // Fix the resource - simple and always succeeds
    for (mut tile_state, resource_type, tile) in tiles.iter_mut() {
        if tile.x == game_data.player_x && tile.y == game_data.player_y {
            if tile_state.revealed && !tile_state.fixed && tile_state.fixing_sprints_left == 0 {
                tile_state.fixing_sprints_left = resource_type.fix_sprints();
                game_data.process_sprint();
            }
            break;
        }
    }
}

pub fn update_game_state(
    mut tiles: Query<(&mut TileState, &ResourceType, &mut Sprite, &Tile)>,
    mut game_data: ResMut<GameData>,
) {
    // Apply decay every 5 sprints - simple version
    if game_data.sprint > 0 && game_data.sprint - game_data.last_decay_sprint >= 5 {
        game_data.last_decay_sprint = game_data.sprint;

        // Apply 20% decay to all unfixed resources
        for (mut tile_state, resource_type, _, _) in tiles.iter_mut() {
            if !tile_state.fixed && !matches!(resource_type, ResourceType::Empty) {
                if tile_state.decay_multiplier == 0.0 {
                    tile_state.decay_multiplier = 1.0; // Initialize if not set
                }
                tile_state.decay_multiplier *= 1.2; // 20% worse
            }
        }
    }

    let mut total_waste = 0;
    let mut total_monthly_savings = 0;
    let mut tiles_to_fix = Vec::new();

    // Collect all tile positions and states for neighborhood calculations
    let mut tile_states = Vec::new();
    for (tile_state, resource_type, _, tile) in tiles.iter() {
        tile_states.push((
            tile.x as i32,
            tile.y as i32,
            tile_state.fixed,
            !matches!(resource_type, ResourceType::Empty)
        ));
    }

    // Collect current state for adjacency calculations
    let mut fixed_positions = Vec::new();
    for (tile_state, _, _, tile) in tiles.iter() {
        if tile_state.fixed {
            fixed_positions.push((tile.x, tile.y));
        }
    }

    // Process tiles
    for (mut tile_state, resource_type, mut sprite, tile) in tiles.iter_mut() {
        // Process fixing animations
        if tile_state.fixing_sprints_left > 0 {
            tile_state.fixing_sprints_left -= 1;
            if tile_state.fixing_sprints_left == 0 {
                tile_state.fixed = true;
                sprite.color = Color::srgb(0.3, 0.7, 0.4); // Green for fixed
                sprite.custom_size = Some(Vec2::new(56.0, 56.0)); // Even smaller to show it's complete
                tiles_to_fix.push((tile.x, tile.y, *resource_type));
            }
        }

        // Count all fixed tiles' savings for display
        if tile_state.fixed {
            let (min_savings, max_savings) = resource_type.savings_range();
            let savings = (min_savings + max_savings) / 2;
            total_monthly_savings += savings;
        }

        // Count ALL unfixed waste with decay and neighborhood effects
        if !tile_state.fixed && !matches!(resource_type, ResourceType::Empty) {
            let base_waste = resource_type.waste_cost();
            let decay_mult = if tile_state.decay_multiplier == 0.0 { 1.0 } else { tile_state.decay_multiplier };

            // Calculate neighborhood effect
            let mut neighbor_multiplier = 1.0;
            let tile_x = tile.x as i32;
            let tile_y = tile.y as i32;

            for (nx, ny, is_fixed, has_resource) in &tile_states {
                // Check if it's an adjacent tile (not diagonal, not self)
                let dx = (tile_x - nx).abs();
                let dy = (tile_y - ny).abs();
                if (dx == 1 && dy == 0) || (dx == 0 && dy == 1) {
                    if *has_resource {
                        if *is_fixed {
                            neighbor_multiplier *= 0.9; // Fixed neighbor reduces waste by 10%
                        } else {
                            neighbor_multiplier *= 1.1; // Unfixed neighbor increases waste by 10%
                        }
                    }
                }
            }

            let final_waste = (base_waste as f32 * decay_mult * neighbor_multiplier) as i32;
            total_waste += final_waste;
        }
    }

    // Verify the total waste matches (this recalculates from scratch)
    game_data.total_waste = total_waste;
    game_data.monthly_savings = total_monthly_savings;

    // Process tiles that were just fixed
    for (x, y, resource_type) in tiles_to_fix {
        // When fixed, reduce the total waste on the map
        game_data.total_waste -= resource_type.waste_cost();

        // Calculate combo multiplier
        if game_data.resources_fixed_in_order.len() > 0 {
            let last_fixed = game_data.resources_fixed_in_order.last().unwrap();
            // Combo if fixing same type or related resources
            if resource_type == *last_fixed || are_resources_related(&resource_type, last_fixed) {
                game_data.combo_multiplier = (game_data.combo_multiplier + 0.2).min(2.0);
            } else {
                game_data.combo_multiplier = 1.0;
            }
        }
        game_data.resources_fixed_in_order.push(resource_type);

        // Check for adjacency bonus
        let adjacent_fixed = count_adjacent_fixed_from_positions(&fixed_positions, x, y, game_data.grid_size);
        if adjacent_fixed > 0 {
            game_data.adjacency_bonus += 50 * adjacent_fixed as i32;
        }

        // Add savings with combo multiplier
        let (min_savings, max_savings) = resource_type.savings_range();
        let base_savings = (min_savings + max_savings) / 2;
        let savings = (base_savings as f32 * game_data.combo_multiplier) as i32;
        game_data.monthly_savings += savings;
    }

    // Only check win condition if game is in progress
    if !game_data.game_lost && !game_data.game_won {
        game_data.check_win_condition();
    }
}

pub fn check_win_condition(
    game_data: Res<GameData>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    if game_data.game_won || game_data.game_lost {
        next_state.set(crate::GameState::GameOver);
    }
}

pub fn animate_reveals(
    mut commands: Commands,
    mut query: Query<(Entity, &mut RevealAnimation)>,
    time: Res<Time>,
) {
    for (entity, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.finished() {
            commands.entity(entity).remove::<RevealAnimation>();
        }
    }
}

pub fn cleanup_game(
    mut commands: Commands,
    tiles_query: Query<Entity, With<Tile>>,
    player_query: Query<Entity, With<PlayerMarker>>,
    ui_query: Query<Entity, With<GameUI>>,
    icons_query: Query<Entity, With<TileIcon>>,
) {
    // Remove all tiles
    for entity in tiles_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Remove player
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Remove UI
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Remove icons
    for entity in icons_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}