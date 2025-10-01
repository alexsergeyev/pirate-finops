use bevy::prelude::*;
use bevy::math::Quat;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct TileIcon {
    pub parent: Entity,
}

#[derive(Component)]
pub struct Player {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct RotationAnimation {
    pub start_rotation: Quat,
    pub target_rotation: Quat,
    pub timer: Timer,
}

#[derive(Component, Default)]
pub struct TileState {
    pub revealed: bool,
    pub fixed: bool,
    pub fixing_sprints_left: u8,
    pub decay_multiplier: f32,  // 1.0 = no decay, 1.2 = 20% worse, etc.
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    // High Value (Critical)
    OverprovisionedOpenShift,
    MissedReservation,
    IdleGPUCluster,

    // Medium-High Value
    IdleVM,
    OversizedAppService,
    UnusedSQLDatabase,
    OverprovisionedCosmosDB,

    // Medium Value
    LogIngestionBloat,
    ExpiredBackups,
    RedundantLoadBalancer,

    // Low Value
    OrphanedDisk,
    StaleSnapshot,
    UnusedPublicIP,
    UntaggedResource,
    IdleCDNEndpoint,
    EmptyStorageAccount,

    // Special
    Empty,
}

impl ResourceType {
    pub fn waste_cost(&self) -> i32 {
        match self {
            // Critical (>$200/mo)
            Self::OverprovisionedOpenShift => 320,
            Self::MissedReservation => 280,
            Self::IdleGPUCluster => 450,

            // High ($100-200/mo)
            Self::IdleVM => 150,
            Self::OversizedAppService => 120,
            Self::UnusedSQLDatabase => 180,
            Self::OverprovisionedCosmosDB => 160,

            // Medium ($50-100/mo)
            Self::LogIngestionBloat => 80,
            Self::ExpiredBackups => 60,
            Self::RedundantLoadBalancer => 90,

            // Low (<$50/mo)
            Self::OrphanedDisk => 30,
            Self::StaleSnapshot => 25,
            Self::UnusedPublicIP => 20,
            Self::UntaggedResource => 10,
            Self::IdleCDNEndpoint => 35,
            Self::EmptyStorageAccount => 15,

            Self::Empty => 0,
        }
    }

    pub fn fix_sprints(&self) -> u8 {
        match self {
            // Complex fixes (3 sprints)
            Self::OverprovisionedOpenShift => 3,
            Self::IdleGPUCluster => 3,
            Self::OverprovisionedCosmosDB => 3,

            // Moderate fixes (2 sprints)
            Self::IdleVM => 2,
            Self::MissedReservation => 2,
            Self::OversizedAppService => 2,
            Self::UnusedSQLDatabase => 2,
            Self::LogIngestionBloat => 2,
            Self::RedundantLoadBalancer => 2,

            // Quick fixes (1 sprint)
            Self::OrphanedDisk => 1,
            Self::StaleSnapshot => 1,
            Self::UnusedPublicIP => 1,
            Self::UntaggedResource => 1,
            Self::ExpiredBackups => 1,
            Self::IdleCDNEndpoint => 1,
            Self::EmptyStorageAccount => 1,

            Self::Empty => 0,
        }
    }

    pub fn savings_range(&self) -> (i32, i32) {
        match self {
            // Excellent ROI
            Self::IdleGPUCluster => (600, 750),
            Self::OverprovisionedOpenShift => (420, 520),
            Self::MissedReservation => (380, 480),

            // Good ROI
            Self::UnusedSQLDatabase => (240, 300),
            Self::OverprovisionedCosmosDB => (210, 280),
            Self::IdleVM => (200, 250),
            Self::OversizedAppService => (160, 200),

            // Moderate ROI
            Self::RedundantLoadBalancer => (110, 140),
            Self::LogIngestionBloat => (100, 130),
            Self::ExpiredBackups => (75, 95),

            // Low ROI
            Self::IdleCDNEndpoint => (40, 50),
            Self::OrphanedDisk => (35, 45),
            Self::StaleSnapshot => (28, 35),
            Self::UnusedPublicIP => (22, 28),
            Self::EmptyStorageAccount => (16, 20),
            Self::UntaggedResource => (10, 12),

            Self::Empty => (0, 0),
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::IdleGPUCluster => "🎮",
            Self::OverprovisionedOpenShift => "🐙",
            Self::MissedReservation => "🧭",
            Self::IdleVM => "💻",
            Self::UnusedSQLDatabase => "🗄️",
            Self::OverprovisionedCosmosDB => "🌌",
            Self::OversizedAppService => "🧱",
            Self::LogIngestionBloat => "📝",
            Self::ExpiredBackups => "📦",
            Self::RedundantLoadBalancer => "⚖️",
            Self::OrphanedDisk => "💾",
            Self::StaleSnapshot => "📸",
            Self::UnusedPublicIP => "🌐",
            Self::UntaggedResource => "🏷️",
            Self::IdleCDNEndpoint => "📡",
            Self::EmptyStorageAccount => "📂",
            Self::Empty => "✨",
        }
    }

