use bevy::prelude::*;
use rand::prelude::*;
use crate::components::ResourceType;

#[derive(Resource, Default)]
pub struct GameData {
    pub budget: i32,
    pub sprint: u32,
    pub total_waste: i32,
    pub monthly_savings: i32,
    pub grid_size: u8,
    pub difficulty: Difficulty,
    pub game_won: bool,
    pub game_lost: bool,
    pub player_x: u8,
    pub player_y: u8,
    pub combo_multiplier: f32,
    pub resources_fixed_in_order: Vec<ResourceType>,
    pub adjacency_bonus: i32,
    pub last_decay_sprint: u32,  // Track when we last applied decay
    pub hidden_waste_discovered: i32,  // Waste revealed through fixing certain resources
    pub cascade_failures: Vec<(u8, u8)>,  // Positions of cascading failures
    pub failed_fix_attempts: u32,  // Total failed attempts
    pub bonus_credits_earned: i32,  // Budget bonuses from special resources
    pub tiles_revealed_count: u32,  // Track exploration progress
    pub critical_resources_found: u32,  // Number of critical resources discovered
}

impl GameData {
    pub fn new(difficulty: Difficulty) -> Self {
        // Always use Normal difficulty settings
        let budget = 30000;  // More generous starting budget
        let grid_size = 8;

        Self {
            budget,
            sprint: 0,
            total_waste: 0,
            monthly_savings: 0,
            grid_size,
            difficulty,
            game_won: false,
            game_lost: false,
            player_x: grid_size / 2,
            player_y: grid_size / 2,
            combo_multiplier: 1.0,
            resources_fixed_in_order: Vec::new(),
            adjacency_bonus: 0,
            last_decay_sprint: 0,
            hidden_waste_discovered: 0,
            cascade_failures: Vec::new(),
            failed_fix_attempts: 0,
            bonus_credits_earned: 0,
            tiles_revealed_count: 0,
            critical_resources_found: 0,
        }
    }

    pub fn process_sprint(&mut self) {
        self.sprint += 1;
        let burn_rate = self.calculate_burn_rate();
        self.budget -= burn_rate;

        if self.budget <= 0 {
            self.game_lost = true;
        }
    }

    pub fn calculate_burn_rate(&self) -> i32 {
        let base_burn = 30;

        // Simple linear waste burn
        let waste_burn = (self.total_waste as f32 * 0.4) as i32;

        base_burn + waste_burn
    }

    pub fn waste_percentage(&self) -> f32 {
        if self.calculate_burn_rate() == 0 {
            return 0.0;
        }
        (self.total_waste as f32 / self.calculate_burn_rate() as f32) * 100.0
    }

    pub fn check_win_condition(&mut self) {
        // Only check win condition after at least 5 sprints
        if self.sprint < 5 {
            return;
        }

        // Simple win condition: reduce waste significantly
        if self.waste_percentage() <= 30.0 && self.monthly_savings >= 1500 {
            self.game_won = true;
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Difficulty {
    #[default]
    Normal,
}

#[derive(Resource, Default)]
pub struct SelectedTile {
    pub entity: Option<Entity>,
    pub position: Option<(u8, u8)>,
}

pub fn generate_random_grid(size: u8) -> Vec<Vec<ResourceType>> {
    let mut rng = thread_rng();
    let mut grid = vec![vec![ResourceType::Empty; size as usize]; size as usize];

    let resource_types = vec![
        // High value (more rare)
        ResourceType::IdleGPUCluster,
        ResourceType::OverprovisionedOpenShift,
        ResourceType::MissedReservation,

        // Medium-high value (common)
        ResourceType::IdleVM,
        ResourceType::IdleVM,  // Duplicate for higher frequency
        ResourceType::OversizedAppService,
        ResourceType::UnusedSQLDatabase,
        ResourceType::OverprovisionedCosmosDB,

        // Medium value (common)
        ResourceType::LogIngestionBloat,
        ResourceType::LogIngestionBloat,  // Duplicate for higher frequency
        ResourceType::ExpiredBackups,
        ResourceType::RedundantLoadBalancer,

        // Low value (very common)
        ResourceType::OrphanedDisk,
        ResourceType::OrphanedDisk,  // Duplicate
        ResourceType::StaleSnapshot,
        ResourceType::StaleSnapshot,  // Duplicate
        ResourceType::UnusedPublicIP,
        ResourceType::UntaggedResource,
        ResourceType::UntaggedResource,  // Duplicate
        ResourceType::IdleCDNEndpoint,
        ResourceType::EmptyStorageAccount,
    ];

    let total_tiles = (size * size) as usize;
    let resource_count = (total_tiles as f32 * 0.40) as usize;  // 40% of tiles have resources

    let mut positions: Vec<(usize, usize)> = Vec::new();
    for x in 0..size as usize {
        for y in 0..size as usize {
            positions.push((x, y));
        }
    }
    positions.shuffle(&mut rng);

    for i in 0..resource_count {
        let (x, y) = positions[i];
        let resource = resource_types.choose(&mut rng).unwrap().clone();
        grid[x][y] = resource;
    }

    grid
}