    pub fn hint(&self) -> &str {
        match self {
            // Critical
            Self::IdleGPUCluster => "GPU cluster idle - $450/mo CRITICAL!",
            Self::OverprovisionedOpenShift => "OpenShift overscaled - $320/mo waste!",
            Self::MissedReservation => "No reservation - $280/mo overpay!",

            // High
            Self::UnusedSQLDatabase => "SQL Database unused - $180/mo waste",
            Self::OverprovisionedCosmosDB => "Cosmos DB oversized - $160/mo waste",
            Self::IdleVM => "VM running idle - $150/mo waste",
            Self::OversizedAppService => "App Service oversized - $120/mo",

            // Medium
            Self::RedundantLoadBalancer => "Duplicate load balancer - $90/mo",
            Self::LogIngestionBloat => "Log ingestion high - $80/mo",
            Self::ExpiredBackups => "Old backups retained - $60/mo",

            // Low
            Self::IdleCDNEndpoint => "CDN endpoint unused - $35/mo",
            Self::OrphanedDisk => "Orphaned disk - $30/mo",
            Self::StaleSnapshot => "Stale snapshot - $25/mo",
            Self::UnusedPublicIP => "Unused public IP - $20/mo",
            Self::EmptyStorageAccount => "Empty storage - $15/mo",
            Self::UntaggedResource => "Missing tags - $10/mo",

            Self::Empty => "Clear waters",
        }
    }

    pub fn get_azure_icon(&self) -> &str {
        match self {
            Self::IdleGPUCluster => "🎮",
            Self::OverprovisionedOpenShift => "⚙️",
            Self::MissedReservation => "📆",
            Self::IdleVM => "☁️",
            Self::UnusedSQLDatabase => "🗄️",
            Self::OverprovisionedCosmosDB => "🌌",
            Self::OversizedAppService => "🏭",
            Self::LogIngestionBloat => "📈",
            Self::ExpiredBackups => "📦",
            Self::RedundantLoadBalancer => "⚖️",
            Self::OrphanedDisk => "💾",
            Self::StaleSnapshot => "📷",
            Self::UnusedPublicIP => "🌐",
            Self::UntaggedResource => "🏷️",
            Self::IdleCDNEndpoint => "📡",
            Self::EmptyStorageAccount => "📂",
            Self::Empty => "✨",
        }
    }

    pub fn get_icon_color(&self) -> bevy::prelude::Color {
        use bevy::prelude::Color;
        match self {
            Self::IdleGPUCluster => Color::srgb(1.0, 0.2, 0.2),
            Self::OverprovisionedOpenShift => Color::srgb(0.2, 0.6, 1.0),
            Self::MissedReservation => Color::srgb(1.0, 0.5, 0.2),
            Self::IdleVM => Color::srgb(0.5, 0.7, 1.0),
            Self::UnusedSQLDatabase => Color::srgb(0.8, 0.4, 0.8),
            Self::OverprovisionedCosmosDB => Color::srgb(0.6, 0.3, 0.9),
            Self::OversizedAppService => Color::srgb(0.4, 0.4, 0.7),
            Self::LogIngestionBloat => Color::srgb(0.9, 0.3, 0.3),
            Self::ExpiredBackups => Color::srgb(0.7, 0.5, 0.3),
            Self::RedundantLoadBalancer => Color::srgb(0.5, 0.8, 0.5),
            Self::OrphanedDisk => Color::srgb(0.6, 0.6, 0.9),
            Self::StaleSnapshot => Color::srgb(0.7, 0.5, 0.9),
            Self::UnusedPublicIP => Color::srgb(0.3, 0.9, 0.3),
            Self::UntaggedResource => Color::srgb(0.9, 0.9, 0.3),
            Self::IdleCDNEndpoint => Color::srgb(0.4, 0.7, 0.7),
            Self::EmptyStorageAccount => Color::srgb(0.8, 0.8, 0.5),
            Self::Empty => Color::srgb(0.6, 0.6, 0.6),
        }
    }

    pub fn get_sprite_path(&self) -> &str {
        match self {
            // Use different patterns for each resource
            Self::IdleGPUCluster => "sprites/patterns/pattern_0029.png",  // Special pattern for GPU
            Self::OverprovisionedOpenShift => "sprites/patterns/pattern_0005.png",
            Self::MissedReservation => "sprites/patterns/pattern_0013.png",

            Self::IdleVM => "sprites/patterns/pattern_0001.png",
            Self::UnusedSQLDatabase => "sprites/patterns/pattern_0019.png",
            Self::OverprovisionedCosmosDB => "sprites/patterns/pattern_0021.png",
            Self::OversizedAppService => "sprites/patterns/pattern_0017.png",

            Self::LogIngestionBloat => "sprites/patterns/pattern_0015.png",
            Self::ExpiredBackups => "sprites/patterns/pattern_0023.png",
            Self::RedundantLoadBalancer => "sprites/patterns/pattern_0025.png",

            Self::OrphanedDisk => "sprites/patterns/pattern_0003.png",
            Self::StaleSnapshot => "sprites/patterns/pattern_0011.png",
            Self::UnusedPublicIP => "sprites/patterns/pattern_0009.png",
            Self::UntaggedResource => "sprites/patterns/pattern_0007.png",
            Self::IdleCDNEndpoint => "sprites/patterns/pattern_0027.png",
            Self::EmptyStorageAccount => "sprites/patterns/pattern_0002.png",

            Self::Empty => "",
        }
    }
}

#[derive(Component)]
pub struct MenuButton {
    pub action: MenuAction,
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Play,
    Quit,
}

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct BudgetText;

#[derive(Component)]
pub struct SprintText;

#[derive(Component)]
pub struct WasteText;

#[derive(Component)]
pub struct SavingsText;

#[derive(Component)]
pub struct BurnRateText;

#[derive(Component)]
pub struct ComboText;

#[derive(Component)]
pub struct CurrentTileText;

#[derive(Component)]
pub struct ActionButton {
    pub action: GameAction,
}

#[derive(Clone, Copy, Debug)]
pub enum GameAction {
    Reveal,
    Fix,
}

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct RevealAnimation {
    pub timer: Timer,
}

#[derive(Component)]
pub struct FixAnimation {
    pub timer: Timer,
}